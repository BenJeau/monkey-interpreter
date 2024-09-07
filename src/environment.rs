use std::collections::BTreeMap;

use crate::object::Object;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Environment {
    store: BTreeMap<String, Object>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
            parent: None,
        }
    }

    pub fn new_child(&self) -> Self {
        Self {
            store: BTreeMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        let value = self.store.get(name);
        if value.is_some() {
            return value;
        }

        self.parent
            .as_ref()
            .map(|parent| parent.get(name))
            .flatten()
    }
}
