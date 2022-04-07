use crate::object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, object::Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn create_enclosed_environment(outer: Rc<RefCell<Environment>>) -> Environment {
        let mut environment = Environment::new();
        environment.outer = Some(outer);
        environment
    }

    pub fn get(&self, name: String) -> Option<object::Object> {
        match self.store.get(&name) {
            Some(value) => Some(value.clone()),
            None => {
                if let Some(outer) = &self.outer {
                    outer.borrow().get(name)
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
