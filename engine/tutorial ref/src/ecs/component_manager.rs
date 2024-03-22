use crate::component::{self, Component};

pub mod component_manager_2d;

pub trait ComponentManager {
    fn add_component(&mut self, component: &dyn Component);
}