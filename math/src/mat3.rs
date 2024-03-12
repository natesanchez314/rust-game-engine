pub struct Mat3 {
    pub r0c0: f32,
    pub r0c1: f32,
    pub r0c2: f32,
    
    pub r1c0: f32,
    pub r1c1: f32,
    pub r1c2: f32,
    
    pub r2c0: f32,
    pub r2c1: f32,
    pub r2c2: f32,
}

impl Mat3 {
    pub fn zero() -> Self {
        Self {
            r0c0: 0.0,
            r0c1: 0.0,
            r0c2: 0.0,
    
            r1c0: 0.0,
            r1c1: 0.0,
            r1c2: 0.0,
    
            r2c0: 0.0,
            r2c1: 0.0,
            r2c2: 0.0,
        }
    }

    pub fn identity() -> Self {
        Self {
            r0c0: 1.0,
            r0c1: 0.0,
            r0c2: 0.0,
    
            r1c0: 0.0,
            r1c1: 1.0,
            r1c2: 0.0,
    
            r2c0: 0.0,
            r2c1: 0.0,
            r2c2: 1.0,
        }
    }
}