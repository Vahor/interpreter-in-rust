use std::collections::HashMap;
use object::object::ObjectType;

#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, ObjectType>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            store: HashMap::new(),
        };
    }

    pub fn get(&self, name: &str) -> Option<&ObjectType> {
        return self.store.get(name);
    }

    pub fn set(&mut self, name: &str, value: ObjectType) {
        self.store.insert(name.to_string(), value);
    }
}