use window::{Window, WindowControl, WindowInput};

// This contains a dummy implementation of a renderer + window for the server

/// A dummy renderer used by the server
struct DummyRenderer {}
impl DummyRenderer {
    pub fn new() -> Self {
        Self {}
    }
}
impl renderer::Renderer for DummyRenderer {}

/// Dummy window for the server
pub struct DummyWindow {}

impl DummyWindow {
    /// Creates a new dummy window
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl<MainFunc> Window<MainFunc> for DummyWindow
where
    MainFunc: FnMut(Option<WindowInput>, &mut dyn renderer::Renderer) -> WindowControl + 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, mut main_loop_function: MainFunc) {
        let mut dummy_renderer = DummyRenderer::new();

        loop {
            match main_loop_function(None, &mut dummy_renderer) {
                WindowControl::Ok => {
                    // keep going
                }
                WindowControl::Shutdown => {
                    // TODO: cleanup
                    todo!("cleanup server")
                }
            }
        }
    }
}
