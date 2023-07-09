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
        return Err(FareCalculationError::InvalidInput {
            input_distance: distance,
        });
    }
    Ok((distance as f64) * 3.0)
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FareCalculationError {
    #[error(
        "invalid input. distance must be greater than zero. Provided value {input_distance:?}"
    )]
    InvalidInput { input_distance: f32 },
    #[error("Unable to compute fare. Service unreachable with status `{0}`.")]
    WeatherServiceUnreachable(i32),
}
