use std::any::TypeId;

use crate::storage::Component;

pub struct Query {
    components: Vec<TypeId>,
}

impl Query {
    pub fn builder() -> QueryBuilder {
        QueryBuilder::default()
    }

    pub fn components(&self) -> &Vec<TypeId> {
        &self.components
    }
}

#[derive(Default)]
pub struct QueryBuilder {
    components: Vec<TypeId>,
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        Self {
            components: Vec::new(),
        }
    }

    pub fn with<T: Component>(mut self) -> QueryBuilder {
        self.components.push(TypeId::of::<T>());
        self
    }

    pub fn build(self) -> Query {
        Query {
            components: self.components,
        }
    }
}
