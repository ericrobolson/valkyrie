use valkyrie_core::ecs::*;
use valkyrie_core::simulation::*;

pub mod components;

pub enum Message {}

pub struct GoreKillConfig {
    /// The number of starting lives the player begins with
    pub starting_lives: u8,
    /// Whether the player may retry the game
    pub retries_enabled: bool,
    /// Whether slowtime is enabled
    pub slowtime_enabled: bool,
    /// Whether a player may gain lives from points scored
    pub gain_lives_enabled: bool,
}

impl GoreKillConfig {
    /// The normal mode for GORE_KILL.
    pub fn normal_mode() -> Self {
        Self {
            starting_lives: 3,
            retries_enabled: true,
            slowtime_enabled: true,
            gain_lives_enabled: true,
        }
    }

    /// 1cc normal mode for GORE_KILL.
    pub fn normal_mode_1cc() -> Self {
        Self {
            starting_lives: 3,
            retries_enabled: false,
            slowtime_enabled: true,
            gain_lives_enabled: true,
        }
    }

    /// God mode for GORE_KILL.
    pub fn god_mode() -> Self {
        Self {
            starting_lives: 3,
            retries_enabled: true,
            slowtime_enabled: false,
            gain_lives_enabled: true,
        }
    }

    /// God mode for GORE_KILL.
    pub fn god_mode_1cc() -> Self {
        Self {
            starting_lives: 3,
            retries_enabled: false,
            slowtime_enabled: false,
            gain_lives_enabled: true,
        }
    }

    /// Insanity mode for GORE_KILL.
    pub fn insanity() -> Self {
        Self {
            starting_lives: 1,
            retries_enabled: false,
            slowtime_enabled: false,
            gain_lives_enabled: false,
        }
    }
}

pub struct GoreKillSim {
    tick: usize,
    pub world: World,
    pub dirty: bool,
}

impl Simulation<GoreKillConfig, Message> for GoreKillSim {
    fn new(config: GoreKillConfig) -> Self {
        let mut world = World::new();
        {
            //TODO: This is a super heavy op. Maybe do in a separate thread?
            use components::*;
            world.register::<Collidable>(200);
            world.register::<Position>(200);
            world.register::<Debug>(200);
            world.register::<Player>(1);

            // Add a player entity
            let player = world.add_entity();
            match world.add::<Collidable>(player) {
                Ok(collidable) => {
                    collidable.radius = 200;
                }
                _ => {}
            }
            world.add::<Debug>(player).unwrap();
            world.add::<Player>(player).unwrap();
            match world.add::<Position>(player) {
                Ok(position) => {
                    position.x = 10;
                    position.y = 10;
                }
                Err(_) => {}
            }
        }

        Self {
            tick: 0,
            world,
            dirty: false,
        }
    }

    fn tick(&mut self, delta_t: Duration, messages: &[Message]) -> ControlMessage {
        self.tick += 1;
        self.dirty = true;

        if self.tick % 2 == 0 {
            return ControlMessage::ExitSim;
        }

        ControlMessage::Ok
    }
}
