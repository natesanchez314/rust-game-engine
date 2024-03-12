use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{mat4::Mat4, util::{isEqual, MATH_TOLERANCE}};

pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector4 {

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn norm(&mut self) {
        let mag = self.get_mag();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.w /= mag;
    }

    pub fn get_norm(&self) -> Self {
        let mag = self.get_mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn get_mag(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn get_mag_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn dot(&self, rhs: &Vector4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, rhs: &Vector4) -> Self {
        Self { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: -(self.x * rhs.z - self.z * rhs.x), 
            z: self.x * rhs.y - self.y * rhs.x,
            w: 1.0
        }
    }

    pub fn getAngle(&self, rhs: &Vector4) -> f32 {
        let dot = self.dot(rhs);
        let magA = self.get_mag();
        let magB = rhs.get_mag();
        (dot / (magA * magB)).acos()
    }

    pub fn isEqual(&self, rhs: &Vector4, epsilon: f32) -> bool{
        isEqual(self.x, rhs.x, epsilon) && 
        isEqual(self.y, rhs.y, epsilon) && 
        isEqual(self.z, rhs.z, epsilon) &&
        isEqual(self.w, rhs.w, epsilon)
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, rhs: &Self) -> bool{
        self.isEqual(rhs, MATH_TOLERANCE)
    }
}

impl Add for Vector4 {
    type Output = Self;
    fn add(self, rhs: Vector4) -> Self {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }    
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vector4 {
    type Output = Self;
    fn sub(self, rhs: Vector4) -> Self {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }    
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Vector4 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: 1.0
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs, 
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl Mul<Mat4> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Mat4) -> Vector4 {
        Vector4 {
            x: self.x * rhs.r0c0 + self.y * rhs.r1c0 + self.z * rhs.r2c0 + rhs.r3c0,
            y: self.x * rhs.r0c1 + self.y * rhs.r1c1 + self.z * rhs.r2c1 + rhs.r3c1,
            z: self.x * rhs.r0c2 + self.y * rhs.r1c2 + self.z + rhs.r2c2 + rhs.r3c2,
            w: self.x * rhs.r0c3 + self.y * rhs.r1c3 + self.z + rhs.r2c3 + rhs.r3c3,
        }
    }
}
/*/
impl Mul<Quat> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Quat) -> Self {
        Self {
            x: 0.0,
            y: 0.0, 
            z: 0.0,
            w: 0.0,
        }
    }
}

impl MulAssign<Quat> for Vector4 {
    fn mul_assign(&mut self, rhs: Quat) {
        *self = Self {
            x: 0.0,
            y: 0.0, 
            z: 0.0,
            w: 0.0,
        }
    }
}*/