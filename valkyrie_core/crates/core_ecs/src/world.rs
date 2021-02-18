use crate::component_store::BackingComponentStore;
use crate::{ComponentStore, ComponentStoreError, Entity};
use core_data_structures::hashmap::HashMap;

pub enum WorldError {
    ComponentStoreError(ComponentStoreError),
    ComponentNotRegistered,
}

impl From<ComponentStoreError> for WorldError {
    fn from(error: ComponentStoreError) -> Self {
        Self::ComponentStoreError(error)
    }
}

pub struct World {
    components: HashMap<ResourceId, Box<dyn BackingComponentStore>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ResourceId {
    type_id: core::any::TypeId,
}

impl ResourceId {
    fn from<Component>() -> Self
    where
        Component: Sized + Default + Clone + 'static,
    {
        Self {
            type_id: core::any::TypeId::of::<Component>(),
        }
    }
}

impl World {
    /// Registers a given component with the world. If the component has already been registered, does nothing.
    pub fn register<Component>(&mut self, capacity: usize)
    where
        Component: Sized + Default + Clone + 'static,
    {
        let component_id = ResourceId::from::<Component>();
        if self.components.get(&component_id).is_none() {
            self.components.insert(
                component_id,
                Box::new(ComponentStore::<Component>::new(capacity)),
            );
        }
    }

    /// Adds a component to the entity.
    pub fn add<Component>(&mut self, entity: Entity) -> Result<&mut Component, WorldError>
    where
        Component: Sized + Default + Clone + 'static,
    {
        if let Some(store) = self.components.get_mut(&ResourceId::from::<Component>()) {
            match store
                .as_any_mut()
                .downcast_mut::<ComponentStore<Component>>()
            {
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
    pub fn remove<Component>(&mut self, entity: Entity)
    where
        Component: Sized + Default + Clone + 'static,
    {
        if let Some(store) = self.components.get_mut(&ResourceId::from::<Component>()) {
            match store
                .as_any_mut()
                .downcast_mut::<ComponentStore<Component>>()
            {
                Some(component_store) => {
                    component_store.destroy(entity);
                }
                None => {}
            }
        }
    }

    /// Kills a given entity
    pub fn kill(&mut self, entity: Entity) {
        for store in self.components.values_mut() {
            store.destroy(entity);
        }
    }
}

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
