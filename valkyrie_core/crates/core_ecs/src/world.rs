use crate::component_store::BackingComponentStore;
use crate::{entity_manager::EntityManager, ComponentStore, ComponentStoreError, Entity};
use core_data_structures::hashmap::HashMap;

#[derive(Debug)]
pub enum WorldError {
    ComponentStoreError(ComponentStoreError),
    ComponentNotRegistered,
}

impl From<ComponentStoreError> for WorldError {
    fn from(error: ComponentStoreError) -> Self {
        Self::ComponentStoreError(error)
    }
}

#[derive(Debug)]
pub struct World {
    entity_manager: EntityManager,
    alive_entities: Vec<Entity>,
    alive_index: usize,
    components: HashMap<ResourceId, Box<dyn BackingComponentStore>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId {
    type_id: core::any::TypeId,
}

pub trait Component: Sized + Default + Clone + std::fmt::Debug + 'static {}

impl ResourceId {
    fn from<C>() -> Self
    where
        C: Component,
    {
        Self {
            type_id: core::any::TypeId::of::<C>(),
        }
    }
}

impl World {
    /// Creates a new world
    pub fn new() -> Self {
        Self {
            alive_entities: vec![Entity::new(0, 0); Entity::MAX_ENTITIES()],
            alive_index: 0,
            entity_manager: EntityManager::new(),
            components: HashMap::new(),
        }
    }

    /// Registers a given component with the world. If the component has already been registered, does nothing.
    pub fn register<C>(&mut self, capacity: usize)
    where
        C: Component,
    {
        let component_id = ResourceId::from::<C>();
        if self.components.get(&component_id).is_none() {
            self.components
                .insert(component_id, Box::new(ComponentStore::<C>::new(capacity)));
        }
    }

    /// Adds a new entity
    pub fn add_entity(&mut self) -> Entity {
        let entity = self.entity_manager.create();
        self.alive_entities[self.alive_index] = entity;
        self.alive_index += 1;
        entity
    }

    /// Retrieves a component for a given entity
    pub fn get<C>(&self, entity: Entity) -> Option<&C>
    where
        C: Component,
    {
        if let Some(store) = self.components.get(&ResourceId::from::<C>()) {
            match store.as_any().downcast_ref::<ComponentStore<C>>() {
                Some(component_store) => {
                    return component_store.get(entity);
                }
                None => {}
            }
        }

        None
    }

    /// Returns a list of entities that are alive
    pub fn entities(&self) -> &[Entity] {
        &self.alive_entities[0..self.alive_index]
    }

    pub fn entity_len(&self) -> usize {
        self.alive_index
    }

    /// Retrieves a mutable component for a given entity
    pub fn get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
    where
        C: Component,
    {
        if let Some(store) = self.components.get_mut(&ResourceId::from::<C>()) {
            match store.as_any_mut().downcast_mut::<ComponentStore<C>>() {
                Some(component_store) => {
                    return component_store.get_mut(entity);
                }
                None => {}
            }
        }

        None
    }

    /// Adds a component to the entity.
    pub fn add<C>(&mut self, entity: Entity) -> Result<&mut C, WorldError>
    where
        C: Component,
    {
        if let Some(store) = self.components.get_mut(&ResourceId::from::<C>()) {
            match store.as_any_mut().downcast_mut::<ComponentStore<C>>() {
                Some(component_store) => match component_store.add(entity) {
                    Ok(c) => return Ok(c),
                    Err(e) => return Err(e.into()),
                },
                None => {}
            }
        }

        return Err(WorldError::ComponentNotRegistered);
    }

    /// Removes a component for a given entity
    pub fn remove<C>(&mut self, entity: Entity)
    where
        C: Component,
    {
        if let Some(store) = self.components.get_mut(&ResourceId::from::<C>()) {
            match store.as_any_mut().downcast_mut::<ComponentStore<C>>() {
                Some(component_store) => {
                    component_store.destroy(entity);
                }
                None => {}
            }
        }
    }

    /// Kills a given entity
    pub fn kill(&mut self, entity: Entity) {
        // Mark it as dead and swap it with the last element
        {
            let index_to_remove = self
                .alive_entities
                .iter()
                .position(|e| e.id() == entity.id());

            if self.alive_index > 0 && index_to_remove.is_some() {
                let index_to_remove = index_to_remove.unwrap_or_default();
                self.alive_entities
                    .swap(index_to_remove, self.alive_index - 1);
                self.alive_index -= 1;
            }
        }

        // Kill it on all components
        for store in self.components.values_mut() {
            store.destroy(entity);
        }
    }
}
/*
mod specs_test {
    use specs::prelude::*;

    #[derive(Debug)]
    struct Vel(f32);

    impl Component for Vel {
        type Storage = VecStorage<Self>;
    }

    #[derive(Debug)]
    struct Pos(f32);

    impl Component for Pos {
        type Storage = VecStorage<Self>;
    }

    struct SysA;

    impl<'a> System<'a> for SysA {
        // These are the resources required for execution.
        // You can also define a struct and `#[derive(SystemData)]`,
        // see the `full` example.
        type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

        fn run(&mut self, (mut pos, vel): Self::SystemData) {
            // The `.join()` combines multiple component storages,
            // so we get access to all entities which have
            // both a position and a velocity.
            for (pos, vel) in (&mut pos, &vel).join() {
                pos.0 += vel.0;
            }
        }
    }

    fn test_main() {
        // The `World` is our
        // container for components
        // and other resources.
        let mut world = World::new();
        world.register::<Pos>();
        world.register::<Vel>();

        // An entity may or may not contain some component.

        world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
        world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
        world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

        // This entity does not have `Vel`, so it won't be dispatched.
        world.create_entity().with(Pos(2.0)).build();

        // This builds a dispatcher.
        // The third parameter of `with` specifies
        // logical dependencies on other systems.
        // Since we only have one, we don't depend on anything.
        // See the `full` example for dependencies.
        let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();
        // This will call the `setup` function of every system.
        // In this example this has no effect since we already registered our components.
        dispatcher.setup(&mut world);

        // This dispatches all the systems in parallel (but blocking).
        dispatcher.dispatch(&mut world);
    }
}
 */
