use crate::{location::LocationMap, Component, Storage};
use std::any::TypeId;

use crate::{
    archetype::{Archetype, Entity},
    ComponentStorages,
};

#[derive(Debug)]
pub struct World {
    entity_id: u32,
    arche_id: u32,
    pub locations: LocationMap,
    pub archetype: Vec<Archetype>,
    pub components: ComponentStorages,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_id: 0,
            arche_id: 0,
            locations: LocationMap::new(),
            archetype: Vec::new(),
            components: ComponentStorages::new(),
        }
    }

    pub fn spawn<C: Component>(&mut self, component: C) {
        let component_type_id = TypeId::of::<C>();

        let entity = Entity(self.entity_id);

        let mut archetypes: Vec<&mut Archetype> = self
            .archetype
            .iter_mut()
            .filter(|archetype| archetype.layout().containes_type_id(component_type_id))
            .collect();

        if archetypes.is_empty() {
            let archetype_index = self.arche_id;

            let new_archetype = Archetype::new(archetype_index, vec![entity.id()]);
            new_archetype.layout().register_component::<C>();

            self.archetype.push(new_archetype);
            self.arche_id += 1;
        } else {
            let archetype = archetypes.get_mut(0).unwrap();
            archetype.assigne_entity(&entity);
        }

        // Get component type specific storage
        let storage = self.components.get_storage_mut::<C>();

        // Push new component into storage
        let component_index = storage.push_component(component);

        // Insert the component index from storage into location map
        self.locations.insert(entity, vec![component_index]);

        self.entity_id += 1;
    }

    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();

        let components_indicies = self.locations.get(&entity);

        let archetype: Vec<&Archetype> = self
            .archetype
            .iter()
            .filter(|archetype| archetype.has_entity(entity))
            .collect();

        unimplemented!("NOT IMPLEMENTED")
    }
}
