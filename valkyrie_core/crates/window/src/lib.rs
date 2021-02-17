/// Input from a window
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WindowInput {
    Shutdown,
    RedrawRequested,
    Resize { w: u32, h: u32 },
}

/// Control messages provided to the window
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WindowControl {
    /// Do nothing, keep processing
    Ok,
    /// Requests a render
    Render,
    /// Update Renderer
    UpdateRenderState,
    /// Shut down
    Shutdown,
}

/// A tickable simulation.
pub trait Simulation {
    fn tick(&mut self, input: Option<WindowInput>) -> WindowControl;
}

/// A renderable simulation
pub trait Renderable {
    fn render(&self, renderer: &mut dyn renderer::Renderer);
}

///  Implementation of a window
pub trait Window<Sim>
where
    Sim: Simulation + Renderable + 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, simulation: Sim);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
