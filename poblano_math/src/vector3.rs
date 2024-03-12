use crate::vector4::Vector4;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec4(v: &Vector4) ->  Self {
        Self { x: v.x, y: v.y, z: v.z }
    }

    pub fn norm(&mut self) {
        let mag = self.get_mag();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    pub fn get_norm(&self) -> Self {
        let mag = self.get_mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn get_mag(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    pub fn get_mag_sqr(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn dot(&self) -> f32 {
        1.0
    }

    pub fn cross(&self) -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn getAngle(&self) -> f32 {
        1.0
    }
}

/*impl ops::Mul<Mat3> for Vec3 {
    todo
}*/