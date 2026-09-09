use arlo_domain::sport_constants::{ATTRIBUTE_MAX, ATTRIBUTE_MIN};
use arlo_math::stats::{logistic_scaled, Probability};

pub fn action_probability(
    stimulus: f64,
    attribute_value: f64,
    stimulus_weight: f64,
    attribute_weight: f64,
    steepness: f64,
) -> Probability {
    let norm_stimulus = stimulus.clamp(0.0, 1.0);
    let norm_attr = (attribute_value.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX) - ATTRIBUTE_MIN)
        / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);
    let total_weight = stimulus_weight + attribute_weight;
    let score = if total_weight > 0.0 {
        (norm_stimulus * stimulus_weight + norm_attr * attribute_weight) / total_weight
    } else {
        0.0
    };
    let prob = logistic_scaled(score - 0.5, steepness);
    Probability::new_clamped(prob)
}