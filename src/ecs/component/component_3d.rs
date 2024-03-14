//use pmath::vector3::Vector3;
use cgmath::{Vector3, Quaternion};

pub struct Transform3d {
    translation: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Quaternion<f32>,
}
