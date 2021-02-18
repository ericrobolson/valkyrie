use valkyrie_core::{application::*, ecs::*};

#[derive(Debug)]
pub struct Game {
    tick: usize,
    world: World,
    entity: Entity,
}

impl Simulation<ClientConfig> for Game {
    fn new(config: ClientConfig) -> Self {
        let mut world = World::new();

        world.register::<bool>(100);

        let entity = world.add_entity();
        let entity = world.add_entity();
        let entity = world.add_entity();
        let entity = world.add_entity();

        match world.add::<bool>(entity) {
            Ok(component) => {
                *component = true;
            }
            Err(e) => {}
        }

        Self {
            world,
            tick: 0,
            entity,
        }
    }

    fn tick(&mut self, messages: &[EngineMessage]) -> ControlMessage {
        self.tick += 1;

        ControlMessage::RequestRenderStateUpdate
    }
}

impl Renderable for Game {
    fn render(&self, renderer: &mut dyn Renderer) {
        println!("RENDERING: {:?} - tick", self.tick);
        println!("Entity: {:?}", self.entity);
        println!("Component: {:?}", self.world.get::<bool>(self.entity));
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
