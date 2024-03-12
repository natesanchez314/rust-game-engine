use std::collections::HashMap;

use crate::component::component_2d::Transform2d;

pub struct Transform2dManager {
    transform2ds: HashMap<u32, Transform2d>,
}

impl Transform2dManager {
    pub fn add_component(&mut self, entity_id: u32, component: Transform2d) {
        self.transform2ds.insert(entity_id, component);
    }
}