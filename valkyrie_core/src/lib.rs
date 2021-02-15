use data_structures::queue::Queue;
use window::WindowControl;

mod game_ecs;
mod rendering;
pub mod windowing;
pub use game_ecs::GameWorld;

// TODO: organize this

pub enum Input {
    WindowInput(windowing::WindowInput),
}

#[derive(Debug)]
pub enum ValkErr {}

pub enum ControlMessage {
    Ok,
    Shutdown,
}

pub enum EngineMessage {
    Input(windowing::WindowInput),
}

pub trait GameImplementation: Default {
    /// A single 'tick' for a game. You can assume this is about 60hz.
    fn tick(&mut self, world: &mut GameWorld, messages: &[EngineMessage]) -> ControlMessage;
}

pub struct GameConfig {
    pub sim_hz: u32,
    pub min_window_w: u32,
    pub min_window_h: u32,
    pub title: &'static str,
}

/// Creates and runs the game.
pub fn run<Game>(config: GameConfig) -> Result<(), ValkErr>
where
    Game: GameImplementation + 'static,
{
    // Config setup
    let max_engine_msgs = 500;

    let sim_hz = {
        if config.sim_hz == 0 {
            1
        } else {
            config.sim_hz
        }
    };

    // Create the window
    let mut window = windowing::WinGfxBuilder::new(config.title, windowing::BackendType::Opengl)
        .with_min_size(config.min_window_w, config.min_window_h)
        .build()
        .unwrap();

    // Create the game engine
    let mut game = Game::default();
    let mut game_world = GameWorld::new();
    let mut engine_queue = Queue::new(max_engine_msgs);

    // Timing used for ticking the game simulation
    let tick_duration = timing::hz_to_duration(sim_hz);
    let mut accumulated_time = timing::Duration::from_secs(0);
    let mut simulation_stopwatch = timing::Stopwatch::new();

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
                match game.tick(&mut game_world, &engine_queue.items()) {
                    ControlMessage::Ok => {
                        engine_queue.clear();
                        updated_state = true;
                        game_ecs::garbage_collect(&mut game_world);
                    }
                    ControlMessage::Shutdown => {
                        window_control = WindowControl::Shutdown;
                        break;
                    }
                }

                // Break out if the sim is taking too long.
                if times_ticked >= MAX_TICKS_PER_FRAME {
                    // This way it keeps processing and doesn't get stuck in a horrendous loop. It'll slow the game down
                    // to a crawl, but at least it isn't preventing people from playing.
                    break;
                }
            }

            // If there's a new state and it's not shutting down render the latest version of the world.
            if updated_state && window_control != WindowControl::Shutdown {
                rendering::render_world(&game_world, renderer);
            }
        }

        // Return the window control
        window_control
    };

    // Kick it all off.
    window.execute(main_loop);

    Ok(())
}
