use crate::{archetype::EntityLayout, Component, Storage};
use std::any::TypeId;

use crate::{
    archetype::{Archetype, Entity},
    ComponentStorages,
};

#[derive(Debug)]
pub struct World {
    entity_id: u32,
    arche_id: u32,
    pub archetype: Vec<Archetype>,
    pub components: ComponentStorages,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_id: 0,
            arche_id: 0,
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
            .filter(|archetype| archetype.layout.containes_type_id(component_type_id))
            .collect();

        if archetypes.is_empty() {
            let archetype_index = self.arche_id;

            let mut new_archetype = Archetype::new(archetype_index, vec![entity.id()]);
            new_archetype.layout.register_component::<C>();

            self.archetype.push(new_archetype);
            self.arche_id += 1;
        } else {
            let archetype = archetypes.get_mut(0).unwrap();
            archetype.assigne_entity(&entity);
        }

        let storage = self.components.get_storage_mut::<C>();
        storage.add(component);

        self.entity_id += 1;
    }
}
