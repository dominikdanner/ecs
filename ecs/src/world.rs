use std::collections::HashMap;

use crate::{
    archetype::{ArchetypeStorage, EntityLayout},
    entry::Entry,
    location::EntityLocation,
    storage::Storage,
};

use crate::{
    archetype::Entity,
    location::LocationMap,
    storage::{Component, ComponentStorages},
};

#[derive(Debug)]
pub struct World {
    entity_id: u32,
    pub locations: LocationMap,
    pub archetypes: ArchetypeStorage,
    pub components: ComponentStorages,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_id: 0,
            locations: LocationMap::new(),
            archetypes: ArchetypeStorage::new(),
            components: ComponentStorages::new(),
        }
    }

    pub fn entry<'a>(&'a mut self, entity: &'a Entity) -> Entry<'a> {
        let archetype = self.archetypes.find_from_entity(entity).unwrap();

        let mut components = HashMap::new();
        for component_id in archetype.layout().clone().into_iter() {
            let unknown_storage = self.components.get_storage_raw(component_id);
            components.insert(component_id, unknown_storage);
        }

        let locations = self.locations.get(entity);
        Entry::new(entity, archetype, components, locations)
    }

    // Creates new enity and adds one component to it
    pub fn spawn<C: Component>(&mut self, component: C) -> Entity {
        let entity = Entity(self.entity_id);

        let mut layout = EntityLayout::new();
        layout.register_component::<C>();

        // If there is no archetype with that specific layout there is a new one created
        let archetype = match self.archetypes.find_from_layout_mut(&layout) {
            Some(archetype) => {
                archetype.assigne_entity(&entity);
                archetype
            }
            None => {
                let archetype = self.archetypes.create_from_layout(layout);
                archetype.assigne_entity(&entity);
                archetype
            }
        };

        // Get component type specific storage
        let storage = self.components.get_storage_mut::<C>();

        // Push new component into storage
        let component_index = storage.push_component(component);

        let location = EntityLocation::new(archetype.index(), component_index);
        // Insert the component index from storage into location map
        self.locations.insert(entity, vec![location]);

        self.entity_id += 1;

        entity.clone()
    }

    /// Extend an enity's compnents by one
    pub fn extend<C: Component>(&mut self, entity: &Entity, component: C) {
        let component_locations = self.locations.get_mut(entity);

        // Get archetype that the entity is assigned to
        let current_archetype = self
            .archetypes
            .find_from_entity_mut(entity)
            .expect("Entity has no archetype!");

        // Create new layout for entity
        let mut new_layout = current_archetype.layout().clone();
        new_layout.register_component::<C>();

        current_archetype.unassigne_entity(entity);

        let archetype = match self.archetypes.find_from_layout_mut(&new_layout) {
            Some(archetype) => {
                archetype.assigne_entity(&entity);
                archetype
            }
            None => {
                let archetype = self.archetypes.create_from_layout(new_layout);
                archetype.assigne_entity(&entity);
                archetype
            }
        };

        let storage = self.components.get_storage_mut::<C>();
        let storage_index = storage.push_component(component);

        let location = EntityLocation::new(archetype.index(), storage_index);
        component_locations.push(location);
    }

    // Needs rewrite hihahuuuu
    pub fn query<C: Component>(&mut self) -> &[C] {
        let storage = self.components.get_storage::<C>();
        storage.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::{Component, VecStorage};

    use super::World;

    #[derive(Debug, PartialEq)]
    struct Health(f32);

    impl Component for Health {
        type Storage = VecStorage<Self>;
    }

    #[derive(Debug, PartialEq)]
    struct Stamina(f32);

    impl Component for Stamina {
        type Storage = VecStorage<Self>;
    }

    #[test]
    fn spawn_entity_with_single_compenent() {
        let mut world = World::new();

        let entity = world.spawn(Health(200.00));

        assert_eq!(
            *world.entry(&entity).get_component::<Health>().unwrap(),
            Health(200.00)
        );
    }

    #[test]
    fn spawn_entitys_check_archetypes() {
        let mut world = World::new();

        world.spawn(Health(100.00));
        world.spawn(Stamina(300.00));
        world.spawn(Health(100.00));
        world.spawn(Stamina(300.00));

        assert_eq!(world.archetypes.len(), 2);
    }
}
