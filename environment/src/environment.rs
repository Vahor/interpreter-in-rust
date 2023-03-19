use std::collections::HashMap;
use crate::object::ObjectType;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, ObjectType>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new(),
            outer: None,
        };
    }

    pub fn new_enclosed(outer: &Environment) -> Environment {
        return Environment {
            store: HashMap::new(),
            outer: Some(Box::new(outer.clone())), // TODO: sad clone
        };
    }

    pub fn get(&self, name: &str) -> Option<&ObjectType> {
        let value = self.store.get(name);
        if value.is_some() {
            return value;
        }
        if let Some(outer) = &self.outer {
            return outer.get(name);
        }
        return None;
    }

    pub fn set(&mut self, name: &str, value: ObjectType) {
        self.store.insert(name.to_string(), value.clone());
    }

    pub fn merge(&mut self, other: &Environment) {
        for (key, value) in &other.store {
            self.store.insert(key.to_string(), value.clone());
        }
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }
}