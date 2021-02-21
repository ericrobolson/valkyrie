// TODO: need to fix up the sim. Not using a rust native solution right now, so avoiding refactoring.

use crate::{
    core_application::*,
    data_structures::queue::Queue,
    renderer::Renderer,
    simulation::*,
    timing::{hz_to_duration, Duration, Stopwatch},
    window,
    window::{Renderable, WindowControl},
};

pub struct RenderableSimManager<Sim, Cfg>
where
    Sim: Simulation<Cfg> + window::Renderable,
    Cfg: Config,
{
    sim_manager: SimManager<Sim, Cfg>,
}

impl<Sim, Cfg> RenderableSimManager<Sim, Cfg>
where
    Sim: Simulation<Cfg> + window::Renderable,
    Cfg: Config,
{
    pub fn new(max_engine_msgs: usize, config: Cfg) -> Self {
        Self {
            sim_manager: SimManager::new(max_engine_msgs, config),
        }
    }
}

impl<Sim, Cfg> window::Simulation for RenderableSimManager<Sim, Cfg>
where
    Sim: Simulation<Cfg> + window::Renderable,
    Cfg: Config,
{
    fn tick(&mut self, input: Option<window::WindowInput>) -> WindowControl {
        self.sim_manager.tick(input)
    }
}

impl<Sim, Cfg> window::Renderable for RenderableSimManager<Sim, Cfg>
where
    Sim: Simulation<Cfg> + window::Renderable,
    Cfg: Config,
{
    fn render(&self, renderer: &mut Renderer) {
        self.sim_manager.sim.render(renderer);
    }
}
