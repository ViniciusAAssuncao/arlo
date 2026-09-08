pub mod aggregate;
pub mod saturation;
pub mod weight;

pub use aggregate::{calculate_weighted_average, calculate_weighted_saturated_average};
pub use saturation::apply_saturation;
pub use weight::AttributeWeight;
