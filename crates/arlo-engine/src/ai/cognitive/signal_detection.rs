use arlo_domain::sport_constants::{ATTRIBUTE_MAX, ATTRIBUTE_MIN};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;

pub fn detection_probability(judgment_score: f64, signal_present: bool) -> Probability {
    let norm = (judgment_score.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX) - 10.0) / 10.0;
    let logit = if signal_present {
        0.5 + 1.8 * norm
    } else {
        -1.5 - 1.5 * norm
    };
    Probability::new_clamped(logistic(logit))
}

pub fn sample_detection_outcome<R: Rng + ?Sized>(
    judgment_score: f64,
    signal_present: bool,
    rng: &mut R,
) -> bool {
    detection_probability(judgment_score, signal_present).sample(rng)
}