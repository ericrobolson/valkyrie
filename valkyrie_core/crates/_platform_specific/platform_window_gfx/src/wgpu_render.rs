use core_renderer::{BackendRenderer, Renderer};

pub fn make() -> impl BackendRenderer {
    WgpuRenderer {}
}

struct WgpuRenderer {}
impl BackendRenderer for WgpuRenderer {
    fn dispatch(&mut self) {
        println!("DA WGPU");
    }
}

impl Drop for WgpuRenderer {
    fn drop(&mut self) {}
}
