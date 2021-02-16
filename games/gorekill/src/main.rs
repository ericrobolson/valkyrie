use valkyrie_core::{
    ClientConfig, ControlMessage, EngineMessage, GameConfig, GameImplementation, World,
};

pub struct Game {
    tick: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self { tick: 0 }
    }
}

impl GameImplementation for Game {
    fn tick(&mut self, world: &mut World, messages: &[EngineMessage]) -> ControlMessage {
        self.tick += 1;
        println!("A tick! {:?}", self.tick);

        ControlMessage::Ok
    }
}

fn main() -> Result<(), String> {
    match valkyrie_core::run::<Game>(
        60,
        GameConfig::Client(ClientConfig {
            title: "GORE KILL",
            min_window_w: 1920,
            min_window_h: 1080,
        }),
    ) {
        Ok(result) => Ok(result),
        Err(e) => panic!("{:?}", e),
    }
}
