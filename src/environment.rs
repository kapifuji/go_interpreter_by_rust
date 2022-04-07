use crate::object;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, object::Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn Get(&self, name: String) -> Option<object::Object> {
        match self.store.get(&name) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn Set(&mut self, name: String, value: object::Object) {
        self.store.insert(name, value);
    }
}
