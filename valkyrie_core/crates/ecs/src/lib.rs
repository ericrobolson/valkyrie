// Based on http://bitsquid.blogspot.com/2014/08/building-data-oriented-entity-system.html

mod component_store;
mod entity;
mod entity_manager;

pub use component_store::{ComponentStore, ComponentStoreError};
pub use entity::Entity;
pub use entity_manager::EntityManager;
