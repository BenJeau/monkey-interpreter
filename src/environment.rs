use std::collections::BTreeMap;

use crate::object::Object;

#[derive(Default)]
pub struct Environment {
    store: BTreeMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }
}
