use valkyrie_core::{
    ecs::*, ClientConfig, ControlMessage, EngineMessage, GameConfig, GameImplementation, Renderer,
};

define_world! {
    id: World,
    components: [],
    systems: []
}

pub struct Game {
    tick: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self { tick: 0 }
    }
}

impl GameImplementation<World> for Game {
    fn tick(world: &mut World, messages: &[EngineMessage]) -> ControlMessage {
        println!("A tick! {:?}", 0);

        ControlMessage::Ok
    }

    fn render_world(world: &World, renderer: &mut dyn Renderer) {}
}

fn main() -> Result<(), String> {
    match valkyrie_core::run::<Game, World>(
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
