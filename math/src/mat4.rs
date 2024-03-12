use std::ops::Mul;

use crate::quat::Quat;

enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
pub struct Mat4 {
    pub r0c0: f32,
    pub r0c1: f32,
    pub r0c2: f32,
    pub r0c3: f32,
    
    pub r1c0: f32,
    pub r1c1: f32,
    pub r1c2: f32,
    pub r1c3: f32,
    
    pub r2c0: f32,
    pub r2c1: f32,
    pub r2c2: f32,
    pub r2c3: f32,
    
    pub r3c0: f32,
    pub r3c1: f32,
    pub r3c2: f32,
    pub r3c3: f32,
}

impl Mat4 {

    pub fn from_quat(q: &Quat) -> Self {
        let x_x = q.x * q.x;
        let x_y = q.x * q.y;
        let x_z = q.x * q.z;
        let x_w = q.x * q.real;
        let y_y = q.y * q.y;
        let y_z = q.y * q.z;
        let y_w = q.y * q.real;
        let z_z = q.z * q.z;
        let z_w = q.z * q.z;

        Self {
            r0c0: 1.0 - (2.0 * y_y + z_z),
            r0c1: 2.0 * (x_y + z_w),
            r0c2: 2.0 * (x_z - y_w),
            r0c3: 0.0,

            r1c0: 2.0 * (x_y - z_w),
            r1c1: 1.0 - (2.0 * (x_x + z_z)),
            r1c2: 2.0 * (y_z + x_w),
            r1c3: 0.0,
    
            r2c0: 2.0 * (x_z + y_w),
            r2c1: 2.0 * (y_z - x_w),
            r2c2: 1.0 - (2.0 * (x_x + y_y)),
            r2c3: 0.0,

            r3c0: 0.0,
            r3c1: 0.0,
            r3c2: 0.0,
            r3c3: 1.0,
        }
    }

    pub fn zero() -> Self {
        Self {
            r0c0: 0.0,
            r0c1: 0.0,
            r0c2: 0.0,
            r0c3: 0.0,

            r1c0: 0.0,
            r1c1: 0.0,
            r1c2: 0.0,
            r1c3: 0.0,
    
            r2c0: 0.0,
            r2c1: 0.0,
            r2c2: 0.0,
            r2c3: 0.0,

            r3c0: 0.0,
            r3c1: 0.0,
            r3c2: 0.0,
            r3c3: 0.0,
        }
    }

    pub fn identity() -> Self {
        Self {
            r0c0: 1.0,
            r0c1: 0.0,
            r0c2: 0.0,
            r0c3: 0.0,

            r1c0: 0.0,
            r1c1: 1.0,
            r1c2: 0.0,
            r1c3: 0.0,
    
            r2c0: 0.0,
            r2c1: 0.0,
            r2c2: 1.0,
            r2c3: 0.0,

            r3c0: 0.0,
            r3c1: 0.0,
            r3c2: 0.0,
            r3c3: 1.0,
        }
    }

    pub fn get_det(&self) -> f32 {
        self.r0c0 * (
            self.r1c1 * (self.r2c2 * self.r3c3 - self.r2c3 * self.r3c2) -
            self.r1c2 * (self.r2c1 * self.r3c3 - self.r2c3 * self.r3c1) + 
            self.r1c3 * (self.r2c1 * self.r3c2 - self.r2c2 * self.r3c1)) -
        self.r0c1 * (
            self.r1c0 * (self.r2c2 * self.r3c3 - self.r2c3 * self.r3c2) -
            self.r1c2 * (self.r2c0 * self.r3c3 - self.r2c3 * self.r3c0) +
            self.r1c3 * (self.r2c0 * self.r3c2 - self.r2c2 * self.r3c0)) +
        self.r0c2 * (
            self.r1c0 * (self.r2c1 * self.r3c3 - self.r2c3 * self.r3c1) -
            self.r1c1 * (self.r2c0 * self.r3c3 - self.r2c3 * self.r3c0) +
            self.r1c3 * (self.r2c0 * self.r3c1 - self.r2c1 * self.r3c0)) -
        self.r0c3 * (
            self.r1c0 * (self.r2c1 * self.r3c2 - self.r2c2 * self.r3c1) -
            self.r1c1 * (self.r2c0 * self.r3c2 - self.r2c2 * self.r3c0) +
            self.r1c2 * (self.r2c0 * self.r3c1 - self.r2c1 * self.r3c0))
    }

