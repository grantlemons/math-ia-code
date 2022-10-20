use crate::{complexity, rules::*};
use std::f32;

pub fn composite_simple(subints: u16, a: f32, b: f32, xcoords: &mut Vec<f32>) -> (f32, f32) {
    let subinterval_width = f32::abs(b - a) / subints as f32;

    let mut t_sum: f32 = 0.0;
    let mut s_sum: f32 = 0.0;

    for i in 0..subints {
        let left = a + ((i as f32) * subinterval_width);
        let right = a + ((i + 1) as f32 * subinterval_width);
        xcoords.push(left);

        t_sum += trapezoid_rule(left, right);
        s_sum += simpsons_rule(left, right);
    }
    (t_sum, s_sum)
}

pub fn composite_established(
    subints: u16,
    a: f32,
    b: f32,
    threshold: f32,
    xcoords: &mut Vec<f32>,
) -> (f32, f32) {
    let subinterval_width = f32::abs(b - a) / subints as f32;

    let mut t_sum: f32 = 0.0;
    let mut s_sum: f32 = 0.0;

    for i in 0..subints {
        let left = a + ((i as f32) * subinterval_width);
        let right = a + ((i + 1) as f32 * subinterval_width);
        xcoords.push(left);

        let mut trap_val = trapezoid_rule(left, right);
        let mut smps_val = simpsons_rule(left, right);

        if f32::abs(trap_val - smps_val) > threshold {
            (trap_val, smps_val) = composite_established(2, left, right, threshold, xcoords);
        }

        t_sum += trap_val;
        s_sum += smps_val;
    }
    (t_sum, s_sum)
}

pub fn composite_complexity(
    subints: u16,
    a: f32,
    b: f32,
    threshold: f32,
    xcoords: &mut Vec<f32>,
) -> (f32, f32) {
    let subinterval_width = f32::abs(b - a) / subints as f32;

    let mut t_sum: f32 = 0.0;
    let mut s_sum: f32 = 0.0;

    for i in 0..subints {
        let left = a + ((i as f32) * subinterval_width);
        let right = a + ((i + 1) as f32 * subinterval_width);
        xcoords.push(left);

        // println!("Complexity: {} ({i})", complexity(a, b));
        let mut trap_val = trapezoid_rule(left, right);
        let mut smps_val = simpsons_rule(left, right);

        if complexity(left, right) > threshold {
            (trap_val, smps_val) = composite_complexity(2, left, right, threshold, xcoords);
        }

        t_sum += trap_val;
        s_sum += smps_val;
    }
    (t_sum, s_sum)
}
