use std::f32;

mod composites;
mod plotting;
mod rules;

use composites::*;

fn main() {
    print!("a: ");
    let a: f32 = text_io::read!("{}\n");

    print!("b: ");
    let b: f32 = text_io::read!("{}\n");

    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    let subintervals: u16 = 2;

    // output
    let (trap_sum, smps_sum) = composite_simple(subintervals, a, b);
    println!(
        "Simple Comp Trapezoid Rule: {trap_sum}\nSimple Comp Simpson's 1/3 Rule: {smps_sum}\n"
    );

    let (trap_sum, smps_sum) = composite_established(subintervals, a, b, 0.05);
    println!("Established Comp Trapezoid Rule: {trap_sum}\nEstablished Comp Simpson's 1/3 Rule: {smps_sum}\n");

    let (trap_sum, smps_sum) = composite_complexity(subintervals, a, b, 0.6);
    println!("Complexity Comp Trapezoid Rule: {trap_sum}\nComplexity Comp Simpson's 1/3 Rule: {smps_sum}");
}

fn f(x: f32) -> f32 {
    (13_f32 / f32::sqrt(2_f32 * f32::consts::PI))
        * f32::powf(f32::consts::E, -(1_f32 / 2_f32) * f32::powi(x, 2))
}

#[allow(dead_code)]
fn f_prime(x: f32) -> f32 {
    -x * f(x)
}

fn complexity(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    f32::abs(f(b) - f(a))
}
