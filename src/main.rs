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

    print!("Subdivisions: ");
    let subdivisions: u16 = text_io::read!("{}\n");

    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    let mut xcoords: Vec<f32> = vec![b];

    let smps_sum = composite(mode, subdivisions, a, b, &mut xcoords);
    let err = error(smps_sum, 9.999_346);
    println!("{mode} Comp Simpson's 1/3 Rule: {smps_sum} ({err}%)");

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
    f(x) * (x.powf(2.0) - 1.0)
}

fn complexity(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    f32::abs(f_prime_prime(b) - f_prime_prime(a))
}

fn error(value: f32, correct_value: f32) -> f32 {
    f32::abs(((correct_value - value) / correct_value) * 100.0)
}
