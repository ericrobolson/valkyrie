use crate::{
    application::*,
    data_structures::queue::Queue,
    renderer::Renderer,
    timing::{hz_to_duration, Duration, Stopwatch},
    window,
    window::{Renderable, WindowControl},
};

pub trait Config: Sized + Copy + Clone {
    fn sim_hz(&self) -> Option<u32>;
}

struct Timekeeper {
    tick_duration: Duration,
    accumulated_time: Duration,
    simulation_stopwatch: Stopwatch,
}

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

pub struct SimManager<Sim, Cfg>
where
    Sim: Simulation<Cfg>,
    Cfg: Config,
{
    use_fixed_timestep: bool,
    time_keeper: Timekeeper,
    sim: Sim,
    engine_queue: Queue<EngineMessage>,
    config: Cfg,
}

impl<Sim, Cfg> window::Simulation for SimManager<Sim, Cfg>
where
    Sim: Simulation<Cfg>,
    Cfg: Config,
{
    fn tick(&mut self, input: Option<window::WindowInput>) -> WindowControl {
        let mut window_control = WindowControl::Ok;

        // TODO: networking fun

        // Queue up any messages
        if let Some(input) = input {
            self.engine_queue.push(EngineMessage::Input(input));
        }

        // If we're using a fixed time step, see if it should be executed.
        if self.use_fixed_timestep {
            // Increase accumulated time + tick if necessary
            // Based on https://gafferongames.com/post/fix_your_timestep/ to divorce rendering + simulations

            self.time_keeper.accumulated_time += self.time_keeper.simulation_stopwatch.elapsed();

            // In the event that the loop gets in a spiral of death where the sim can't keep up,
            // clamp it to a set number of ticks per frame to prevent spiraling downward.
            const MAX_TICKS_PER_FRAME: u8 = 10;
            let mut times_ticked = 0;

            // Tick the simulation until it has caught up
            while self.time_keeper.accumulated_time > self.time_keeper.tick_duration {
                self.time_keeper.accumulated_time -= self.time_keeper.tick_duration;
                times_ticked += 1;

                window_control = self.execute_sim_tick(window_control);

                // Break out if the sim is taking too long. Or it should shut down.
                // This way it keeps processing and doesn't get stuck in a horrendous loop. It'll slow the game down
                // to a crawl, but at least it isn't preventing people from playing.
                if times_ticked >= MAX_TICKS_PER_FRAME || window_control == WindowControl::Shutdown
                {
                    break;
                }
            }
        }
        // Otherwise execute it on any non-empty inputs
        else if let Some(input) = input {
            window_control = self.execute_sim_tick(window_control);
        }

        if window_control == WindowControl::Shutdown {
            todo!("How to shut down? How to deal with servers and/or clients?");
        }

        // Return the window control
        window_control
    }
}

impl<Sim, Cfg> SimManager<Sim, Cfg>
where
    Sim: Simulation<Cfg>,
    Cfg: Config,
{
    pub fn new(max_engine_msgs: usize, config: Cfg) -> Self {
        let (use_fixed_timestep, sim_hz) = match config.sim_hz() {
            Some(hz) => (true, hz.max(1)),
            None => (false, 0),
        };

        let time_keeper = Timekeeper {
            tick_duration: hz_to_duration(sim_hz),
            accumulated_time: Duration::from_secs(0),
            simulation_stopwatch: Stopwatch::new(),
        };

        let sim = Sim::new(config);

        Self {
            use_fixed_timestep,
            time_keeper,
            sim,
            engine_queue: Queue::new(max_engine_msgs),
            config,
        }
    }

    pub fn execute_sim_tick(&mut self, window_control: WindowControl) -> WindowControl {
        let msg = self.sim.tick(&self.engine_queue.items());

        match msg {
            ControlMessage::Ok => {
                self.engine_queue.clear();
            }
            ControlMessage::Shutdown => {
                return WindowControl::Shutdown;
            }
            ControlMessage::RequestServerChange => {
                // If requesting local, reset local server state

                // If requesting remote, connect to remote

                todo!("Request changing the server to either a remote or local.");
            }
            ControlMessage::RequestRedraw => {
                // Only need to request a redraw if not shutting down or updating the render state
                if window_control != WindowControl::Shutdown
                    && window_control != WindowControl::UpdateRenderState
                {
                    return WindowControl::Render;
                }
            }
            ControlMessage::RequestRenderStateUpdate => {
                return WindowControl::UpdateRenderState;
            }
        }

        window_control
    }
}
