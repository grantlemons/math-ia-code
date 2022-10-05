fn main() {
    println!("Hello, world!");
}

fn f(x: f32) -> f32 {
    (13 / f32::sqrt(2 * f32::consts::PI)) * f23::consts::E ^ (-(1 / 2) * x ^ 2)
}

fn f_prime(x: f32) -> f32 {
    -x * f(x)
}

fn trapezoid_rule(a: f32, b: f32, threshold: u8) -> f32 {
    assert!(b > a, "Right bound must be greater than left bound");
    let left = f_prime(a);
    let right = f_prime(b);
    let width = b - a;
    let avg_slope = i32::abs((right - left) / width);

    // recursively divide
    if (avg_slope >= threshold) {
        trapezoid_rule(a, b - (height / 2), threshold)
            + trapezoid_rule(a + (height / 2), b, threshold)
    }

    // calculate trapezoidal area
    (height / 2) * (left + right)
}