    pub fn t(&mut self) {
        let mut tmp = self.r0c1;
        self.r0c1 = self.r1c0;
        self.r1c0 = tmp;

        tmp = self.r0c2;
        self.r0c2 = self.r2c0;
        self.r2c0 = tmp;

        tmp = self.r0c3;
        self.r0c3 = self.r3c0;
        self.r3c0 = tmp;

        tmp = self.r1c2;
        self.r1c2 = self.r2c1;
        self.r2c1 = tmp;

        tmp = self.r1c3;
        self.r1c3 = self.r3c1;
        self.r3c1 = tmp;

        tmp = self.r2c3;
        self.r2c3 = self.r3c2;
        self.r3c2 = tmp;
    }

    pub fn get_t(&self) -> Self {
        Self {
            r0c0: self.r0c0, r0c1: self.r1c0, r0c2: self.r2c0, r0c3: self.r3c0,
            r1c0: self.r0c1, r1c1: self.r1c1, r1c2: self.r2c1, r1c3: self.r3c1,
            r2c0: self.r0c2, r2c1: self.r1c2, r2c2: self.r2c2, r2c3: self.r3c2,
            r3c0: self.r0c3, r3c1: self.r1c3, r3c2: self.r2c3, r3c3: self.r3c3,
        }
    }

    pub fn inv(&mut self) {
        todo!()
    }

