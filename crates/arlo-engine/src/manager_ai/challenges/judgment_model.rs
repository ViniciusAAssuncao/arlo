use crate::ai::cognitive::SignalDetectionModel;
use arlo_domain::sport_constants::manager_cognition::{
    SIGNAL_DETECTION_BASE_SENSITIVITY, SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_domain::sport_constants::{
    CHALLENGE_JUDGMENT_COMPOSITE_WEIGHT, JUDGING_ABILITY_COMPOSITE_WEIGHT,
};

pub fn build_challenge_judgment_model(
    challenge_judgment: f64,
    judging_ability: f64,
) -> SignalDetectionModel {
    let judgment_score = challenge_judgment * CHALLENGE_JUDGMENT_COMPOSITE_WEIGHT
        + judging_ability * JUDGING_ABILITY_COMPOSITE_WEIGHT;
    let d_prime = SIGNAL_DETECTION_BASE_SENSITIVITY
        + judgment_score * SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE;
    SignalDetectionModel::new(d_prime, 0.0)
}