use crate::{
    data_structures::queue::Queue,
    timing::{hz_to_duration, Stopwatch},
};

pub use crate::timing::Duration;

pub trait Config: Sized + Copy + Clone {
    /// Whether to use a fixed timestep for the game
    fn fixed_timestep(&self) -> bool;
    /// If `fixed_timestep()` == true, then execute at the given hz. Not providing one will default to 1hz.
    fn sim_hz(&self) -> Option<u32>;
}

/// Messages a simulation may pass back to the engine
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlMessage {
    Ok,
    ExitSim,
}

/// Common functionality a simulation must implement.
pub trait Simulation<Cfg, Msg> {
    /// Creates a new simulation.
    fn new(config: Cfg) -> Self;

    /// A single 'tick' for an application.
    fn tick(&mut self, delta_t: Duration, messages: &[Msg]) -> ControlMessage;
}

/// Executor for simulation. Handles timestepping.
pub struct SimulationExecutor<Sim, Cfg, Msg>
where
    Sim: Simulation<Cfg, Msg>,
    Cfg: Config,
{
    use_fixed_timestep: bool,
    time_keeper: Timekeeper,
    sim: Sim,
    engine_queue: Queue<Msg>,
    config: Cfg,
}

impl<Sim, Cfg, Msg> SimulationExecutor<Sim, Cfg, Msg>
where
    Sim: Simulation<Cfg, Msg>,
    Cfg: Config,
{
    /// Creates a new SimulationExecutor
    pub fn new(max_engine_msgs: usize, config: Cfg) -> Self {
        let sim_hz = match config.sim_hz() {
            Some(hz) => hz.max(1),
            None => 0,
        };

        let time_keeper = Timekeeper {
            tick_duration: hz_to_duration(sim_hz),
            accumulated_time: Duration::from_secs(0),
            simulation_stopwatch: Stopwatch::new(),
        };

        let sim = Sim::new(config);

        Self {
            use_fixed_timestep: config.fixed_timestep(),
            time_keeper,
            sim,
            engine_queue: Queue::new(max_engine_msgs),
            config,
        }
    }

    /// Passes in the input message and attempts to execute.
    pub fn tick(&mut self, input: Option<Msg>) -> ControlMessage {
        let mut control_msg = ControlMessage::Ok;

        // Queue up any messages
        if let Some(input) = input {
            self.engine_queue.push(input);
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

                control_msg = self
                    .sim
                    .tick(self.time_keeper.tick_duration, &self.engine_queue.items());
                self.engine_queue.clear();

                // Break out if the sim is taking too long. Or it should shut down.
                // This way it keeps processing and doesn't get stuck in a horrendous loop. It'll slow the game down
                // to a crawl, but at least it isn't preventing people from playing.
                if times_ticked >= MAX_TICKS_PER_FRAME || control_msg == ControlMessage::ExitSim {
                    break;
                }
            }
        }
        // Otherwise execute it
        else {
            let delta_t = self.time_keeper.simulation_stopwatch.elapsed();
            control_msg = self.sim.tick(delta_t, &self.engine_queue.items());
            self.engine_queue.clear();
        }

        // Return the control message
        control_msg
    }
}

/// Time tracking record manager
struct Timekeeper {
    tick_duration: Duration,
    accumulated_time: Duration,
    simulation_stopwatch: Stopwatch,
}
