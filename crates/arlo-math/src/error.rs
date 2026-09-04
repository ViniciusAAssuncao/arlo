#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum MathError {
    #[error("Invalid probability value: {0}, must be in 0.0..=1.0")]
    InvalidProbability(f64),
}