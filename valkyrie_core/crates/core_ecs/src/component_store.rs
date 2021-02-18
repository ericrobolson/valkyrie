// Based on http://bitsquid.blogspot.com/2014/09/building-data-oriented-entity-system.html
use crate::{entity::MAX_ENTITIES, entity_manager::EntityManager, Entity};

#[derive(PartialEq, Debug)]
pub enum ComponentStoreError {
    BufferOverflow,
}

pub trait BackingComponentStore: std::fmt::Debug {
    fn destroy(&mut self, entity_to_destroy: Entity);
    fn as_any(&self) -> &dyn core::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
}

impl<Component> BackingComponentStore for ComponentStore<Component>
where
    Component: Sized + Default + Clone + std::fmt::Debug + 'static,
{
    /// Destroys a component for a given entity.
    fn destroy(&mut self, entity_to_destroy: Entity) {
        if self.active_components == 0 {
            return;
        }

        // Take this element
        let index_to_remove = match self.component_index(entity_to_destroy) {
            Some(i) => i,
            None => {
                return;
            }
        };

        // Swap with last
        let last_element_index = self.active_components - 1;
        self.components.swap(index_to_remove, last_element_index);

        // Update the last element to point to the new index
        for i in 0..self.entity_map.len() {
            if let Some((entity_to_keep, index)) = self.entity_map[i] {
                if index == last_element_index {
                    // Found it, so update it to point to the swapped index and break out.
                    self.entity_map[i] = Some((entity_to_keep, index_to_remove));
                    break;
                }
            }
        }

        // Decrease active components + clear this map
        self.entity_map[entity_to_destroy.id() as usize] = None;
        self.active_components -= 1;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct ComponentStore<Component>
where
    Component: Sized + Default + std::fmt::Debug + Clone,
{
    entity_map: Vec<Option<(Entity, usize)>>, // An option here would be using a hashmap instead of a Vec
    components: Vec<Component>,
    active_components: usize,
}

impl<Component> ComponentStore<Component>
where
    Component: Sized + Default + std::fmt::Debug + Clone,
{
    /// Creates a new component store.
    pub fn new(component_capacity: usize) -> Self {
        Self {
            entity_map: vec![None; MAX_ENTITIES],
            components: vec![Component::default(); component_capacity],
            active_components: 0,
        }
    }

    /// Returns the collection of entities
    pub fn entities(&self) -> impl Iterator<Item = Option<&Entity>> {
        self.entity_map.iter().map(|e| match e {
            Some((e, _i)) => Some(e),
            None => None,
        })
    }

    /// Returns the component index for the given entity
    fn component_index(&self, entity: Entity) -> Option<usize> {
        let index = entity.id() as usize;
        match self.entity_map[index] {
            Some((_, index)) => Some(index),
            None => None,
        }
    }

    /// Returns a reference to the component for a given entity.
    pub fn get(&self, entity: Entity) -> Option<&Component> {
        match self.component_index(entity) {
            Some(index) => Some(&self.components[index]),
            None => None,
        }
    }

    /// Returns a mutable reference to the given component.
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut Component> {
        match self.component_index(entity) {
            Some(index) => Some(&mut self.components[index]),
            None => None,
        }
    }

    /// Adds a new component for the given entity.
    pub fn add(&mut self, entity: Entity) -> Result<&mut Component, ComponentStoreError> {
        let id = entity.id() as usize;

        let mut component_index = 0;

        // If it doesn't exist, add it. Can just return existing one if attempting to add again.
        if self.component_index(entity).is_none() {
            if self.active_components == self.components.capacity() {
                return Err(ComponentStoreError::BufferOverflow);
            }
            component_index = self.active_components;
            self.active_components += 1;

            self.entity_map[id] = Some((entity, component_index));
        }

        Ok(&mut self.components[component_index])
    }

    /// Returns the slice for the active components. Not tied to entities.
    pub fn components(&self) -> &[Component] {
        &self.components[0..self.active_components]
    }

    /// Returns a mutable list of the components
    pub fn components_mut(&mut self) -> impl Iterator<Item = &mut Component> {
        self.components[0..self.active_components].iter_mut()
    }

    /*
    /// Removes all entities that aren't alive
    pub fn garbage_collection(&mut self, entity_manager: &EntityManager) {
        // Potential optimization: only destroy a few components at a time

        for map_index in 0..self.entity_map.len() {
            if let Some((entity, _)) = self.entity_map[map_index] {
                if entity_manager.is_alive(entity) == false {
                    self.destroy(entity);
                }
            }
        }
    }
     */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let max_components = 129;
        let store: ComponentStore<u8> = ComponentStore::new(max_components);
        assert_eq!(0, store.active_components);
        assert_eq!(max_components, store.components.capacity());
        assert_eq!(vec![u8::default(); max_components], store.components);
        assert_eq!(vec![None; MAX_ENTITIES], store.entity_map);
    }

    /*
       #[test]
       fn component_garbage_collection() {
           let max_components = 5;
           let mut store: ComponentStore<u8> = ComponentStore::new(max_components);

           let mut entity_manager = EntityManager::new();

           let entity1 = entity_manager.create();
           let entity1_value = 123;
           match store.add(entity1) {
               Ok(e) => {
                   *e = entity1_value;
               }
               _ => todo!(),
           }

           let entity2 = entity_manager.create();
           let entity2_value = 234;
           match store.add(entity2) {
               Ok(e) => {
                   *e = entity2_value;
               }
               _ => todo!(),
           }

           let entity3 = entity_manager.create();
           let entity3_value = 2;
           match store.add(entity3) {
               Ok(e) => {
                   *e = entity3_value;
               }
               _ => todo!(),
           }

           let entity4 = entity_manager.create();
           let entity4_value = 010;
           match store.add(entity4) {
               Ok(e) => {
                   *e = entity4_value;
               }
               _ => todo!(),
           }

           entity_manager.destroy(entity2);
           entity_manager.destroy(entity3);

           store.garbage_collection(&entity_manager);

           assert_eq!(None, store.entity_map[entity2.id() as usize]);
           assert_eq!(None, store.entity_map[entity3.id() as usize]);

           assert_eq!(entity1_value, store.components[0]);
           assert_eq!(entity4_value, store.components[1]);

           assert_eq!(Some((entity1, 0)), store.entity_map[entity1.id() as usize]);
           assert_eq!(Some((entity4, 1)), store.entity_map[entity4.id() as usize]);
           assert_eq!(2, store.active_components)
       }
    */
    #[test]
    fn component_destroy() {
        let max_components = 5;
        let mut store: ComponentStore<u8> = ComponentStore::new(max_components);

        let entity1 = Entity::new(10, 0);
        let entity1_value = 123;
        match store.add(entity1) {
            Ok(e) => {
                *e = entity1_value;
            }
            _ => todo!(),
        }

        let entity2 = Entity::new(12, 0);
        let entity2_value = 234;
        match store.add(entity2) {
            Ok(e) => {
                *e = entity2_value;
            }
            _ => todo!(),
        }

        store.destroy(entity1);
        assert_eq!(1, store.active_components);

        // Make sure it was destroyed properly
        assert_eq!(None, store.entity_map[entity1.id() as usize]);

        // Make sure it was shifted so last active is new active
        assert_eq!(entity2_value, store.components[0]);
        assert_eq!(entity1_value, store.components[1]);

        // Make sure last entity is now pointing to the removed one
        assert_eq!(Some((entity2, 0)), store.entity_map[entity2.id() as usize]);

        // Some random chaos testing
        store.destroy(entity1);
        store.destroy(entity2);
        store.destroy(Entity::new(155, 0));
        assert_eq!(0, store.active_components);
    }

    #[test]
    fn component_get_mut() {
        let max_components = 5;
        let mut store: ComponentStore<u8> = ComponentStore::new(max_components);

        let entity = Entity::new(12, 0);
        match store.add(entity) {
            Ok(e) => {
                *e = 123;
            }
            _ => todo!(),
        }

        let component = store.add(entity);
        match store.get_mut(entity) {
            Some(e) => {
                *e = 234;
            }
            None => {
                todo!()
            }
        }

        match store.get(entity) {
            Some(e) => {
                assert_eq!(234, *e);
            }
            None => {
                todo!()
            }
        }

        let entity = Entity::new(14, 0);
        assert_eq!(None, store.get_mut(entity));
    }

    #[test]
    fn component_get() {
        let max_components = 5;
        let mut store: ComponentStore<u8> = ComponentStore::new(max_components);

        let entity = Entity::new(12, 0);
        match store.add(entity) {
            Ok(e) => {
                *e = 123;
            }
            _ => todo!(),
        }

        let component = store.add(entity);
        match store.get(entity) {
            Some(e) => {
                assert_eq!(123, *e);
            }
            None => {
                todo!()
            }
        }

        let entity = Entity::new(14, 0);
        assert_eq!(None, store.get(entity));
    }

    #[test]
    fn component_index() {
        let max_components = 5;
        let mut store: ComponentStore<u8> = ComponentStore::new(max_components);

        let entity = Entity::new(12, 0);
        let component = store.add(entity);
        assert_eq!(Some(0), store.component_index(entity));

        let entity = Entity::new(69, 0);
        let component = store.add(entity);
        assert_eq!(Some(1), store.component_index(entity));
    }

    #[test]
    fn add() {
        let max_components = 2;
        let mut store: ComponentStore<u8> = ComponentStore::new(max_components);

        // Normal
        {
            let entity = Entity::new(12, 0);
            let component = store.add(entity);
            assert_eq!(true, component.is_ok());
            assert_eq!(Some((entity, 0)), store.entity_map[entity.id() as usize]);

            let entity = Entity::new(13, 0);
            let component = store.add(entity);
            assert_eq!(true, component.is_ok());
            assert_eq!(Some((entity, 1)), store.entity_map[entity.id() as usize]);
        }

        // Would overflow
        let entity = Entity::new(15, 0);
        let component = store.add(entity);
        assert_eq!(true, component.is_err());
        assert_eq!(ComponentStoreError::BufferOverflow, component.unwrap_err());
    }
}
