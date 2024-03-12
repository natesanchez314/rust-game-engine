enum Axis {
    X,
    Y,
    Z,
}

enum Orient {
    LocalToWorld,
    WorldToLocal,
}

pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    real: f32
}

impl Quat {

    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            real: 0.0,
        }
    }

    fn identity() -> Self {
        Self {
            x: 0.0, 
            y: 0.0,
            z: 0.0,
            real: 1.0
        }
    }

    fn lqcvq() {
        todo!();
    }

    fn lqvqc() {
        todo!();
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

    fn t() {

    }

    fn get_t() {

    }

    fn get_mag() -> f32 {
        0.0
    }

    fn get_mag_sqr() -> f32 {
        0.0
    }

    fn get_inv_mag() -> f32 {
        0.0
    }

    fn norm() {
        
    }

    fn get_norm() {

    }

    fn inv() {

    }

    fn get_inv() {

    }

    fn dot() {

    }

    fn get_angle() {

    }

    fn get_axis() {
        
    }
}

