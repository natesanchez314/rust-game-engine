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
}