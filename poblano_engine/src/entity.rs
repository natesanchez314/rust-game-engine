use std::hash::{DefaultHasher, Hash, Hasher};

use crate::component::Component;

pub struct Entity {
    pub id: u32
}

impl Entity {
    pub fn new(name: &str) -> Self {
        let mut dh = DefaultHasher::new();
        name.hash(&mut dh);
        let id = dh.finish();

        Self {
            id: id as u32,
        }

    }

    pub fn add_component(&self, component: &dyn Component) {
        component.add_to_entity(self.id);
    }
}