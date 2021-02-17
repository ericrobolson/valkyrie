use valkyrie_core::{application::*, ecs::*};

define_world! {
    id: World,
    components: [],
    systems: []
}

pub struct Game {
    tick: usize,
}

impl Simulation<ClientConfig> for Game {
    fn new(config: ClientConfig) -> Self {
        Self { tick: 0 }
    }

    fn tick(&mut self, messages: &[EngineMessage]) -> ControlMessage {
        self.tick += 1;

        ControlMessage::RequestRenderStateUpdate
    }
}

impl Renderable for Game {
    fn render(&self, renderer: &mut dyn Renderer) {
        println!("RENDERING: {:?} - tick", self.tick);
    }
}

fn main() -> Result<(), String> {
    match run_client::<Game>(ClientConfig {
        sim_hz: Some(60),
        min_window_w: 1920,
        min_window_h: 1080,
        title: "GORE KILL",
    }) {
        Ok(result) => Ok(result),
        Err(e) => panic!("{:?}", e),
    }
}
