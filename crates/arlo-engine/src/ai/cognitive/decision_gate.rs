use arlo_domain::sport_constants::{ATTRIBUTE_MAX, ATTRIBUTE_MIN};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;

pub fn evaluate_decision_gate(stimulus: f64, manager_attribute: f64, discipline: f64) -> Probability {
    let s = stimulus.clamp(0.0, 1.0);
    if s <= 1e-4 {
        return Probability::new_clamped(0.0);
    }

    let norm_attr = (manager_attribute.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX) - ATTRIBUTE_MIN)
        / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);
    let norm_disc = (discipline.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX) - ATTRIBUTE_MIN)
        / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);

    let sensitivity = 3.0 + 3.0 * norm_attr;
    let threshold = 0.65 - 0.25 * norm_attr;
    let discipline_dampening = 0.85 + 0.15 * norm_disc;

    let logit = sensitivity * (s - threshold);
    let raw_prob = (logistic(logit) * discipline_dampening).clamp(0.0, 1.0);
    let gated_prob = raw_prob * s.powf(1.8);

    Probability::new_clamped(gated_prob)
}