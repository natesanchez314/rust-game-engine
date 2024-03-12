pub const MATH_TOLERANCE: f32 = 0.001;

pub fn isEqual(a: f32, b: f32, epsilon: f32) -> bool {
    let diff = a - b;
    diff >= -epsilon && diff <= epsilon
}

pub fn isNotEqual(a: f32, b: f32, epsilon: f32) -> bool {
    let diff = a - b;
    diff < -epsilon || diff > epsilon
}

pub fn isOne(a: f32, epsilon: f32) -> bool {
    let diff = a - 1.0;
    diff >= -epsilon && diff <= epsilon
}

pub fn isZero(a: f32, epsilon: f32) -> bool {
    a >= -epsilon && a <= epsilon
}

pub fn isNonZero(a: f32, epsilon: f32) -> bool {
    a < -epsilon || a > epsilon
}
