use cgmath::Vector2;

pub struct Transform2d {
    translation: Vector2<f32>,
    scale: Vector2<f32>,
    rotation: f32,
}