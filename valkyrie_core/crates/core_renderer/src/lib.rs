use core_ecs::*;

pub struct Renderer {
    backend: Box<dyn BackendRenderer>,
}

impl Renderer {
    fn new(backend: Box<dyn BackendRenderer>) -> Self {
        Self { backend }
    }
}

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
