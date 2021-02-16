mod server_dummy_wingfx;
pub mod windowing;
pub use renderer::Renderer;
pub mod ecs {
    pub use core_ecs::*;
}

use data_structures::queue::Queue;
use ecs::*;
use std::marker::PhantomData;
use timing::{Duration, Stopwatch};
use window::{InteractableSimulation, WindowControl};

// TODO: organize this

/// Creates and runs the game.
pub fn run<Game, World>(sim_hz: u32, config: GameConfig) -> Result<(), ValkErr>
where
    World: WorldImplementation + 'static,
    Game: GameImplementation<World> + 'static,
{
    // Config setup
    let max_engine_msgs = 500;

    let sim_hz = {
        if sim_hz == 0 {
            1
        } else {
            sim_hz
        }
    };

    let (client_config, server_config) = match config {
        GameConfig::Client(client) => (Some(client), None),
        GameConfig::Server(server) => (None, Some(server)),
        GameConfig::ClientServer {
            client_config,
            server_config,
        } => (Some(client_config), Some(server_config)),
    };

    // Build up the state
    let mut client_state: Option<ClientState<GameManager<World, Game>>> = None;
    let mut server_state: Option<ServerState<GameManager<World, Game>>> = None;
    {
        match client_config.clone() {
            Some(config) => {
                // Build up the client state
                // Create the window
                let window =
                    windowing::WinGfxBuilder::new(config.title, windowing::BackendType::Opengl)
                        .with_min_size(config.min_window_w, config.min_window_h)
                        .build()
                        .unwrap();

                client_state = Some(ClientState { window });
            }
            None => {}
        }
        match server_config.clone() {
            Some(config) => {
                // Build up the server state
                server_state = Some(ServerState {
                    window: server_dummy_wingfx::DummyWindow::new(),
                });
            }
            None => {}
        };
    }

    // Create the game simulation
    let sim = GameManager::new(max_engine_msgs, sim_hz, client_config, server_config);

    // Kick it all off.
    match client_state {
        Some(mut client_state) => {
            // possibly client + server stuff
            client_state.window.execute(sim);
        }
        None => {
            if let Some(mut server_state) = server_state {
                server_state.window.execute(sim);
            }
        }
    }

    Ok(())
}

pub enum Input {
    WindowInput(windowing::WindowInput),
}

#[derive(Debug)]
pub enum ValkErr {}

pub enum ControlMessage {
    Ok,
    Shutdown,
    RequestServerChange,
}

pub enum EngineMessage {
    Input(windowing::WindowInput),
}

pub trait GameImplementation<World>: Default {
    /// A single 'tick' for a game. The world will be dispatched at the end of the execution.
    fn tick(world: &mut World, messages: &[EngineMessage]) -> ControlMessage;

    fn render_world(world: &World, renderer: &mut dyn Renderer);
}

#[derive(Clone)]
pub struct ClientConfig {
    pub min_window_w: u32,
    pub min_window_h: u32,
    pub title: &'static str,
}

#[derive(Clone)]
pub struct ServerConfig {}

pub enum GameConfig {
    Client(ClientConfig),
    Server(ServerConfig),
    ClientServer {
        client_config: ClientConfig,
        server_config: ServerConfig,
    },
}

struct ClientState<Game> {
    window: Box<dyn windowing::Window<Game>>,
}

struct ServerState<Game> {
    window: Box<dyn windowing::Window<Game>>,
}

/// Struct for managing the client/server state.
struct GameManager<World, Game>
where
    World: WorldImplementation,
    Game: GameImplementation<World>,
{
    client_world: Option<World>,
    client_engine_queue: Queue<EngineMessage>,

    tick_duration: Duration,
    accumulated_time: Duration,
    simulation_stopwatch: Stopwatch,

    server_world: Option<World>,
    phantom_data: PhantomData<Game>,
}