    pub fn get_inv(&self) -> Self {
        let m = Mat4{
            r0c0: (self.r1c1 * self.r2c2 * self.r3c3) + (self.r1c2 * self.r2c3 * self.r3c1) + (self.r1c3 * self.r2c1 * self.r3c2) - (self.r1c1 * self.r2c3 * self.r3c2) - (self.r1c2 * self.r2c1 * self.r3c3) - (self.r1c3 * self.r2c2 * self.r3c1),
            r0c1: (self.r0c1 * self.r2c3 * self.r3c2) + (self.r0c2 * self.r2c1 * self.r3c3) + (self.r0c3 * self.r2c2 * self.r3c1) - (self.r0c1 * self.r2c2 * self.r3c3) - (self.r0c2 * self.r2c3 * self.r3c1) - (self.r0c3 * self.r2c1 * self.r3c2),
            r0c2: (self.r0c1 * self.r1c2 * self.r3c3) + (self.r0c2 * self.r1c3 * self.r3c1) + (self.r0c3 * self.r1c1 * self.r3c2) - (self.r0c1 * self.r1c3 * self.r3c2) - (self.r0c2 * self.r1c1 * self.r3c3) - (self.r0c3 * self.r1c2 * self.r3c1),
            r0c3: (self.r0c1 * self.r1c3 * self.r2c2) + (self.r0c2 * self.r1c1 * self.r2c3) + (self.r0c3 * self.r1c2 * self.r2c1) - (self.r0c1 * self.r1c2 * self.r2c3) - (self.r0c2 * self.r1c3 * self.r2c1) - (self.r0c3 * self.r1c1 * self.r2c2),
            r1c0: (self.r1c0 * self.r2c3 * self.r3c2) + (self.r1c2 * self.r2c0 * self.r3c3) + (self.r1c3 * self.r2c2 * self.r3c0) - (self.r1c0 * self.r2c2 * self.r3c3) - (self.r1c2 * self.r2c3 * self.r3c0) - (self.r1c3 * self.r2c0 * self.r3c2),
            r1c1: (self.r0c0 * self.r2c2 * self.r3c3) + (self.r0c2 * self.r2c3 * self.r3c0) + (self.r0c3 * self.r2c0 * self.r3c2) - (self.r0c0 * self.r2c3 * self.r3c2) - (self.r0c2 * self.r2c0 * self.r3c3) - (self.r0c3 * self.r2c2 * self.r3c0),
            r1c2: (self.r0c0 * self.r1c3 * self.r3c2) + (self.r0c2 * self.r1c0 * self.r3c3) + (self.r0c3 * self.r1c2 * self.r3c0) - (self.r0c0 * self.r1c2 * self.r3c3) - (self.r0c2 * self.r1c3 * self.r3c0) - (self.r0c3 * self.r1c0 * self.r3c2),
            r1c3: (self.r0c0 * self.r1c2 * self.r2c3) + (self.r0c2 * self.r1c3 * self.r2c0) + (self.r0c3 * self.r1c0 * self.r2c2) - (self.r0c0 * self.r1c3 * self.r2c2) - (self.r0c2 * self.r1c0 * self.r2c3) - (self.r0c3 * self.r1c2 * self.r2c0),
            r2c0: (self.r1c0 * self.r2c1 * self.r3c3) + (self.r1c1 * self.r2c3 * self.r3c0) + (self.r1c3 * self.r2c0 * self.r3c1) - (self.r1c0 * self.r2c3 * self.r3c1) - (self.r1c1 * self.r2c0 * self.r3c3) - (self.r1c3 * self.r2c1 * self.r3c0),
            r2c1: (self.r0c0 * self.r2c3 * self.r3c1) + (self.r0c1 * self.r2c0 * self.r3c3) + (self.r0c3 * self.r2c1 * self.r3c0) - (self.r0c0 * self.r2c1 * self.r3c3) - (self.r0c1 * self.r2c3 * self.r3c0) - (self.r0c3 * self.r2c0 * self.r3c1),
            r2c2:  (self.r0c0 * self.r1c1 * self.r3c3) + (self.r0c1 * self.r1c3 * self.r3c0) + (self.r0c3 * self.r1c0 * self.r3c1) - (self.r0c0 * self.r1c3 * self.r3c1) - (self.r0c1 * self.r1c0 * self.r3c3) - (self.r0c3 * self.r1c1 * self.r3c0),
            r2c3:  (self.r0c0 * self.r1c3 * self.r2c1) + (self.r0c1 * self.r1c0 * self.r2c3) + (self.r0c3 * self.r1c1 * self.r2c0) - (self.r0c0 * self.r1c1 * self.r2c3) - (self.r0c1 * self.r1c3 * self.r2c0) - (self.r0c3 * self.r1c0 * self.r2c1),
            r3c0:  (self.r1c0 * self.r2c2 * self.r3c1) + (self.r1c1 * self.r2c0 * self.r3c2) + (self.r1c2 * self.r2c1 * self.r3c0) - (self.r1c0 * self.r2c1 * self.r3c2) - (self.r1c1 * self.r2c2 * self.r3c0) - (self.r1c2 * self.r2c0 * self.r3c1),
            r3c1:  (self.r0c0 * self.r2c1 * self.r3c2) + (self.r0c1 * self.r2c2 * self.r3c0) + (self.r0c2 * self.r2c0 * self.r3c1) - (self.r0c0 * self.r2c2 * self.r3c1) - (self.r0c1 * self.r2c0 * self.r3c2) - (self.r0c2 * self.r2c1 * self.r3c0),
            r3c2:  (self.r0c0 * self.r1c2 * self.r3c1) + (self.r0c1 * self.r1c0 * self.r3c2) + (self.r0c2 * self.r1c1 * self.r3c0) - (self.r0c0 * self.r1c1 * self.r3c2) - (self.r0c1 * self.r1c2 * self.r3c0) - (self.r0c2 * self.r1c0 * self.r3c1),
            r3c3:  (self.r0c0 * self.r1c1 * self.r2c2) + (self.r0c1 * self.r1c2 * self.r2c0) + (self.r0c2 * self.r1c0 * self.r2c1) - (self.r0c0 * self.r1c2 * self.r2c1) - (self.r0c1 * self.r1c0 * self.r2c2) - (self.r0c2 * self.r1c1 * self.r2c0),
        };
		(1.0 / self.get_det()) * m
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            r0c0: self * rhs.r0c0,
            r0c1: self * rhs.r0c1,
            r0c2: self * rhs.r0c2,
            r0c3: self * rhs.r0c3,

            r1c0: self * rhs.r1c0,
            r1c1: self * rhs.r1c1,
            r1c2: self * rhs.r1c2,
            r1c3: self * rhs.r1c3,
            
            r2c0: self * rhs.r2c0,
            r2c1: self * rhs.r2c1,
            r2c2: self * rhs.r2c2,
            r2c3: self * rhs.r2c3,

            r3c0: self * rhs.r3c0,
            r3c1: self * rhs.r3c1,
            r3c2: self * rhs.r3c2,
            r3c3: self * rhs.r3c3,
        }
    }
}