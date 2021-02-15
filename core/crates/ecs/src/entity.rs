// Based on http://bitsquid.blogspot.com/2014/08/building-data-oriented-entity-system.html

pub type EntityId = u32;
pub type Generation = u8;

pub const MAX_ENTITIES: usize = (ENTITY_INDEX_MASK >> ENTITY_GENERATION_BITS) as usize;

const ENTITY_GENERATION_BITS: usize = 8;
const ENTITY_INDEX_BITS: usize = 32 - ENTITY_GENERATION_BITS;

const ENTITY_INDEX_MASK: EntityId =
    EntityId::MAX >> ENTITY_GENERATION_BITS << ENTITY_GENERATION_BITS;
const ENTITY_GENERATION_MASK: EntityId = !ENTITY_INDEX_MASK;

/// An entity in the system.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Entity {
    entity: EntityId,
}

impl Entity {
    /// Creates a new entity.
    pub fn new(id: EntityId, generation: Generation) -> Self {
        let generation = generation as EntityId;
        let id = id << ENTITY_GENERATION_BITS;

        Self {
            entity: id | generation,
        }
    }

    /// Returns the id of the entity
    pub fn id(&self) -> EntityId {
        self.entity & ENTITY_INDEX_MASK
    }

    /// Returns the generation of the entity
    pub fn generation(&self) -> Generation {
        (self.entity & ENTITY_GENERATION_MASK) as Generation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_new_returns_expected() {
        // Normal case
        let entity = Entity::new(0b1010, 0b1110);
        assert_eq!(0b0000_0000_0000_0000_0000_1010_0000_1110, entity.entity);

        let entity = Entity::new(0b1110, 0b0);
        assert_eq!(0b0000_0000_0000_0000_0000_1110_0000_0000, entity.entity);

        // Id exceeds max entities, truncates
        let entity = Entity::new(0b1111_1111_1111_1111_1111_1111_1111_1111, 0b0);
        assert_eq!(0b1111_1111_1111_1111_1111_1111_0000_0000, entity.entity);
    }

    #[test]
    fn entity_generation_returns_expected() {
        let e = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let entity = Entity { entity: e };

        assert_eq!(0b1111_1111, entity.generation());

        let e = 0b1111_0000_1111_1111_1111_1111_1011_1101;
        let entity = Entity { entity: e };

        assert_eq!(0b1011_1101, entity.generation());
    }

    #[test]
    fn entity_id_returns_expected() {
        let e = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let entity = Entity { entity: e };

        assert_eq!(0b1111_1111_1111_1111_1111_1111_0000_0000, entity.id());

        let e = 0b1001_1111_1001_1100_1111_1111_1111_1111;
        let entity = Entity { entity: e };

        assert_eq!(0b1001_1111_1001_1100_1111_1111_0000_0000, entity.id());
    }

    #[test]
    fn entity_index_bits() {
        assert_eq!(24, ENTITY_INDEX_BITS);
    }

    #[test]
    fn entity_generation_bits() {
        assert_eq!(8, ENTITY_GENERATION_BITS);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
