pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec4(&v: Vec4) ->  Self {
        Self { v.x, v.y, v.z }
    }

    pub fn norm(&mut self) {
        let mag = self.get_mag();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    pub fn get_norm(&self) -> Self {
        Self { 0.0, 0.0, 0.0 }
    }

    pub fn get_mag(&self) -> f32 {
        // Todo!()
    }

    pub fn get_mag_sqr(&self) -> f32 {
        1.0
    }

    pub fn dot(&self) -> f32 {
        1.0
    }

    pub fn cross(&self) -> Self {
        Self { v.x, v.y, v.z }
    }

    pub fn getAngle(&self) -> f32 {
        1.0
    }
}

impl ops::Mul<Mat3> for Vec3 {
    fn mul 
}