use crate::domain::sport_constants::ability::{ATTRIBUTE_MAX, ATTRIBUTE_MIN};

pub const GOALGUARD_RX_MIN: f64 = 0.00;
pub const GOALGUARD_RX_MAX: f64 = 0.05;

pub const DEFENSE_RX_MIN: f64 = 0.08;
pub const DEFENSE_RX_MAX: f64 = 0.35;

pub const BACK_RX_MIN: f64 = 0.35;
pub const BACK_RX_MAX: f64 = 0.65;

pub const OFFENSIVE_RX_MIN: f64 = 0.65;
pub const OFFENSIVE_RX_MAX: f64 = 0.98;

pub const TACTICAL_STYLE_LOGIT_SCALE: f64 = 0.2;

pub const SHORT_PASS_RANGE_SENSITIVITY: f64 = 0.35;
pub const LONG_LAUNCH_RANGE_SENSITIVITY: f64 = 0.35;
pub const SHORT_PASS_BASE_ADVANCE_MIRIM: f64 = 4.0;
pub const SHORT_PASS_MIN_ADVANCE_MIRIM: f64 = 1.0;
pub const LONG_LAUNCH_BASE_ADVANCE_MIRIM: f64 = 12.0;
pub const LONG_LAUNCH_MIN_ADVANCE_MIRIM: f64 = 3.0;

pub const BLOCK_MARKING_PROXIMITY_WEIGHT: f64 = 0.6;
pub const BLOCK_MARKING_AGGRESSION_WEIGHT: f64 = 0.4;
pub const BLOCK_MARKING_BITE_TRACKING_BOOST: f64 = 0.85;
pub const BLOCK_MARKING_COVER_DISCIPLINE_BOOST: f64 = 0.5;

pub const ARTRINE_DECISION_LOGIT_STEEPNESS: f64 = 0.25;
pub const RANGE_UTILITY_SCALE: f64 = 2.0;
pub const DRIVES_NEEDED_UTILITY_SCALE: f64 = 2.5;
pub const DOWN_PRESSURE_UTILITY_SCALE: f64 = 1.5;
pub const PRESSURE_READ_UTILITY_SCALE: f64 = 1.8;
pub const LAST_DOWN_DESPERATION_UTILITY_SCALE: f64 = 3.0;
pub const MAN_COVERAGE_OPENNESS_PENALTY: f64 = 0.25;
pub const LAUNCHER_TARGET_WEIGHT_MULTIPLIER: f64 = 1.35;

pub fn decision_steepness_for(decisions_attribute: f64) -> f64 {
    let normalized = (decisions_attribute.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX) - ATTRIBUTE_MIN)
        / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);
    0.10 + (normalized * 0.30)
}