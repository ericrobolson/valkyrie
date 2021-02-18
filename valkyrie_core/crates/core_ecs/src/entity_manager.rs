use crate::{
    entity::{EntityId, Generation, MAX_ENTITIES},
    Entity,
};

use core_data_structures::queue::Queue;

const MINIMUM_FREE_INDICES: usize = 1024;

#[derive(Debug)]
/// Class that manages creating and deletion of entities.
pub struct EntityManager {
    generations: Vec<Generation>,
    free_entity_ids: Queue<EntityId>,
    next_id: EntityId,
}

impl EntityManager {
    /// Creates a new instance of an EntityManager.
    pub fn new() -> Self {
        // Populate all generations to 0
        let mut generations = Vec::with_capacity(MAX_ENTITIES);
        for _ in 0..MAX_ENTITIES {
            generations.push(0);
        }

        Self {
            next_id: 0,
            generations,
            free_entity_ids: Queue::new(MAX_ENTITIES),
        }
    }

    /// Creates a new entity.
    pub fn create(&mut self) -> Entity {
        let entity_id = if self.free_entity_ids.len() > MINIMUM_FREE_INDICES {
            let index = match self.free_entity_ids.pop() {
                Some(e) => e,
                None => 0,
            };

            index
        } else {
            let id = self.next_id;
            self.next_id += 1;
            if self.next_id as usize >= MAX_ENTITIES {
                self.next_id = 0;
            }

            id
        };

        Entity::new(entity_id, self.generations[entity_id as usize])
    }

    /// Returns whether the given entity is classified as 'alive'.
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.generations[entity.id() as usize] == entity.generation()
    }

    /// Destroys the given entity.
    pub fn destroy(&mut self, entity: Entity) {
        // Increase the generation id for the given entity.
        let id = {
            let id = entity.id() as usize;
            let generation_len = self.generations.len();

            id % generation_len
        };

        // If generation hasn't already been incremented free it
        if self.generations[id] == entity.generation() {
            self.generations[id] = self.generations[id].wrapping_add(1);

            // Mark it as a 'free' entity to use later on.
            self.free_entity_ids.push(entity.id());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_manager_create_adds_new_entity() {
        let mut manager = EntityManager::new();

        // Basic test
        for i in 0..MAX_ENTITIES - 1 {
            let i = i as u32;
            let entity = manager.create();
            assert_eq!(Entity::new(i, 0), entity);
            assert_eq!(i + 1, manager.next_id);
        }

        // Now check for wrapping add.
        // This could lead to random entities having components they shouldn't,
        // but that seems better than exceptions or not creating entities.
        let entity = manager.create();
        let id = (MAX_ENTITIES - 1) as u32;
        assert_eq!(Entity::new(id, 0), entity);
        assert_eq!(0, manager.next_id);

        // Now check for generational
        for entity in 0..MAX_ENTITIES {
            let entity = Entity::new(entity as EntityId, 0);
            manager.destroy(entity);
        }

        let entity = manager.create();
        assert_eq!(Entity::new(0, 1), entity);
    }

    #[test]
    fn entity_manager_destroy_removes_entity() {
        let mut manager = EntityManager::new();

        let entity = manager.create();
        manager.destroy(entity);

        let id = entity.id() as usize;
        assert_eq!(1, manager.generations[id]);
        assert_eq!(entity.id(), manager.free_entity_ids.items()[0]);

        // Only free entities that have a matching generation; if they don't match do nothing
        manager.destroy(entity);
        assert_eq!(1, manager.generations[id]);
        assert_eq!(entity.id(), manager.free_entity_ids.items()[0]);

        // Now check that it wraps the generation if it goes over the max generations
        manager.generations[id] = Generation::MAX;
        manager.free_entity_ids.clear();

        let entity = Entity::new(entity.id(), Generation::MAX);
        manager.destroy(entity);
        assert_eq!(0, manager.generations[id]);
        assert_eq!(entity.id(), manager.free_entity_ids.items()[0]);
    }

    #[test]
    fn entity_manager_is_alive_generation_mismatch_returns_false() {
        let mut manager = EntityManager::new();
        manager.generations[0] = 1;

        let entity = Entity::new(0, 0);

        assert_eq!(false, manager.is_alive(entity));
    }

    #[test]
    fn entity_manager_is_alive_generation_matches_returns_true() {
        let mut manager = EntityManager::new();
        manager.generations[0] = 0;

        let entity = Entity::new(0, 0);

        assert_eq!(true, manager.is_alive(entity));
    }

    #[test]
    fn entity_manager_new_returns_expected() {
        let manager = EntityManager::new();

        // Make sure all generations are set to 0
        let mut generations = Vec::with_capacity(MAX_ENTITIES);
        for _ in 0..MAX_ENTITIES {
            generations.push(0);
        }

        assert_eq!(generations, manager.generations);
        assert_eq!(0, manager.free_entity_ids.len());
    }
}
