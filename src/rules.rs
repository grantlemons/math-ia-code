use crate::f;

pub fn trapezoid_rule(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    // calculate trapezoidal area
    ((b - a) / 2.0) * (f(a) + f(b))
}

pub fn simpsons_rule(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    // calculate area
    (f(a) + 4.0 * f((a + b) / 2.0) + f(b)) * ((b - a) / 6.0)
}
