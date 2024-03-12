use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{mat3::Mat3, mat4::Mat4, quat::Quat, util::{is_equal, MATH_TOLERANCE}, vector4::Vector4};

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn from_vec4(v: &Vector4) ->  Self {
        Self { x: v.x, y: v.y, z: v.z }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
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
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn get_mag_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vector3) -> Self {
        Self { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: -(self.x * rhs.z - self.z * rhs.x), 
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn get_angle(&self, rhs: &Vector3) -> f32 {
        let dot = self.dot(rhs);
        let mag_a = self.get_mag();
        let mag_b = rhs.get_mag();
        (dot / (mag_a * mag_b)).acos()
    }

    pub fn is_equal(&self, rhs: &Vector3, epsilon: f32) -> bool{
        is_equal(self.x, rhs.x, epsilon) && 
        is_equal(self.y, rhs.y, epsilon) && 
        is_equal(self.z, rhs.z, epsilon)
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, rhs: &Self) -> bool{
        self.is_equal(rhs, MATH_TOLERANCE)
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }    
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }    
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs, 
            z: self.z * rhs
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}


impl Mul<Mat3> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Mat3) -> Self {
        Self {
            x: self.x * rhs.r0c0 + self.y * rhs.r1c0 + self.z * rhs.r2c0,
            y: self.x * rhs.r0c1 + self.y * rhs.r1c1 + self.z * rhs.r2c1,
            z: self.x * rhs.r0c2 + self.y * rhs.r1c2 + self.z + rhs.r2c2,
        }
    }
}


impl MulAssign<Mat3> for Vector3 {
    fn mul_assign(&mut self, rhs: Mat3) {
        *self = Self {
            x: self.x * rhs.r0c0 + self.y * rhs.r1c0 + self.z * rhs.r2c0,
            y: self.x * rhs.r0c1 + self.y * rhs.r1c1 + self.z * rhs.r2c1,
            z: self.x * rhs.r0c2 + self.y * rhs.r1c2 + self.z + rhs.r2c2,
        }
    }
}

impl Mul<Mat4> for Vector3 {
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

impl Mul<Quat> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Quat) -> Self {
        rhs.lqcvq(&self)
    }
}

impl MulAssign<Quat> for Vector3 {
    fn mul_assign(&mut self, rhs: Quat) {
        *self = rhs.lqcvq(self)
    }
}