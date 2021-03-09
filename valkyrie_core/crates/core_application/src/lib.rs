use core_simulation::*;
mod window_builder;
use core_window::Window;

const MAX_ENGINE_MSGS: usize = 256;

pub use core_simulation::{ControlMessage, Input, Simulation, ValkErr};
pub use core_window::Renderable;
pub use window_builder::BackendType;

pub struct SimulationOptions<Cfg> {
    /// Whether a fixed timestep should be used
    pub use_fixed_timestep: bool,
    /// The amount of hz the simulation should run at
    pub sim_hz: Option<u32>,
    /// The configuration for the sim
    pub cfg: Cfg,
}

pub fn run_server<Sim, Cfg, Msg>(options: SimulationOptions<Cfg>) -> Result<(), ValkErr>
where
    Sim: Simulation<Cfg, Msg>,
{
    let mut executor = SimulationExecutor::<Sim, Cfg, Msg>::new(
        MAX_ENGINE_MSGS,
        options.sim_hz,
        options.use_fixed_timestep,
        options.cfg,
    );

    // TODO: networking + io?
    loop {
        match executor.tick(None) {
            ControlMessage::Ok => {}
            ControlMessage::ExitSim => {
                break;
            }
        }
    }

    Ok(())
}

pub fn run_client<Sim, Cfg, Msg>(
    title: &'static str,
    min_width: u32,
    min_height: u32,
    options: SimulationOptions<Cfg>,
    backend: BackendType,
) -> Result<(), ValkErr>
where
    Sim: Simulation<Cfg, Msg> + Renderable + 'static,
    Cfg: 'static,
    Msg: 'static,
{
    let mut executor = SimulationExecutor::<Sim, Cfg, Msg>::new(
        MAX_ENGINE_MSGS,
        options.sim_hz,
        options.use_fixed_timestep,
        options.cfg,
    );

    let mut window: Box<dyn Window<Sim, Cfg, Msg>> = window_builder::WinGfxBuilder {
        title: title,
        size: window_builder::Size {
            w: min_width,
            h: min_height,
        },
    }
    .build(backend)
    .unwrap();

    window.execute(executor);

    Ok(())
}
