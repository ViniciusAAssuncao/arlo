use arlo_domain::sport_constants::{LONG_LAUNCH_RANGE_SENSITIVITY, SHORT_PASS_RANGE_SENSITIVITY};
use arlo_tactics::PassingRange;

pub fn short_pass_advance_multiplier(passing_range: PassingRange) -> f64 {
    (1.0 - passing_range.value() * SHORT_PASS_RANGE_SENSITIVITY).max(0.0)
}

pub fn long_launch_advance_multiplier(passing_range: PassingRange) -> f64 {
    (1.0 + passing_range.value() * LONG_LAUNCH_RANGE_SENSITIVITY).max(0.0)
}