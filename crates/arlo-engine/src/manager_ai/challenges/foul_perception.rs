use crate::ai::cognitive::signal_detection::{sample_detection_outcome, SignalDetectionModel};
use arlo_domain::sport_constants::manager_cognition::{
    SIGNAL_DETECTION_BASE_SENSITIVITY, SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use rand::Rng;

pub fn perceives_bad_foul_call<R: Rng + ?Sized>(
    original_call_correct: bool,
    challenge_judgment: f64,
    judging_ability: f64,
    rng: &mut R,
) -> bool {
    let judgment_score = challenge_judgment * 0.6 + judging_ability * 0.4;
    let d_prime = SIGNAL_DETECTION_BASE_SENSITIVITY
        + judgment_score * SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE;
    let model = SignalDetectionModel::new(d_prime, 0.0);
    let signal_present = !original_call_correct;
    sample_detection_outcome(&model, signal_present, rng)
}