use cgmath::{Vector3, Quaternion};

use crate::ecs::component::{Component, InstanceRaw};

pub struct Transform3d {
    pub translation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Component for Transform3d {
    fn to_raw(&self) -> InstanceRaw {
        let model = cgmath::Matrix4::from_translation(self.translation)
            * cgmath::Matrix4::from_scale(1.0)
            * cgmath::Matrix4::from(self.rotation);
        InstanceRaw {
            model: model.into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
    
    fn add_to_entity(&self, entity_id: u32) {
        todo!()
    }
}
