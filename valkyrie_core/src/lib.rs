pub mod windowing;
pub use renderer::Renderer;
pub mod ecs {
    pub use core_ecs::*;
}

mod sim_managers;
use sim_managers::{Config, RenderableSimManager, SimManager};

pub use window::Renderable;

use data_structures::queue::Queue;
use window::WindowControl;

// TODO: organize this

pub trait Simulation<Cfg> {
    fn new(config: Cfg) -> Self;

    /// A single 'tick' for an application.
    fn tick(&mut self, messages: &[EngineMessage]) -> ControlMessage;
}

const MAX_ENGINE_MSGS: usize = 1000;

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
    use window::{Renderable, Simulation};

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

pub enum Input {
    WindowInput(windowing::WindowInput),
}

#[derive(Debug)]
pub enum ValkErr {}

pub enum ControlMessage {
    Ok,
    Shutdown,
    RequestRedraw,
    RequestRenderStateUpdate,
    RequestServerChange,
}

pub enum EngineMessage {
    Input(windowing::WindowInput),
}

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

#[derive(Clone, Copy)]
pub struct ServerConfig {
    /// The hz to run the simulation at. Not providing a simulation hz will only execute when some form of input is provided.
    pub sim_hz: Option<u32>,
    pub is_local_server: bool,
}

impl Config for ServerConfig {
    fn sim_hz(&self) -> Option<u32> {
        self.sim_hz
    }
}

pub struct GameConfig {
    /// The client configuration
    pub client_config: Option<ClientConfig>,
    /// The server configuration
    pub server_config: Option<ServerConfig>,
}
