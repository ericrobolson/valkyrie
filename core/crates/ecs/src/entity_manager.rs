use crate::{
    entity::{EntityId, Generation, MAX_ENTITIES},
    Entity,
};

use data_structures::queue::Queue;

const MINIMUM_FREE_INDICES: usize = 1024;

pub struct EntityManager {
    generations: Vec<Generation>,
    free_entity_ids: Queue<EntityId>,
    next_id: EntityId,
}

impl EntityManager {
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

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.generations[entity.id() as usize] == entity.generation()
    }

    pub fn destroy(&mut self, entity: Entity) {
        // Increase the generation id for the given entity.
        let id = entity.id() as usize;
        self.generations[id] = self.generations[id].wrapping_add(1);

        // Mark it as a 'free' entity to use later on.
        self.free_entity_ids.push(entity.id());
    }
}
