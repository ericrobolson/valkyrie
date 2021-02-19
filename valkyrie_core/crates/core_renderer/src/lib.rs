use core_ecs::*;

// Top level renderer.
// Does things at the scene level, to allow individual backends to handle what's going on.

pub struct Renderer {
    backend: Box<dyn BackendRenderer>,
}

pub struct LightId;

impl Renderer {
    fn new(backend: Box<dyn BackendRenderer>) -> Self {
        Self { backend }
    }

    pub fn add_light(&mut self) -> LightId {
        todo!()
    }

    pub fn delete_light(&mut self, light: LightId) {
        todo!()
    }

    pub fn dispatch(&mut self) {}
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageFile {
    name: String,
    bytes: Vec<u8>,
}

impl Component for ImageFile {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Buffered;

impl Component for Buffered {}

#[derive(Debug, Clone, PartialEq)]
enum GpuResource {
    NotLoaded,
    Loaded,
}
impl Default for GpuResource {
    fn default() -> Self {
        Self::NotLoaded
    }
}

impl Component for GpuResource {}

pub trait BackendRenderer {}

pub fn make_renderer(backend: Box<dyn BackendRenderer>) -> Renderer {
    Renderer::new(backend)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
