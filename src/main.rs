fn main() {
    let distance: f32 = 0.0;
    let res = compute_dynamic_fare(distance);
    match res {
        Ok(fare) => {
            println!("fare computed  = {} ", fare)
        }
        Err(msg) => {
            println!("error computing fare : {}", msg)
        }
    }
}

fn compute_dynamic_fare(distance: f32) -> Result<f64, FareCalculationError> {
    if distance == 0.0 {
        return Err(FareCalculationError::new(
            DistanceCalculatorErrorReason::InvalidInput(distance),
        ));
    }
    Ok((distance as f64) * 3.0)
}

use core::fmt::{Debug, Display};
use std::fmt::Formatter;
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)>;
}

#[derive(Debug)]
enum DistanceCalculatorErrorReason {
    // pass the distance that was used as input to this enum variant.
    InvalidInput(f32),
    // store the status code within the enum value.
    WeatherServiceUnreachable(i32),
}

#[derive(Debug)]
struct FareCalculationError {
    reason: DistanceCalculatorErrorReason,
}

impl Display for FareCalculationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            DistanceCalculatorErrorReason::InvalidInput(distance) => {
                write!(
                    f,
                    "invalid input. distance must be greater than zero. Provided value {}.",
                    distance
                )
            }
            DistanceCalculatorErrorReason::WeatherServiceUnreachable(status) => {
                write!(
                    f,
                    "Unable to compute fare. Service unreachable with status {}.",
                    status
                )
            }
        }
    }
}

impl FareCalculationError {
    fn new(val: DistanceCalculatorErrorReason) -> Self {
        Self { reason: val }
    }
}
