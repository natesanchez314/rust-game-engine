use cgmath::Vector3;

pub struct Transform3d {
    translation: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
}
