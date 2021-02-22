use euclid::vec2;
use gdnative::prelude::*;

use valkyrie_core::simulation::*;

mod sim;

// Utilized https://godot-rust.github.io/book/introduction.html heavily
// This class is basically the link between Godot + the rust libs

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GoreKill {
    sim: SimulationExecutor<sim::GoreKillSim, sim::GoreKillConfig, sim::Message>,
    debug_nodes: Vec<Ref<gdnative::prelude::Node2D>>,
}

#[methods]
impl GoreKill {
    fn new(owner: &Node2D) -> Self {
        let max_engine_msgs = 100;
        let sim_hz = Some(60);
        let fixed_timestep = true;
        let config = sim::GoreKillConfig::normal_mode();
        let debug_shape_count = 1000;

        let mut debug_nodes = vec![];

        // Add debug scenes
        // (https://github.com/godot-rust/godot-rust/issues/582)
        {
            let debug_scene = match load_scene("res://nodes/DebugSphere.tscn") {
                Some(scene) => scene,
                None => {
                    godot_print!("Unable to load scene. Check name.");
                    panic!();
                }
            };

            // Add them to the struct, hiding them first.
            for _ in 0..debug_shape_count {
                match instance_scene::<Node2D>(&debug_scene) {
                    Ok(instance) => {
                        instance.hide();

                        let shared = instance.into_shared();
                        debug_nodes.push(shared);
                        owner.add_child(shared, false);
                    }
                    Err(e) => {
                        godot_print!("{:?} making scene.", e);
                        panic!();
                    }
                }
            }
        }

        Self {
            debug_nodes,
            sim: SimulationExecutor::new(max_engine_msgs, sim_hz, fixed_timestep, config),
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node2D) {}

    #[export]
    fn _process(&mut self, owner: &Node2D, delta: f32) {
        match self.sim.tick(None) {
            ControlMessage::Ok => {
                if self.sim.sim_mut().dirty {
                    self.render();
                }
                self.sim.sim_mut().dirty = false;

                // TODO: interpolation?
            }
            ControlMessage::ExitSim => {
                godot_print!("Its exit!");
            }
        }
    }
}

impl GoreKill {
    fn render(&mut self) {
        use sim::components;

        // Update render sim
        let sim = self.sim.sim();
        let mut debug_shape_index = 0;

        for entity in sim.world.entities() {
            let entity = *entity;
            let debug = sim.world.get::<components::Debug>(entity);
            let position = sim.world.get::<components::Position>(entity);
            let collision_shape = sim.world.get::<components::Collidable>(entity);

            // Draw debug shapes
            if let Some(debug) = debug {
                if let Some(position) = position {
                    if let Some(collision_shape) = collision_shape {
                        // Only update it if it's valid
                        if debug_shape_index < self.debug_nodes.len() {
                            unsafe {
                                let instance = self.debug_nodes[debug_shape_index].assume_safe();
                                instance.show();
                                instance.set_global_position(vec2(
                                    position.x as f32,
                                    position.y as f32,
                                ));

                                instance.set("radius", collision_shape.radius);
                            }
                            debug_shape_index += 1;
                        }
                    }
                }
            }
        }

        // Make sure all debug shapes that aren't valid are hidden
        for index in debug_shape_index..self.debug_nodes.len() {
            unsafe {
                self.debug_nodes[index].assume_safe().hide();
            }
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<GoreKill>();
}

godot_init!(init);

#[derive(Debug, Clone, PartialEq)]
pub enum ManageErrs {
    CouldNotMakeInstance,
    RootClassNotSpatial(String),
}

pub fn load_scene(path: &str) -> Option<Ref<PackedScene, ThreadLocal>> {
    let scene = ResourceLoader::godot_singleton().load(path, "PackedScene", false)?;

    let scene = unsafe { scene.assume_thread_local() };

    scene.cast::<PackedScene>()
}

/// Root here is needs to be the same type (or a parent type) of the node that you put in the child
///   scene as the root. For instance Spatial is used for this example.
fn instance_scene<Root>(scene: &PackedScene) -> Result<Ref<Root, Unique>, ManageErrs>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .ok_or(ManageErrs::CouldNotMakeInstance)?;
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .map_err(|instance| ManageErrs::RootClassNotSpatial(instance.name().to_string()))
}
