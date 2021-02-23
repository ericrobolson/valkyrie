use valkyrie_core::ecs::*;
use valkyrie_core::simulation::*;

pub mod components;

pub enum AttackType {
    /// Focused attack that cuts movespeed in half
    LaserAttack,
    /// Wide, but weak, attack
    ShotAttack,
    // TODO: Bomb attack
    // TODO: special/turbo attack
}

pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub enum Message {
    Move(MoveDirection),
    Attack(AttackType),
}

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
    player_entity: Entity,
}

impl Simulation<GoreKillConfig, Message> for GoreKillSim {
    fn new(config: GoreKillConfig) -> Self {
        let mut world = World::new();

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

        Self {
            tick: 0,
            world,
            dirty: false,
            player_entity: player,
        }
    }

    fn tick(&mut self, delta_t: Duration, messages: &[Message]) -> ControlMessage {
        self.tick += 1;
        self.dirty = true;

        // Do player input
        {
            let mut move_x = 0;
            let mut move_y = 0;
            let mut fire_bomb = false;
            let mut fire_turbo = false;
            let mut laser_shot = false;
            let mut shot = false;

            for msg in messages {
                match msg {
                    Message::Move(mov) => match mov {
                        MoveDirection::Up => move_y = 1,
                        MoveDirection::Down => move_y = -1,
                        MoveDirection::Left => move_x = -1,
                        MoveDirection::Right => move_x = 1,
                    },
                    Message::Attack(atk) => match atk {
                        AttackType::LaserAttack => laser_shot = true,
                        AttackType::ShotAttack => shot = true,
                    },
                }
            }

            match self
                .world
                .get_mut::<components::Position>(self.player_entity)
            {
                Some(position) => {
                    position.x += move_x;
                    position.y += move_y;
                }
                None => {
                    gdnative::prelude::godot_print!("Nothing?");
                }
            }
        }

        ControlMessage::Ok
    }
}
