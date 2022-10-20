use std::f32;

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

fn composite_simple(subints: u16, a: f32, b: f32) -> (f32, f32) {
    let subinterval_width = f32::abs(b - a) / subints as f32;

    let mut t_sum: f32 = 0.0;
    let mut s_sum: f32 = 0.0;

    for i in 0..subints {
        let left = a + ((i as f32) * subinterval_width);
        let right = a + ((i + 1) as f32 * subinterval_width);

        t_sum += trapezoid_rule(left, right);
        s_sum += simpsons_rule(left, right);
    }
    (t_sum, s_sum)
}

fn composite_established(subints: u16, a: f32, b: f32, threshold: f32) -> (f32, f32) {
    let subinterval_width = f32::abs(b - a) / subints as f32;

    let mut t_sum: f32 = 0.0;
    let mut s_sum: f32 = 0.0;

    for i in 0..subints {
        let left = a + ((i as f32) * subinterval_width);
        let right = a + ((i + 1) as f32 * subinterval_width);

        let mut trap_val = trapezoid_rule(left, right);
        let mut smps_val = simpsons_rule(left, right);

        if f32::abs(trap_val - smps_val) > threshold {
            (trap_val, smps_val) = composite_established(2, left, right, threshold);
        }

        t_sum += trap_val;
        s_sum += smps_val;
    }
    (t_sum, s_sum)
}

fn composite_complexity(subints: u16, a: f32, b: f32, threshold: f32) -> (f32, f32) {
    let subinterval_width = f32::abs(b - a) / subints as f32;

    let mut t_sum: f32 = 0.0;
    let mut s_sum: f32 = 0.0;

    for i in 0..subints {
        let left = a + ((i as f32) * subinterval_width);
        let right = a + ((i + 1) as f32 * subinterval_width);

        // println!("Complexity: {} ({i})", complexity(a, b));
        let mut trap_val = trapezoid_rule(left, right);
        let mut smps_val = simpsons_rule(left, right);

        if complexity(left, right) > 0.5 {
            (trap_val, smps_val) = composite_complexity(2, left, right, threshold);
        }

        t_sum += trap_val;
        s_sum += smps_val;
    }
    (t_sum, s_sum)
}

fn trapezoid_rule(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    // calculate trapezoidal area
    ((b - a) / 2.0) * (f(a) + f(b))
}

fn simpsons_rule(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    // calculate area
    (f(a) + 4.0 * f((a + b) / 2.0) + f(b)) * ((b - a) / 6.0)
}

fn complexity(a: f32, b: f32) -> f32 {
    // Check input validity
    assert!(b > a, "Right bound must be greater than left bound");

    f32::abs(f(b) - f(a))
}
