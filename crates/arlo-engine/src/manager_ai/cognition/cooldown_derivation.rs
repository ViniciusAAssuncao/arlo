use arlo_domain::sport_constants::{
    ATTRIBUTE_MAX, ATTRIBUTE_MIN, DECISION_COOLDOWN_MAX_PERIOD_FRACTION,
    DECISION_COOLDOWN_MIN_PERIOD_FRACTION,
};

pub fn derive_cooldown_seconds(period_duration_seconds: f64, reactivity_attribute: f64) -> f64 {
    let norm = (reactivity_attribute.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX) - ATTRIBUTE_MIN)
        / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);
    let fraction = DECISION_COOLDOWN_MAX_PERIOD_FRACTION
        - norm * (DECISION_COOLDOWN_MAX_PERIOD_FRACTION - DECISION_COOLDOWN_MIN_PERIOD_FRACTION);
    fraction * period_duration_seconds
}
