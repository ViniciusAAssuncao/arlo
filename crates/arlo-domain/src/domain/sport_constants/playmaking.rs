use crate::domain::sport_constants::ability::{ATTRIBUTE_MAX, ATTRIBUTE_MIN};

pub const ARTRINE_DECISION_LOGIT_STEEPNESS: f64 = 0.25;
pub const RANGE_UTILITY_SCALE: f64 = 2.0;
pub const DRIVES_NEEDED_UTILITY_SCALE: f64 = 2.5;
pub const DOWN_PRESSURE_UTILITY_SCALE: f64 = 1.5;
pub const PRESSURE_READ_UTILITY_SCALE: f64 = 1.8;
pub const LAST_DOWN_DESPERATION_UTILITY_SCALE: f64 = 3.0;

pub fn decision_steepness_for(decisions_attribute: f64) -> f64 {
    let normalized = (decisions_attribute.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX) - ATTRIBUTE_MIN)
        / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);
    0.10 + (normalized * 0.30)
}