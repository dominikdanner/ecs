use std::any::TypeId;

use crate::storage::Component;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Entity(pub u32);

impl Entity {
    pub fn id(&self) -> u32 {
        self.0
    }
}

pub type ArchetypeIndex = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct Archetype {
    index: ArchetypeIndex,
    entitys: Vec<u32>,
    pub layout: EntityLayout,
}

impl Archetype {
    pub fn new(index: ArchetypeIndex, layout: EntityLayout) -> Self {
        Self {
            index,
            entitys: Vec::new(),
            layout,
        }
    }

    pub fn layout(&self) -> &EntityLayout {
        &self.layout
    }

    pub fn assigne_entity(&mut self, entity: &Entity) {
        self.entitys.push(entity.id())
    }

    pub fn unassigne_entity(&mut self, target_entity: &Entity) {
        let index = self
            .entitys
            .iter()
            .position(|entity_id| entity_id == &target_entity.id())
            .unwrap();

        self.entitys.remove(index);
    }

    pub fn contains_entity(&self, entity: &Entity) -> bool {
        self.entitys.contains(&entity.0)
    }

    pub fn index(&self) -> ArchetypeIndex {
        self.index
    }
}

pub trait LayoutFilter {
    fn matches_layout(&self, other: EntityLayout);
}

#[derive(Debug, Clone)]
pub struct EntityLayout {
    layout: Vec<TypeId>,
}

impl EntityLayout {
    pub fn new() -> Self {
        Self { layout: Vec::new() }
    }

    pub fn register_component<T>(&mut self)
    where
        T: Component,
    {
        let type_id = TypeId::of::<T>();
        self.layout.push(type_id)
    }

    pub fn containes_type(&self, type_id: TypeId) -> bool {
        self.layout.contains(&type_id)
    }
}

impl IntoIterator for EntityLayout {
    type Item = TypeId;
    type IntoIter = <Vec<TypeId> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.layout.into_iter()
    }
}

impl PartialEq for EntityLayout {
    fn eq(&self, other: &Self) -> bool {
        self.layout.eq(&other.layout)
    }
}

#[derive(Debug)]
pub struct ArchetypeStorage {
    // Provides unique indecies for every archetype
    ids: ArchetypeIndex,
    archetypes: Vec<Archetype>,
}

impl ArchetypeStorage {
    pub fn new() -> Self {
        Self {
            ids: 0,
            archetypes: Vec::new(),
        }
    }

    /// Returns how much archetypes exist
    pub fn len(&self) -> usize {
        self.archetypes.len()
    }

    /// Adds new archetype from its layout and returns a mutable reference to it
    pub fn create_from_layout(&mut self, layout: EntityLayout) -> &mut Archetype {
        let index = self.ids;
        let archetype = Archetype::new(index, layout);

        self.archetypes.push(archetype);
        self.ids += 1;

        self.get_mut(index)
    }

    /// Get archetype from index as a reference
    pub fn get(&self, index: ArchetypeIndex) -> &Archetype {
        self.archetypes.get(index as usize).unwrap()
    }

    /// Get archetype from index mutable
    pub fn get_mut(&mut self, index: ArchetypeIndex) -> &mut Archetype {
        self.archetypes.get_mut(index as usize).unwrap()
    }

    /// Find an archetype that has the same layout as provided and returns a reference to it
    pub fn find_from_layout(&self, layout: &EntityLayout) -> Option<&Archetype> {
        // Checks every Archetype and compairs it to the provided layout
        let archetypes: Vec<&Archetype> = self
            .archetypes
            .iter()
            .filter(|archetype| archetype.layout == *layout)
            .collect();

        if archetypes.is_empty() {
            None
        } else {
            // There should be only 1 archetype that matches the layout
            assert_eq!(archetypes.len(), 1);

            let archetype = *archetypes.first().unwrap();

            Some(archetype)
        }
    }

    /// Find an archetype that has the same layout as provided and returns a mutable reference to it
    pub fn find_from_layout_mut(&mut self, layout: &EntityLayout) -> Option<&mut Archetype> {
        // Checks every Archetype and compairs it to the provided layout
        let archetypes: Vec<&mut Archetype> = self
            .archetypes
            .iter_mut()
            .filter(|archetype| archetype.layout == *layout)
            .collect();

        if archetypes.is_empty() {
            None
        } else {
            // There should be only 1 archetype that matches the layout
            assert_eq!(archetypes.len(), 1);

            let index = archetypes.get(0).unwrap().index;
            let archetype = self.archetypes.get_mut(index as usize).unwrap();

            Some(archetype)
        }
    }

    /// Find a an entitys archetype and returns a reference to it
    pub fn find_from_entity(&self, entity: &Entity) -> Option<&Archetype> {
        let archetypes: Vec<&Archetype> = self
            .archetypes
            .iter()
            .filter(|archetype| archetype.contains_entity(entity))
            .collect();

        if archetypes.is_empty() {
            None
        } else {
            // An Entity can't be assigned to two archetypes
            assert_eq!(archetypes.len(), 1);

            let archetype = *archetypes.first().unwrap();
            Some(archetype)
        }
    }

    /// Find a an entitys archetype and returns a mutable reference to it
    pub fn find_from_entity_mut(&mut self, entity: &Entity) -> Option<&mut Archetype> {
        let archetypes: Vec<&mut Archetype> = self
            .archetypes
            .iter_mut()
            .filter(|archetype| archetype.contains_entity(entity))
            .collect();

        if archetypes.is_empty() {
            None
        } else {
            // An Entity can't be assigned to two archetypes
            assert_eq!(archetypes.len(), 1);

            let index = archetypes.get(0).unwrap().index;
            let archetype = self.archetypes.get_mut(index as usize).unwrap();
            Some(archetype)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use crate::archetype::{Archetype, Entity};

    use super::{ArchetypeStorage, EntityLayout};

    #[test]
    fn archetype_create_from_layout() {
        let mut archetype_manager = ArchetypeStorage::new();

        let layout = EntityLayout::new();
        archetype_manager.create_from_layout(layout);

        let length = archetype_manager.len();
        assert_eq!(length, 1);
    }

    #[test]
    fn archetype_find_from_layout() {
        let mut archetype_manager = ArchetypeStorage::new();

        let layout = EntityLayout::new();
        archetype_manager.create_from_layout(layout.clone());
        let archetype = archetype_manager.find_from_layout(&layout).unwrap();

        assert_eq!(
            archetype,
            Archetype {
                layout,
                index: 0,
                entitys: vec![]
            }
            .borrow_mut()
        )
    }

    #[test]
    fn archetype_find_from_entity() {
        let mut archetype_manager = ArchetypeStorage::new();
        let entity = Entity(0);

        let layout = EntityLayout::new();
        let new_archetype = archetype_manager.create_from_layout(layout.clone());
        new_archetype.assigne_entity(&entity);

        let archetype = archetype_manager.find_from_entity(&entity).unwrap();

        assert_eq!(
            archetype,
            Archetype {
                layout,
                index: 0,
                entitys: vec![entity.id()]
            }
            .borrow_mut()
        )
    }
}
