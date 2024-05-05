use std::collections::HashMap;

use crate::{archetype::Entity, storage::ComponentIndex};

// Stores the Storage Location of an entity's data
#[derive(Debug)]
pub struct LocationMap {
    locations: HashMap<Entity, Vec<ComponentIndex>>,
}

impl LocationMap {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
        }
    }

    pub fn get(&self, entity: &Entity) -> &[ComponentIndex] {
        self.locations.get(&entity).unwrap()
    }

    pub fn get_mut(&mut self, entity: &Entity) -> &mut Vec<ComponentIndex> {
        self.locations.get_mut(&entity).unwrap()
    }

    pub fn insert(&mut self, entity: Entity, component_indecies: Vec<ComponentIndex>) {
        self.locations.insert(entity, component_indecies);
    }
}
