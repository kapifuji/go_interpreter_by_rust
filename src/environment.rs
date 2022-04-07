use crate::object;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, object::Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn create_enclosed_environment(outer: Environment) -> Environment {
        let mut environment = Environment::new();
        environment.outer = Some(Box::new(outer));
        environment
    }

    pub fn get(&self, name: String) -> Option<object::Object> {
        match self.store.get(&name) {
            Some(value) => Some(value.clone()),
            None => {
                if let Some(outer) = &self.outer {
                    outer.get(name)
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, name: String, value: object::Object) {
        self.store.insert(name, value);
    }
}
