use std::f32;

fn main() {
    print!("a: ");
    let a: f32 = text_io::read!("{}\n");

    print!("b: ");
    let b: f32 = text_io::read!("{}\n");

    let interval = b - a;
    let subintervals = 5;
    let subinterval_width = interval / subintervals as f32;

    // calculate sum
    let mut sum: f32 = 0.0;
    for i in 0..subintervals {
        sum += trapezoid_rule(
            a + ((i as f32) * subinterval_width),
            a + ((i + 1) as f32 * subinterval_width),
            3,
        );
    }

    // output
    println!("Integral: {}", sum)
}

fn f(x: f32) -> f32 {
    (13_f32 / f32::sqrt(2_f32 * f32::consts::PI))
        * f32::powf(f32::consts::E, -(1_f32 / 2_f32) * f32::powi(x, 2))
}

fn f_prime(x: f32) -> f32 {
    -x * f(x)
}

fn trapezoid_rule(a: f32, b: f32, threshold: u8) -> f32 {
    assert!(b > a, "Right bound must be greater than left bound");
    let left = f_prime(a);
    let right = f_prime(b);
    let width = b - a;
    let avg_slope = f32::abs((right - left) / width);

    // recursively divide
    if avg_slope >= threshold.into() {
        return trapezoid_rule(a, b - (width / 2.0), threshold)
            + trapezoid_rule(a + (width / 2.0), b, threshold);
    }

    // calculate trapezoidal area
    (width / 2.0) * (left + right)
}
