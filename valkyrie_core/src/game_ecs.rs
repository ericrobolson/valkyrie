use ecs::{Entity, EntityManager};

pub struct GameWorld {
    entity_manager: EntityManager,
    destroyed_entities: Vec<Entity>,
}

impl GameWorld {
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

/// Triggers garbage collection on the GameWorld.
pub fn garbage_collect(game_world: &mut GameWorld) {
    for destroyed_entity in game_world.destroyed_entities.iter() {
        // TODO: for each system, destroy that entity.
    }

    game_world.destroyed_entities.clear();
}
