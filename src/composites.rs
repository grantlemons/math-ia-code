use crate::{complexity, rules::*};
use std::f32;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Mode {
    Established,
    Complexity,
    Simple,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Mode::Established => "Established",
            Mode::Complexity => "Complexity",
            Mode::Simple => "Simple",
        };
        write!(f, "{}", string)
    }
}

impl std::str::FromStr for Mode {
    type Err = ();
    fn from_str(s: &str) -> Result<Mode, ()> {
        match s.to_lowercase().as_str() {
            "e" => Ok(Mode::Established),
            "c" => Ok(Mode::Complexity),
            "s" => Ok(Mode::Simple),
            _ => {
                println!("Please enter either 'e', 'c', or 's', for the established method, complexity method, and simple method respectively.");
                Err(())
            }
        }
    }
}

impl Mode {
    pub fn threshold(self) -> f32 {
        match self {
            Mode::Established => 0.002,
            Mode::Complexity => 0.4,
            _ => 0.0,
        }
    }
}

pub fn composite(mode: Mode, subints: u16, a: f32, b: f32, xcoords: &mut Vec<f32>) -> f32 {
    // Calculate subinterval width
    let subinterval_width = f32::abs(b - a) / subints as f32;

    // Declare and Initialize sum variable
    let mut simpsons_sum: f32 = 0.0;

    // Loop through each subinterval
    for i in 0..subints {
        // Calculate subinterval bounds
        let left = a + ((i as f32) * subinterval_width);
        let right = a + ((i + 1) as f32 * subinterval_width);

        // Add left subinterval bound to vector for output
        xcoords.push(left);

        // Store outputs of respective rules
        let mut simpsons_value = simpsons_rule(left, right);

        // Check the conditions for division dependent on mode parameter
        let condition = match mode {
            Mode::Established => {
                f32::abs(trapezoid_rule(left, right) - simpsons_value) > mode.threshold()
            }
            Mode::Complexity => complexity(left, right) > mode.threshold(),
            Mode::Simple => false,
        };

        // Recursively divide if above condition met
        if condition {
            simpsons_value = composite(mode, 2, left, right, xcoords);
        }

        // Add values of local variables to sum variable
        simpsons_sum += simpsons_value;
    }

    // Return the two sums
    simpsons_sum
}
