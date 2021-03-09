use core_simulation::*;

pub use core_simulation::{ControlMessage, Input, Simulation, ValkErr};

pub fn run_server<Sim, Cfg, Msg>(cfg: Cfg) -> Result<(), ValkErr>
where
    Sim: Simulation<Cfg, Msg>,
{
    let mut executor = SimulationExecutor::<Sim, Cfg, Msg>::new(100, Some(60), true, cfg);

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
