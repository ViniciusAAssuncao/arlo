use crate::ai::cognitive::signal_detection::sample_detection_outcome;
use crate::manager_ai::challenges::judgment_model::build_challenge_judgment_model;
use rand::Rng;

pub fn perceives_bad_foul_call<R: Rng + ?Sized>(
    original_call_correct: bool,
    challenge_judgment: f64,
    judging_ability: f64,
    rng: &mut R,
) -> bool {
    let model = build_challenge_judgment_model(challenge_judgment, judging_ability);
    let signal_present = !original_call_correct;
    sample_detection_outcome(&model, signal_present, rng)
}