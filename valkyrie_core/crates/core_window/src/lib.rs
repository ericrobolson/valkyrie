use core_simulation::{Simulation, SimulationExecutor};

/// A renderable simulation
pub trait Renderable {
    fn render(&self, renderer: &mut core_renderer::Renderer);
}

///  Implementation of a window
pub trait Window<Sim, Cfg, Msg>
where
    Sim: Simulation<Cfg, Msg> + Renderable + 'static,
{
    /// Implementation of the 'main loop' that drives the window. Note: in implementations may need to make main_loop_function() mutable.
    fn execute(&mut self, executor: SimulationExecutor<Sim, Cfg, Msg>);
}
