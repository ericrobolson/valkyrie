use window::{InteractableSimulation, Window, WindowControl, WindowInput};

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

impl<Sim> Window<Sim> for DummyWindow
where
    Sim: InteractableSimulation + 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, mut simulation: Sim) {
        let mut dummy_renderer = DummyRenderer::new();

        loop {
            match simulation.tick(None, &mut dummy_renderer) {
                WindowControl::Ok => {
                    // Do nothing
                }
                WindowControl::Shutdown => {
                    // TODO: cleanup
                    todo!("cleanup server")
                }
            }
        }
    }
}
