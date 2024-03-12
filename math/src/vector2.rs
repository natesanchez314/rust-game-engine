use crate::vector3::Vector3;
use crate::vector4::Vector4;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_vector3(v: &Vector3) ->  Self {
        Self { x: v.x, y: v.y }
    }

    pub fn from_vector4(v: &Vector4) ->  Self {
        Self { x: v.x, y: v.y }
    }

    pub fn norm(&mut self) {
        let mag = self.get_mag();
        self.x /= mag;
        self.y /= mag;
    }

    pub fn get_norm(&self) -> Self {
        let mag = self.get_mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn get_mag(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y))
    }

    pub fn get_mag_sqr(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn dot(&self, rhs: &Vector2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn get_angle(&self, rhs: &Vector2) -> f32 {
        let dot = self.dot(rhs);
        let mag_a = self.get_mag();
        let mag_b = rhs.get_mag();
        (dot / (mag_a * mag_b)).acos()
    }
}