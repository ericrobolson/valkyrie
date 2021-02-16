use data_structures::queue::Queue;
use window::WindowControl;

mod server_dummy_wingfx;
pub mod windowing;

pub use ecs::{define_world, WorldImplementation, WorldType};

// TODO: organize this

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
    /// A single 'tick' for a game. You can assume this is about 60hz.
    fn tick(
        &mut self,
        world: &impl WorldImplementation,
        messages: &[EngineMessage],
    ) -> ControlMessage;

    fn render_world(world: &World, renderer: &mut dyn renderer::Renderer);
}

pub struct ClientConfig {
    pub min_window_w: u32,
    pub min_window_h: u32,
    pub title: &'static str,
}

pub struct ServerConfig {}

pub enum GameConfig {
    Client(ClientConfig),
    Server(ServerConfig),
    ClientServer {
        client_config: ClientConfig,
        server_config: ServerConfig,
    },
}

struct ClientState<MainFunc> {
    window: Box<dyn windowing::Window<MainFunc>>,
}

struct ServerState<MainFunc> {
    window: Box<dyn windowing::Window<MainFunc>>,
}

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

    // This next section is a little hacky, but it basically allows one to create a client, server, or client + server to run the game.
    // Client means you're connecting to a remote server
    // Server means you're allowing remote clients to join
    // ClientServer means you're allowing a single player mode, or the option to do both.
    let mut client_state: Option<ClientState<_>> = None;
    let mut server_state: Option<ServerState<_>> = None;
    {
        let (client_config, server_config) = match config {
            GameConfig::Client(client) => (Some(client), None),
            GameConfig::Server(server) => (None, Some(server)),
            GameConfig::ClientServer {
                client_config,
                server_config,
            } => (Some(client_config), Some(server_config)),
        };

        match client_config {
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
        match server_config {
            Some(config) => {
                // Build up the server state
                server_state = Some(ServerState {
                    window: server_dummy_wingfx::DummyWindow::new(),
                });
            }
            None => {}
        };
    }

    if client_state.is_some() && server_state.is_some() {
        todo!("How to run both client + server state in a single player setting? Can this be done in a single thread?");
    }

    // Create the game engine
    let mut game = Game::default();
    let mut world = World::new(WorldType::Client);
    let mut engine_queue = Queue::new(max_engine_msgs);

    let mut local_server_game: Game = Game::default();
    let mut local_server_world = World::new(WorldType::Server);

    // Timing used for ticking the game simulation
    let tick_duration = timing::hz_to_duration(sim_hz);
    let mut accumulated_time = timing::Duration::from_secs(0);
    let mut simulation_stopwatch = timing::Stopwatch::new();

    let mut has_local_server = client_state.is_some() && server_state.is_some();

    // Create the main loop
    let main_loop = move |input: Option<windowing::WindowInput>,
                          renderer: &mut dyn renderer::Renderer| {
        let mut window_control = WindowControl::Ok;

        // Queue up any messages
        if let Some(input) = input {
            engine_queue.push(EngineMessage::Input(input));
        }

        // Increase accumulated time + tick if necessary
        // Based on https://gafferongames.com/post/fix_your_timestep/ to divorce rendering + simulations
        {
            accumulated_time += simulation_stopwatch.elapsed();
            let mut updated_state = false;

            // In the event that the loop gets in a spiral of death where the sim can't keep up,
            // clamp it to a set number of ticks per frame to prevent spiraling downward.
            const MAX_TICKS_PER_FRAME: u8 = 10;
            let mut times_ticked = 0;

            // Tick the simulation until it has caught up
            while accumulated_time > tick_duration {
                accumulated_time -= tick_duration;
                times_ticked += 1;

                // tick the game
                match game.tick(&mut world, &engine_queue.items()) {
                    ControlMessage::Ok => {
                        engine_queue.clear();
                        updated_state = true;
                        world.dispatch();
                    }
                    ControlMessage::Shutdown => {
                        window_control = WindowControl::Shutdown;
                    }
                    ControlMessage::RequestServerChange => {
                        // If requesting local, reset local server state

                        // If requesting remote, connect to remote

                        todo!("Request changing the server to either a remote or local.");
                    }
                }

                // TODO: I don't like this. Instead, what about abstracting it so there's only one actual world?
                if has_local_server {
                    local_server_game.tick(&mut local_server_world, &[]);
                    local_server_world.dispatch();
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
                if world.world_type() == WorldType::Client {
                    Game::render_world(&world, renderer);
                }
            }
        }

        // Return the window control
        window_control
    };

    // Kick it all off.
    match client_state {
        Some(mut client_state) => {
            // possibly client + server stuff
            client_state.window.execute(main_loop);
        }
        None => {
            if let Some(mut server_state) = server_state {
                server_state.window.execute(main_loop);
            }
        }
    }

    Ok(())
}
