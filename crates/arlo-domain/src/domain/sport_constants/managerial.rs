use crate::domain::sport_constants::ability::{ATTRIBUTE_MAX, ATTRIBUTE_MIN};

pub const ADAPTABILITY_FLEXIBILITY_WEIGHT: f64 = 0.65;
pub const BASE_FLEXIBILITY_WEIGHT: f64 = 0.35;
pub const TACTICAL_KNOWLEDGE_SYSTEM_BONUS: f64 = 0.15;
pub const IN_GAME_ADJUSTMENT_TRIGGER_THRESHOLD: f64 = 0.60;
pub const IN_GAME_ADJUSTMENT_MAX_IMPACT: f64 = 0.25;
pub const ARTRINE_COMMUNICATION_EFFICIENCY_SCALE: f64 = 0.30;
pub const TIME_CALL_DECISION_WEIGHT: f64 = 0.40;
pub const CHALLENGE_JUDGMENT_ACCURACY_SCALE: f64 = 0.50;
pub const ROTATION_POLICY_FATIGUE_THRESHOLD_STRICT: f64 = 0.85;
pub const ROTATION_POLICY_FATIGUE_THRESHOLD_SITUATIONAL: f64 = 0.70;
pub const ROTATION_POLICY_FATIGUE_THRESHOLD_HIGH: f64 = 0.55;
pub const MAX_PREFERRED_FORMATIONS: usize = 3;
pub const LAUNCHER_PASSING_RANGE_PREFERENCE_WEIGHT: f64 = 0.25;
pub const BLOCKER_ROLE_BASE_THRESHOLD: f64 = 14.0;
pub const BLOCKER_ROLE_PHYSICALITY_ADJUSTMENT: f64 = 4.0;

pub fn effective_manager_flexibility(
    adaptability_attribute: f64,
    flexibility_tendency: f64,
) -> f64 {
    let normalized_adaptability = (adaptability_attribute.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX)
        - ATTRIBUTE_MIN)
        / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);
    let raw = (normalized_adaptability * ADAPTABILITY_FLEXIBILITY_WEIGHT)
        + (flexibility_tendency.clamp(0.0, 1.0) * BASE_FLEXIBILITY_WEIGHT);
    raw.clamp(0.0, 1.0)
}