use core_data_structures::queue::Queue;
use core_renderer::{BackendRenderer, Renderer};

pub fn make() -> impl BackendRenderer {
    WgpuRenderer {}
}

struct WgpuRenderer {}
impl BackendRenderer for WgpuRenderer {
    fn dispatch(&mut self) {
        println!("DA WGPU");
    }

    fn set_render_pass(&mut self, commands: &Queue<core_renderer::RenderCommand>) {
        for command in commands.items() {
            match command {
                core_renderer::RenderCommand::UpdateCamera => {
                    println!("update camera");
                }
            }
        }
    }
}

impl Drop for WgpuRenderer {
    fn drop(&mut self) {}
}
