use valkyrie_core::simulation::*;

pub enum Message {}

pub struct GoreKillConfig {}

pub struct GoreKillSim {
    tick: usize,
}

impl Simulation<GoreKillConfig, Message> for GoreKillSim {
    fn new(config: GoreKillConfig) -> Self {
        Self { tick: 0 }
    }

    fn tick(&mut self, delta_t: Duration, messages: &[Message]) -> ControlMessage {
        self.tick += 1;
        if self.tick % 2 == 0 {
            return ControlMessage::ExitSim;
        }

        ControlMessage::Ok
    }
}
