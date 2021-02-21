use crate::{
    core_sim_managers::{Config, RenderableSimManager, SimManager},
    window, windowing,
};

const MAX_ENGINE_MSGS: usize = 1000;

// Reexport things to make it easier for users.

pub use crate::{
    renderer::Renderer,
    window::{Renderable, WindowControl, WindowInput},
};

/// Messages provided by the engine to the application.
pub enum EngineMessage {
    Input(WindowInput),
}

/// Messages the application may send to the engine.
pub enum ControlMessage {
    Ok,
    Shutdown,
    RequestRedraw,
    RequestRenderStateUpdate,
    RequestServerChange,
}

/// Common functionality a simulation must implement.
pub trait Simulation<Cfg> {
    /// Creates a new simulation.
    fn new(config: Cfg) -> Self;

    /// A single 'tick' for an application.
    fn tick(&mut self, messages: &[EngineMessage]) -> ControlMessage;
}

#[derive(Debug)]
pub enum ValkErr {}

#[derive(Clone, Copy)]
pub struct ClientConfig {
    /// The hz to run the simulation at. Not providing a simulation hz will only execute when some form of input is provided.
    pub sim_hz: Option<u32>,
    pub min_window_w: u32,
    pub min_window_h: u32,
    pub title: &'static str,
}

impl Config for ClientConfig {
    fn sim_hz(&self) -> Option<u32> {
        self.sim_hz
    }
}

/// Configuration required for a server instance
#[derive(Clone, Copy)]
pub struct ServerConfig {
    /// The hz to run the simulation at. Not providing a simulation hz will only execute when some form of input is provided.
    pub sim_hz: Option<u32>,
    /// Whether the server is a locally ran one or a remote server
    pub is_local_server: bool,
}

impl Config for ServerConfig {
    fn sim_hz(&self) -> Option<u32> {
        self.sim_hz
    }
}

/// Runs a headless server.
pub fn run_server<Sim>(config: ServerConfig) -> Result<(), ValkErr>
where
    Sim: Simulation<ServerConfig>,
{
    use window::Simulation;

    let mut server: SimManager<Sim, ServerConfig> = SimManager::new(MAX_ENGINE_MSGS, config);

    loop {
        match server.tick(None) {
            WindowControl::Ok => {}
            WindowControl::Render => {}
            WindowControl::UpdateRenderState => {}
            WindowControl::Shutdown => {
                break;
            }
        }
    }

    Ok(())
}

/// Creates and runs the game.
pub fn run_client<Client>(config: ClientConfig) -> Result<(), ValkErr>
where
    Client: Simulation<ClientConfig> + window::Renderable + 'static,
{
    let mut window = windowing::WinGfxBuilder::new(config.title, windowing::BackendType::Opengl)
        .with_min_size(config.min_window_w, config.min_window_h)
        .build()
        .unwrap();

    // Build up the client
    let client: RenderableSimManager<Client, ClientConfig> =
        RenderableSimManager::new(MAX_ENGINE_MSGS, config);

    // Start executing in the window
    window.execute(client);

    Ok(())
}
