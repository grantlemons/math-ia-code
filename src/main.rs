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

    println!("Please enter either 'e', 'c', or 's', for the established method, complexity method, and simple method respectively.");
    print!("Mode (e/c/s): ");
    let mode: Mode = text_io::read!("{}\n");

    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    let subintervals: u16 = 4;
    let mut xcoords: Vec<f32> = vec![b];

    let (trap_sum, smps_sum) = composite(mode, subintervals, a, b, 0.8, &mut xcoords);
    let trap_err = error(trap_sum, 9.999_367);
    let smps_err = error(smps_sum, 9.999_367);
    println!("{mode} Comp Trapezoid Rule: {trap_sum} ({trap_err}%)\n{mode} Comp Simpson's 1/3 Rule: {smps_sum} ({smps_err}%)");

    xcoords.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xcoords.dedup();
    println!("{:?}", xcoords);

    plotting::draw(xcoords);
}

fn f(x: f32) -> f32 {
    (10_f32 / f32::sqrt(2_f32 * f32::consts::PI))
        * f32::powf(f32::consts::E, -(1_f32 / 2_f32) * f32::powi(x, 2))
}

#[allow(dead_code)]
fn f_prime(x: f32) -> f32 {
    -x * f(x)
}

#[allow(dead_code)]
fn f_prime_prime(x: f32) -> f32 {
    -f(x) - x * f_prime(x)
}

fn complexity(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    f32::abs(f_prime_prime(b) - f_prime_prime(a))
}

fn error(value: f32, correct_value: f32) -> f32 {
    f32::abs(((correct_value - value) / correct_value) * 100.0)
}
