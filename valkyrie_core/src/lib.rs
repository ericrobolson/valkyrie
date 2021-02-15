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
    pub min_window_w: u32,
    pub min_window_h: u32,
    pub title: &'static str,
}

/// Creates and runs the game.
pub fn run<Game>(config: GameConfig) -> Result<(), ValkErr>
where
    Game: GameImplementation + 'static,
{
    let max_engine_msgs = 500;

    // Create the game
    let mut game = Game::default();
    let mut game_world = GameWorld::new();
    let mut engine_queue = Queue::new(max_engine_msgs);

    // Create the main loop
    let main_loop = move |input: Option<windowing::WindowInput>,
                          renderer: &mut dyn renderer::Renderer| {
        let mut window_control = WindowControl::Ok;

        // Queue up any messages
        if let Some(input) = input {
            engine_queue.push(EngineMessage::Input(input));
        }

        //TODO: fix your timestep by games on gaffer
        // Tick the game

        match game.tick(&mut game_world, &engine_queue.items()) {
            ControlMessage::Ok => {
                engine_queue.clear();
                game_ecs::garbage_collect(&mut game_world);
                rendering::render_world(&game_world, renderer);
            }
            ControlMessage::Shutdown => window_control = WindowControl::Shutdown,
        }

        // Return
        window_control
    };

    // Create the window
    let mut window = windowing::WinGfxBuilder::new(config.title, windowing::BackendType::Opengl)
        .with_min_size(config.min_window_w, config.min_window_h)
        .build()
        .unwrap();

    // Kick it all off.
    window.execute(main_loop);

    Ok(())
}
