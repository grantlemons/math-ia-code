fn main() {
    print!("a: ");
    let a: f32 = text_io::read!("{}\n");

    print!("b: ");
    let b: f32 = text_io::read!("{}\n");

    let interval = b - a;
    let subintervals = 5;
    let subinterval_width = interval / subintervals;

    // calculate sum
    let sum = 0;
    for i in 0..subintervals {
        sum += trapezoid_rule(
            a + (i * subinterval_width),
            a + ((i + 1) * subinterval_width),
            3,
        );
    }
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
