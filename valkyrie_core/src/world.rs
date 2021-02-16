use ecs::{Entity, EntityManager};

// Note; World is a wrapper for two inner worlds: Client + Server
// This enables you to use the same interfaces for games, but manage it internally. not sure i like that...
// Should they be kept separate? What about marking components as 'client authorative', 'server authoratitive', and the like?
// also a 'client provide' type, such as input?

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WorldType {
    Client,
    Server,
    ClientServer,
}

pub struct World {
    world_type: WorldType,
    client_world: Option<InnerWorld>,
    server_world: Option<InnerWorld>,
}

impl World {
    pub fn new(world_type: WorldType) -> Self {
        todo!()
    }

    pub fn world_type(&self) -> WorldType {
        self.world_type
    }

    pub fn add_entity(&mut self) -> Entity {
        self.entity_manager.create()
    }

    pub fn is_active(&self, entity: Entity) -> bool {
        self.entity_manager.is_alive(entity)
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.entity_manager.destroy(entity);
        self.destroyed_entities.push(entity);
    }
}

impl Drop for World {
    fn drop(&mut self) {
        match self.world_type {
            WorldType::Client => {
                // Just bail
            }
            WorldType::Server => {
                todo!("Let clients know that the server is closing.")
            }
            WorldType::ClientServer => {}
        }
    }
}

/// Triggers garbage collection on the GameWorld.
pub fn garbage_collect(game_world: &mut World) {
    /*
    for destroyed_entity in game_world.destroyed_entities.iter() {
        // TODO: for each system, destroy that entity.
    }

    game_world.destroyed_entities.clear();
     */
}

// TODO: add ability to specify client or server components. For example, server's don't need to sync resolution stuff or option menus.

struct InnerWorld {
    entity_manager: EntityManager,
    destroyed_entities: Vec<Entity>,
}

impl InnerWorld {
    pub fn new() -> Self {
        Self {
            destroyed_entities: Vec::with_capacity(Entity::MAX_ENTITIES()),
            entity_manager: EntityManager::new(),
        }
    }

    pub fn add_entity(&mut self) -> Entity {
        self.entity_manager.create()
    }

    pub fn is_active(&self, entity: Entity) -> bool {
        self.entity_manager.is_alive(entity)
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.entity_manager.destroy(entity);
        self.destroyed_entities.push(entity);
    }
}
