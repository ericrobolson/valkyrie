use valkyrie_core::{application::*, ecs::*};

#[derive(Debug)]
pub struct Game {
    tick: usize,
    world: World,
    entity: Entity,
}

#[derive(Default, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {}

#[derive(Default, Clone, Debug)]
struct Velocity {
    x: i32,
    y: i32,
    active: bool,
}

impl Component for Velocity {}

#[derive(Default, Clone, Debug)]
struct Alive {
    value: bool,
}

impl Component for Alive {}

impl Simulation<ClientConfig> for Game {
    fn new(config: ClientConfig) -> Self {
        let mut world = World::new();

        world.register::<Alive>(1);
        world.register::<Position>(100);
        world.register::<Velocity>(100);

        let entity = world.add_entity();
        let entity = world.add_entity();
        let entity = world.add_entity();
        let entity = world.add_entity();

        match world.add::<Alive>(entity) {
            Ok(component) => {
                component.value = true;
            }
            Err(e) => {}
        }

        match world.add::<Position>(entity) {
            Ok(position) => {
                position.x = 1;
                position.y = -1;
            }
            _ => {}
        }

        match world.add::<Velocity>(entity) {
            Ok(velocity) => {
                velocity.x = 11;
                velocity.y = -11;
            }
            _ => {}
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
        println!("Component: {:?}", self.world.get::<Alive>(self.entity));
        println!("Component: {:?}", self.world.get::<Position>(self.entity));
        println!("Component: {:?}", self.world.get::<Velocity>(self.entity));

        for entity in self.world.entities() {
            println!("Alive entity: {:?}", entity);
        }
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
