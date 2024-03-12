use std::ops::{Div, DivAssign};

use crate::{mat4::Mat4, vector3::Vector3};

enum Axis {
    X,
    Y,
    Z,
}

enum Orient {
    LocalToWorld,
    WorldToLocal,
}

#[derive(Clone, Copy)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub real: f32
}

impl Quat {

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            real: 0.0,
        }
    }

    pub fn identity() -> Self {
        Self {
            x: 0.0, 
            y: 0.0,
            z: 0.0,
            real: 1.0
        }
    }

    pub fn from_mat4(m: &Mat4) -> Self {
        let t = m.r0c0 + m.r1c1 + m.r2c2;
        if t > 0.0 {
            let s = 2.0 * f32::sqrt(t + 1.0);
            Self {
                x: -(m.r2c1 - m.r1c2) / s,
                y: -(m.r0c2 - m.r2c0) / s,
                z: -(m.r1c0 - m.r0c1) / s,
                real: 0.25 * s,
            }
        } else if m.r0c0 > m.r1c1 && m.r0c0 > m.r2c2 {
            let s = 2.0 * f32::sqrt(1.0 + m.r0c0 - m.r1c1 - m.r2c2);
            Self {
                x: 0.25 * s,
                y: (m.r0c1 + m.r1c0) / s,
                z: (m.r0c2 + m.r2c0) / s,
                real: -(m.r2c1 - m.r1c2) / s,
            }
        } else if m.r1c1 > m.r2c2 {
            let s = 2.0 * f32::sqrt(1.0 + m.r1c1 - m.r0c0 - m.r2c2);
            Self {
                x: (m.r0c1 + m.r1c0) / s,
                y: 0.25 * s,
                z: (m.r1c2 + m.r2c1) / s,
                real: -(m.r0c2 - m.r2c0) / s,
            }
        } else {
            let s = 2.0 * f32::sqrt(1.0 + m.r2c2 - m.r0c0 - m.r1c1);
            Self {
                x: (m.r0c2 + m.r2c0) / s,
                y: (m.r1c2 + m.r2c1) / s,
                z: 0.25 * s,
                real: -(m.r1c0 - m.r0c1) / s,
            }
        }
    }

    pub fn get_v(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn lqcvq(&self, v_in: &Vector3) -> Vector3 {
        let qv = self.get_v();
        2.0 * self.real * qv.cross(v_in) + 
        (self.real * self.real - qv.dot(&qv)) * 
        v_in + 2.0 * qv.dot(v_in) * qv
    }

    fn lqvqc(&self, v_in: &Vector3) -> Vector3 {
        let qv = self.get_v();
        2.0 * self.real * v_in.cross(&qv) +
        (self.real * self.real - qv.dot(&qv)) *
        v_in + 2.0 * qv.dot(v_in) * qv
    }

    fn conj(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    fn get_conj(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            real: 1.0,
        }
    }

    fn t(&mut self) {
        let mut m = Mat4::from_quat(&self);
        m.t();
        *self = Quat::from_mat4(&m);
    }

    fn get_t(&self) {
        let mut m = Mat4::from_quat(&self);
        m.t();
        Quat::from_mat4(&m);
    }

    fn get_mag(&self) -> f32 {
        f32::sqrt(self.x * self.x + 
            self.y * self.y +
            self.z * self.z +
            self.real * self.real
        )
    }

    fn get_mag_sqr(&self) -> f32 {
        self.x * self.x + 
        self.y * self.y +
        self.z * self.z +
        self.real * self.real
    }

    fn get_inv_mag(&self) -> f32 {
        1.0 / self.get_mag()
    }

    fn norm(&mut self) {
        let mag = self.get_mag();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.real /= mag;
    }

    fn get_norm(&self) -> Self {
        let mag = self.get_mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            real: self.real / mag,
        }  
    }

    fn inv(&mut self) {
        *self = self.get_conj() / self.get_mag_sqr()
    }

    fn get_inv(&self) -> Self {
        self.get_conj() / self.get_mag_sqr()
    }

    fn dot(&self, rhs: &Quat) -> f32 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z +
        self.real * rhs.real
    }

    fn get_angle(&self) -> f32 {
        2.0 * self.real.acos()
    }

    fn get_axis(&self) -> Vector3 {
        let mut v = Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        };
        v.norm();
        v
    }
}

impl Div<f32> for Quat {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            real: self.real / rhs,
        }
    }
}

impl DivAssign<f32> for Quat {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.real /= rhs;
    }
}