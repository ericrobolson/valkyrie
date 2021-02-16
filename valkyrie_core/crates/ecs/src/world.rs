use crate::{entity::MAX_ENTITIES, ComponentStore, ComponentStoreError, Entity, EntityManager};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WorldType {
    Client,
    Server,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NetworkType {
    ClientAuthorative,
    ServerAuthoratative,
    None,
}

pub trait WorldImplementation {
    /// Instantiates a new world
    fn new(world_type: WorldType) -> Self;

    /// Returns the type of the world
    fn world_type(&self) -> WorldType;

    /// Creates a new entity
    fn add_entity(&mut self) -> Entity;

    /// Returns whether the entity is alive
    fn is_alive(&self, entity: Entity) -> bool;

    /// Queue the entity for destruction.
    fn destroy(&mut self, entity: Entity);

    /// Execute all systems for the world.
    fn dispatch(&mut self);
}

#[macro_export]
macro_rules! define_world {
    {
        id: $world:ident,
        components: {$({$component_id:ident : $component_type:ty, $networked:expr, capacity: $component_size:expr }), *},
        systems: [$($system_execution:ident),*]
    } => {
        pub struct $world {
            world_type: WorldType,
            entity_manager: EntityManager,
            destroyed_entities: Vec<Entity>,
            $(pub $component_id : ComponentStore<$component_type>,)*
        }


        impl WorldImplementation for $world {
            /// Instantiates a new world
            fn new(world_type: WorldType) -> Self {
                Self {
                    world_type,
                    entity_manager: EntityManager::new(),
                    destroyed_entities: Vec::with_capacity(MAX_ENTITIES),
                    $($component_id : ComponentStore::new($component_size),)*

                }
            }

            /// Returns the type of the world
            fn world_type(&self) -> WorldType {
                self.world_type
            }

            /// Creates a new entity
            fn add_entity(&mut self) -> Entity {
                self.entity_manager.create()
            }

            /// Returns whether the entity is alive
            fn is_alive(&self, entity: Entity) -> bool {
                self.entity_manager.is_alive(entity)
            }

            /// Queue the entity for destruction.
            fn destroy(&mut self, entity: Entity) {
                self.entity_manager.destroy(entity);
                self.destroyed_entities.push(entity);
            }

            /// Execute all systems for the world.
            fn dispatch(&mut self) {
                // System calls
                {
                    $(
                        $system_execution(self)
                    );*
                }

                // Garbage collection
                for destroyed_entity in self.destroyed_entities.iter() {
                    $(self.$component_id.destroy(*destroyed_entity));*
                }

                self.destroyed_entities.clear();
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_func(world: &mut World) {}

    define_world! {
        id: World,
        components: {
            {
                alive: bool,
                NetworkType::None,
                capacity: 300
            },
            {
                alive2: u8,
                NetworkType::None,
                capacity: 300
            }
        },
        systems: [test_func]
    }

    #[test]
    fn world() {
        let mut world = World::new(WorldType::Server);
        let entity = world.add_entity();
        match world.alive.add(entity) {
            Ok(mut alive) => {
                *alive = true;
            }
            Err(_) => {
                todo!()
            }
        }

        match world.alive.get(entity) {
            Some(alive) => {
                assert_eq!(true, *alive);
            }
            None => {
                todo!()
            }
        }

        world.destroy(entity);
        world.dispatch();

        assert_eq!(false, world.is_alive(entity));
        match world.alive.get(entity) {
            Some(_) => {
                todo!()
            }
            None => {
                // ok
            }
        }
    }
}