impl<World, Game> InteractableSimulation for GameManager<World, Game>
where
    World: WorldImplementation,
    Game: GameImplementation<World>,
{
    fn tick(
        &mut self,
        input: Option<window::WindowInput>,
        renderer: &mut dyn renderer::Renderer,
    ) -> WindowControl {
        let mut window_control = WindowControl::Ok;

        // Queue up any messages
        if let Some(input) = input {
            self.client_engine_queue.push(EngineMessage::Input(input));
        }

        // Increase accumulated time + tick if necessary
        // Based on https://gafferongames.com/post/fix_your_timestep/ to divorce rendering + simulations
        {
            self.accumulated_time += self.simulation_stopwatch.elapsed();
            let mut updated_state = false;

            // In the event that the loop gets in a spiral of death where the sim can't keep up,
            // clamp it to a set number of ticks per frame to prevent spiraling downward.
            const MAX_TICKS_PER_FRAME: u8 = 10;
            let mut times_ticked = 0;

            // Tick the simulation until it has caught up
            while self.accumulated_time > self.tick_duration {
                self.accumulated_time -= self.tick_duration;
                times_ticked += 1;

                // Tick the client if it exists
                if let Some(ref mut world) = self.client_world {
                    match Game::tick(world, &self.client_engine_queue.items()) {
                        ControlMessage::Ok => {
                            self.client_engine_queue.clear();
                            updated_state = true;
                            world.dispatch();
                        }
                        ControlMessage::Shutdown => {
                            window_control = WindowControl::Shutdown;
                            world.dispatch();
                        }
                        ControlMessage::RequestServerChange => {
                            // If requesting local, reset local server state

                            // If requesting remote, connect to remote

                            todo!("Request changing the server to either a remote or local.");
                        }
                    }
                }

                // Tick the server if it exists
                if let Some(ref mut world) = self.server_world {
                    match Game::tick(world, &[]) {
                        ControlMessage::Ok => {
                            self.client_engine_queue.clear();
                            updated_state = true;
                            world.dispatch();
                        }
                        ControlMessage::Shutdown => {
                            window_control = WindowControl::Shutdown;
                            world.dispatch();
                        }
                        ControlMessage::RequestServerChange => {
                            // Server shouldn't change here.
                        }
                    }
                }

                // Break out if the sim is taking too long.
                if times_ticked >= MAX_TICKS_PER_FRAME {
                    // This way it keeps processing and doesn't get stuck in a horrendous loop. It'll slow the game down
                    // to a crawl, but at least it isn't preventing people from playing.
                    break;
                }

                if window_control == WindowControl::Shutdown {
                    todo!("How to shut down? How to deal with servers and/or clients?");
                    break;
                }
            }

            // If there's a new state and it's not shutting down render the latest version of the world.
            if updated_state && window_control != WindowControl::Shutdown {
                if let Some(world) = &self.client_world {
                    if world.world_type() == WorldType::Client {
                        Game::render_world(&world, renderer);
                    }
                }
            }
        }

        // Return the window control
        window_control
    }
}

impl<World, Game> GameManager<World, Game>
where
    World: WorldImplementation,
    Game: GameImplementation<World>,
{
    fn new(
        max_engine_msgs: usize,
        sim_hz: u32,
        client_config: Option<ClientConfig>,
        server_config: Option<ServerConfig>,
    ) -> Self {
        let tick_duration = timing::hz_to_duration(sim_hz);
        let accumulated_time = timing::Duration::from_secs(0);
        let simulation_stopwatch = timing::Stopwatch::new();

        let client_world = match client_config {
            Some(config) => Some(World::new(WorldType::Client)),
            None => None,
        };

        let server_world = match server_config {
            Some(config) => Some(World::new(WorldType::Server)),
            None => None,
        };

        Self {
            client_world,
            server_world,

            client_engine_queue: Queue::new(max_engine_msgs),
            tick_duration,
            accumulated_time,
            simulation_stopwatch,
            phantom_data: PhantomData,
        }
    }
}
