use gdnative::prelude::*;

use valkyrie_core::simulation::*;

mod sim;

// Utilized https://godot-rust.github.io/book/introduction.html heavily
// This class is basically the link between Godot + the rust libs

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GoreKill {
    sim: SimulationExecutor<sim::GoreKillSim, sim::GoreKillConfig, sim::Message>,
}

#[methods]
impl GoreKill {
    fn new(_owner: &Node2D) -> Self {
        let max_engine_msgs = 100;
        let sim_hz = Some(60);
        let fixed_timestep = true;
        let config = sim::GoreKillConfig {};

        Self {
            sim: SimulationExecutor::new(max_engine_msgs, sim_hz, fixed_timestep, config),
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node2D) {}

    #[export]
    fn _process(&mut self, owner: &Node2D, delta: f32) {
        match self.sim.tick(None) {
            ControlMessage::Ok => {
                godot_print!("Its ok!");
            }
            ControlMessage::ExitSim => {
                godot_print!("Its exit!");
            }
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<GoreKill>();
}

godot_init!(init);
