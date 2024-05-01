use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

pub mod archetype;
pub mod world;

// Defines a Component
pub trait Component: Debug + Sized + Any {
    // The Type of Storage this Component uses
    type Storage: Storage<Self>;
}

// Defines a Storage that can store a single component
pub trait Storage<T: Debug>: Debug + Any {
    fn new() -> Self
    where
        Self: Sized;

    fn add(&mut self, component: T);
}

// Holds all the storages for every single component
#[derive(Debug)]
pub struct ComponentStorages {
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStorages {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }

    pub fn get_storage<C: Component>(&mut self) -> &C::Storage {
        let type_id = TypeId::of::<C>();

        if !self.storages.contains_key(&type_id) {
            let storage = C::Storage::new();

            self.storages.insert(type_id, Box::from(storage));
        }

        match self.storages.get(&type_id) {
            Some(unknown_storage) => {
                match unknown_storage.downcast_ref::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(""),
                }
            }
            None => unreachable!(""),
        }
    }

    pub fn get_storage_mut<C: Component>(&mut self) -> &mut <C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        if !self.storages.contains_key(&type_id) {
            let storage = C::Storage::new();

            self.storages.insert(type_id, Box::from(storage));
        }

        match self.storages.get_mut(&type_id) {
            Some(unknown_storage) => {
                match unknown_storage.downcast_mut::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!(""),
                }
            }
            None => unreachable!(""),
        }
    }
}

#[derive(Debug)]
pub struct VecStorage<T: Component> {
    storage: Vec<T>,
}

impl<T> Storage<T> for VecStorage<T>
where
    T: Component,
{
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            storage: Vec::new(),
        }
    }

    fn add(&mut self, component: T) {
        self.storage.push(component);
    }
}
