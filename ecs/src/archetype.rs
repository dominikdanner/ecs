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

#[derive(Debug, Clone)]
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

    pub fn contains_entity(&self, entity: &Entity) -> bool {
        self.entitys.contains(&entity.0)
    }

    pub fn index(&self) -> ArchetypeIndex {
        self.index
    }
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
pub struct ArchetypeManager {
    // Provides unique indecies for the array
    ids: ArchetypeIndex,
    archetypes: Vec<Archetype>,
}

impl ArchetypeManager {
    pub fn new() -> Self {
        Self {
            ids: 0,
            archetypes: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.archetypes.len()
    }

    // Adds new archetype and returns its archetype index
    pub fn add(&mut self, layout: EntityLayout) -> &mut Archetype {
        let index = self.ids;
        let archetype = Archetype::new(index, layout);

        self.archetypes.push(archetype);
        self.ids += 1;

        self.get_mut(index)
    }

    pub fn get(&self, index: ArchetypeIndex) -> &Archetype {
        self.archetypes.get(index as usize).unwrap()
    }

    pub fn get_mut(&mut self, index: ArchetypeIndex) -> &mut Archetype {
        self.archetypes.get_mut(index as usize).unwrap()
    }

    // Find an archetype that has the same layout as provided and returns a reference to it
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

    // Find an archetype that has the same layout as provided and returns a mutable reference to it
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

    // Find a an entitys archetype and returns a reference to it
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

    // Find a an entitys archetype and returns a mutable reference to it
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
