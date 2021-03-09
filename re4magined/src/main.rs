// RE4magined

use valkyrie_core::application::*;

struct Re4magined {}

struct Cfg {}

enum Msg {}

impl Simulation<Cfg, Msg> for Re4magined {
    fn new(config: Cfg) -> Self {
        Self {}
    }

    fn tick(&mut self, delta_t: std::time::Duration, messages: &[Input<Msg>]) -> ControlMessage {
        println!("Hello, world!");

        ControlMessage::ExitSim
    }
}

fn main() {
    run_server::<Re4magined, Cfg, Msg>(Cfg {}).unwrap();
}
