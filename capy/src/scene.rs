use crate::camera::{Camera2d, camera3d::Camera3d};
use crate::ecs::entity::Entity;

pub trait Scene {
    fn update(delta: f32);
    fn render(); 
}

pub struct Scene2d {
    camera: Camera2d,
    entity: Vec<Entity>,
}

impl Scene for Scene2d {
    fn update(delta: f32) {

    }

    fn render() {

    }
}

pub struct Scene3d {
    camera: Camera3d,
    entity: Vec<Entity>,
}

impl Scene for Scene3d {
    fn update(delta: f32) {

    }

    fn render() {

    }
}