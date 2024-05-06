use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    archetype::{Archetype, Entity},
    location::EntityLocation,
    storage::{Component, Storage},
};

pub struct Entry<'a> {
    entity: &'a Entity,
    archetype: &'a Archetype,
    components: HashMap<TypeId, &'a Box<dyn Any>>,
    locations: &'a [EntityLocation],
}

impl<'a> Entry<'a> {
    pub fn new(
        entity: &'a Entity,
        archetype: &'a Archetype,
        components: HashMap<TypeId, &'a Box<dyn Any>>,
        locations: &'a [EntityLocation],
    ) -> Self {
        Self {
            entity,
            archetype,
            components,
            locations,
        }
    }

    pub fn entity(&self) -> &'a Entity {
        self.entity
    }

    pub fn archetype(&self) -> &Archetype {
        self.archetype
    }

    pub fn get_component<C: Component>(&mut self) -> Option<&'a C> {
        let type_id = TypeId::of::<C>();

        let index = self
            .archetype
            .layout()
            .clone()
            .into_iter()
            .position(|component_id| component_id == type_id)
            .unwrap();

        let location = &self.locations[index];

        let unknown_storage = *self.components.get_mut(&type_id).unwrap();

        if let Some(storage) = unknown_storage.downcast_ref::<C::Storage>() {
            Some(storage.get_component(location.component()))
        } else {
            None
        }
    }
}
