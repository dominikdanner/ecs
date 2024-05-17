use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    archetype::{Archetype, ArchetypeStorage, Entity},
    location::{EntityLocation, LocationMap},
    storage::{Component, ComponentStorages, Storage},
};

pub struct EntryRef<'a> {
    entity: &'a Entity,
    archetype: &'a Archetype,
    components: HashMap<TypeId, &'a Box<dyn Any>>,
    locations: &'a [EntityLocation],
}

impl<'a> EntryRef<'a> {
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

    pub fn get_component<C: Component>(&self) -> Option<&'a C> {
        let type_id = TypeId::of::<C>();

        let index = self
            .archetype
            .layout()
            .clone()
            .into_iter()
            .position(|component_id| component_id == type_id)
            .unwrap();

        let location = &self.locations[index];

        let unknown_storage = *self.components.get(&type_id).unwrap();

        if let Some(storage) = unknown_storage.downcast_ref::<C::Storage>() {
            Some(storage.get_component(location.component()).unwrap())
        } else {
            None
        }
    }
}

pub struct EntryMut<'a> {
    entity: &'a Entity,
    archetypes: &'a mut ArchetypeStorage,
    components: &'a mut ComponentStorages,
    locations: &'a mut LocationMap,
}

impl<'a> EntryMut<'a> {
    pub fn new(
        entity: &'a Entity,
        archetypes: &'a mut ArchetypeStorage,
        components: &'a mut ComponentStorages,
        locations: &'a mut LocationMap,
    ) -> Self {
        Self {
            entity,
            archetypes,
            components,
            locations,
        }
    }

    pub fn entity(&self) -> &'a Entity {
        self.entity
    }

    pub fn archetype(&self) -> &Archetype {
        self.archetypes.find_from_entity(self.entity).unwrap()
    }

    // Trys to get Component `C` from entity
    pub fn get_component<C: Component>(&mut self) -> Option<&C> {
        let entity = self.entity();
        let archetype = self.archetype();

        let type_id = TypeId::of::<C>();
        let index = archetype
            .layout()
            .clone()
            .into_iter()
            .position(|component_id| component_id == type_id)
            .unwrap();

        let location = &self.locations.get(entity)[index];
        let storage = self.components.get_storage::<C>();

        storage.get_component(location.component())
    }

    // Adds a component to an entity
    pub fn add_component<C: Component>(&mut self, component: C) {
        let component_locations = self.locations.get_mut(self.entity);

        // Get archetype that the entity is assigned to
        let current_archetype = self
            .archetypes
            .find_from_entity_mut(self.entity)
            .expect("Entity has no archetype!");

        // Create new layout for entity
        let mut new_layout = current_archetype.layout().clone();
        new_layout.register_component::<C>();

        current_archetype.unassigne_entity(self.entity);

        let archetype = match self.archetypes.find_from_layout_mut(&new_layout) {
            Some(archetype) => {
                archetype.assigne_entity(self.entity);
                archetype
            }
            None => {
                let archetype = self.archetypes.create_from_layout(new_layout);
                archetype.assigne_entity(self.entity);
                archetype
            }
        };

        let storage = self.components.get_storage_mut::<C>();
        let storage_index = storage.push_component(component);

        let location = EntityLocation::new(archetype.index(), storage_index);
        component_locations.push(location);
    }
}
