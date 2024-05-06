use std::collections::HashMap;

use crate::{
    archetype::{ArchetypeIndex, Entity},
    storage::ComponentIndex,
};

#[derive(Debug)]
pub struct EntityLocation(pub ArchetypeIndex, pub ComponentIndex);

impl EntityLocation {
    pub fn new(archetype_index: ArchetypeIndex, component_index: ComponentIndex) -> Self {
        Self(archetype_index, component_index)
    }

    pub fn archetype(&self) -> ArchetypeIndex {
        self.0
    }

    pub fn component(&self) -> ComponentIndex {
        self.1
    }
}

// Stores the Storage Location of an entity's data
#[derive(Debug)]
pub struct LocationMap {
    locations: HashMap<Entity, Vec<EntityLocation>>,
}

impl LocationMap {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
        }
    }

    pub fn get(&self, entity: &Entity) -> &[EntityLocation] {
        self.locations.get(&entity).unwrap()
    }

    pub fn get_mut(&mut self, entity: &Entity) -> &mut Vec<EntityLocation> {
        self.locations.get_mut(&entity).unwrap()
    }

    pub fn insert(&mut self, entity: Entity, component_indecies: Vec<EntityLocation>) {
        self.locations.insert(entity, component_indecies);
    }
}
