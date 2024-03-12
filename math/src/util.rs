pub const MATH_TOLERANCE: f32 = 0.001;

pub fn is_equal(a: f32, b: f32, epsilon: f32) -> bool {
    let diff = a - b;
    diff >= -epsilon && diff <= epsilon
}

pub fn is_not_equal(a: f32, b: f32, epsilon: f32) -> bool {
    let diff = a - b;
    diff < -epsilon || diff > epsilon
}

pub fn is_one(a: f32, epsilon: f32) -> bool {
    let diff = a - 1.0;
    diff >= -epsilon && diff <= epsilon
}

pub fn is_zero(a: f32, epsilon: f32) -> bool {
    a >= -epsilon && a <= epsilon
}

pub fn is_non_zero(a: f32, epsilon: f32) -> bool {
    a < -epsilon || a > epsilon
}
