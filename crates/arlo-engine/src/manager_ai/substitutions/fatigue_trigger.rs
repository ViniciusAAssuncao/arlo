use crate::physical::FatigueState;
use arlo_domain::sport_constants::managerial::{
    ROTATION_POLICY_FATIGUE_THRESHOLD_HIGH, ROTATION_POLICY_FATIGUE_THRESHOLD_SITUATIONAL,
    ROTATION_POLICY_FATIGUE_THRESHOLD_STRICT,
};
use arlo_domain::sport_constants::substitution::SUBSTITUTION_LOAD_MANAGEMENT_PROACTIVITY_SCALE;
use arlo_domain::RotationPolicy;

pub fn threshold_for_policy(policy: RotationPolicy, load_management: f64) -> f64 {
    let base_threshold = match policy {
        RotationPolicy::StrictCore => ROTATION_POLICY_FATIGUE_THRESHOLD_HIGH,
        RotationPolicy::Situational => ROTATION_POLICY_FATIGUE_THRESHOLD_SITUATIONAL,
        RotationPolicy::HighRotation => ROTATION_POLICY_FATIGUE_THRESHOLD_STRICT,
    };
    let normalized_lm = (load_management.clamp(0.0, 20.0)) / 20.0;
    let proactivity_boost = normalized_lm * SUBSTITUTION_LOAD_MANAGEMENT_PROACTIVITY_SCALE;
    (base_threshold + proactivity_boost).clamp(0.0, 1.0)
}

pub fn urgency_for_player(state: &FatigueState, rotation_policy: RotationPolicy) -> f64 {
    urgency_for_player_with_load_management(state, rotation_policy, 10.0)
}

pub fn urgency_for_player_with_load_management(
    state: &FatigueState,
    rotation_policy: RotationPolicy,
    load_management: f64,
) -> f64 {
    let threshold = threshold_for_policy(rotation_policy, load_management);
    let w_bal = state.w_prime_balance();
    if w_bal < threshold {
        ((threshold - w_bal) / threshold).clamp(0.0, 1.0)
    } else {
        0.0
    }
}
