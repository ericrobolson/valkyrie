// Based on http://bitsquid.blogspot.com/2014/09/building-data-oriented-entity-system.html
use crate::{
    entity::{EntityId, Generation, MAX_ENTITIES},
    Entity, EntityManager,
};

pub enum ComponentStoreError {
    BufferOverflow,
}

pub struct ComponentStore<Component>
where
    Component: Default + Clone,
{
    entity_map: Vec<Option<EntityId>>,
    components: Vec<Component>,
    active_components: usize,
}

impl<Component> ComponentStore<Component>
where
    Component: Default + Clone,
{
    /// Creates a new component store.
    pub fn new(component_capacity: usize) -> Self {
        Self {
            entity_map: vec![None; MAX_ENTITIES],
            components: vec![Component::default(); component_capacity],
            active_components: 0,
        }
    }

    fn get_component_index(&self, entity: Entity) -> Option<usize> {
        todo!()
    }

    pub fn get(&self, entity: Entity) -> Option<&Component> {
        todo!()
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut Component> {
        todo!()
    }

    pub fn add(&mut self, entity: Entity) -> Result<&mut Component, ComponentStoreError> {
        todo!()
    }

    pub fn components(&self) -> &[Component] {
        &self.components[0..self.active_components]
    }

    /// Removes all entities that aren't alive
    pub fn garbage_collection(&mut self, entity_manager: &EntityManager) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_store_new() {
        let max_components = 129;
        let store: ComponentStore<u8> = ComponentStore::new(max_components);
        assert_eq!(0, store.active_components);
        assert_eq!(max_components, store.components.capacity());
        assert_eq!(vec![u8::default(); max_components], store.components);
        assert_eq!(vec![None; MAX_ENTITIES], store.entity_map);
    }

    #[test]
    fn component_store_add() {
        let max_components = 129;
        let mut store: ComponentStore<u8> = ComponentStore::new(max_components);

        // Normal
        {
            let entity = Entity::new(12, 0);
            let component = store.add(entity);
            assert_eq!(true, component.is_ok());

            assert_eq!()
        }
    }
}
