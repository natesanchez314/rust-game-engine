pub mod component_2d;
pub mod component_3d;

pub trait Component {
    fn add_to_entity(&self, entity_id: u32);
}

pub struct Script {
    
}