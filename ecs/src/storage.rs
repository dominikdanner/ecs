use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

// A components specific index into its type storage
pub type ComponentIndex = usize;

// Defines a Component
pub trait Component: Debug + Sized + Any {
    // The Type of Storage this Component uses
    type Storage: Storage<Self>;
}

// Defines a Storage that can store a single component
pub trait Storage<T: Debug>: Debug + Any {
    // creates new instance of storage
    fn new() -> Self
    where
        Self: Sized;

    // pushes new component into the storage
    fn push_component(&mut self, component: T) -> ComponentIndex;

    // returns the component with a given index
    fn get_component(&self, index: ComponentIndex) -> &T;

    // Returns all components stored in the component as a slice of T
    fn as_slice(&self) -> &[T];

    // returns how much components are stored in a storage
    fn size(&self) -> usize;
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

    /// Creates a new component storage
    pub fn create_storage<C: Component>(&mut self) {
        let component_type_id = TypeId::of::<C>();
        let storage = C::Storage::new();

        self.storages.insert(component_type_id, Box::from(storage));
    }

    // Gives back a reference to the components storage
    /// If storage of component does not exist it will be created automatically
    pub fn get_storage<C: Component>(&mut self) -> &C::Storage {
        let type_id = TypeId::of::<C>();

        if !self.storages.contains_key(&type_id) {
            self.create_storage::<C>();
        }

        match self.storages.get(&type_id) {
            Some(unknown_storage) => {
                match unknown_storage.downcast_ref::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!("We're fucked"),
                }
            }
            None => unreachable!("We're fucked"),
        }
    }

    /// Gives back a mutable reference to the components storage
    /// If storage of component does not exist it will be created automatically
    pub fn get_storage_mut<C: Component>(&mut self) -> &mut <C as Component>::Storage {
        let type_id = TypeId::of::<C>();

        if !self.storages.contains_key(&type_id) {
            self.create_storage::<C>()
        }

        match self.storages.get_mut(&type_id) {
            Some(unknown_storage) => {
                match unknown_storage.downcast_mut::<<C as Component>::Storage>() {
                    Some(storage) => storage,
                    None => unreachable!("We're fucked"),
                }
            }
            None => unreachable!("We're fucked"),
        }
    }

    /// Gets storage data but type is not known
    pub fn get_storage_raw(&self, type_id: TypeId) -> &Box<dyn Any> {
        match self.storages.get(&type_id) {
            Some(unknown_storage) => unknown_storage,
            None => unreachable!("We're fucked"),
        }
    }
}

// Actual implementation of a storage based on a vector
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

    fn push_component(&mut self, component: T) -> ComponentIndex {
        let component_index = self.size();
        self.storage.push(component);

        component_index
    }

    fn size(&self) -> usize {
        self.storage.len()
    }

    fn get_component(&self, index: ComponentIndex) -> &T {
        self.storage.get(index).unwrap()
    }

    fn as_slice(&self) -> &[T] {
        self.storage.as_slice()
    }
}
