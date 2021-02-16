use crate::{World, WorldType};

pub fn render_world(world: &World, renderer: &mut dyn renderer::Renderer) {
    if world.world_type() == WorldType::Server {
        return;
    }

    //println!("TODO: render the world!");
}
