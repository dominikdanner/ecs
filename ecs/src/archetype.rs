use std::any::TypeId;

use crate::Component;

#[derive(Debug)]
pub struct Entity(pub u32);

impl Entity {
    pub fn id(&self) -> u32 {
        self.0
    }
}

pub type ArchetypeIndex = u32;

#[derive(Debug)]
pub struct Archetype {
    index: ArchetypeIndex,
    entitys: Vec<u32>,
    pub layout: EntityLayout,
}

impl Archetype {
    pub fn new(index: u32, entitys: Vec<u32>) -> Self {
        Self {
            index,
            entitys,
            layout: EntityLayout::new(),
        }
    }

    pub fn assigne_entity(&mut self, entity: &Entity) {
        self.entitys.push(entity.id())
    }

    pub fn index(&self) -> ArchetypeIndex {
        self.index
    }
}

#[derive(Debug)]
pub struct EntityLayout {
    components: Vec<TypeId>,
}

impl EntityLayout {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn register_component<T>(&mut self)
    where
        T: Component,
    {
        let type_id = TypeId::of::<T>();
        self.components.push(type_id)
    }

    pub fn containes_type_id(&self, type_id: TypeId) -> bool {
        self.components.contains(&type_id)
    }
}

impl PartialEq for EntityLayout {
    fn eq(&self, other: &Self) -> bool {
        self.components.eq(&other.components)
    }
}
