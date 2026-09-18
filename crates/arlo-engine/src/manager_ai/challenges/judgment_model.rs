use crate::ai::cognitive::SignalDetectionModel;
use arlo_domain::sport_constants::{
    CHALLENGE_JUDGMENT_COMPOSITE_WEIGHT, JUDGING_ABILITY_COMPOSITE_WEIGHT,
};

pub fn build_challenge_judgment_model(
    challenge_judgment: f64,
    judging_ability: f64,
) -> SignalDetectionModel {
    let judgment_score = challenge_judgment * CHALLENGE_JUDGMENT_COMPOSITE_WEIGHT
        + judging_ability * JUDGING_ABILITY_COMPOSITE_WEIGHT;
    SignalDetectionModel::from_judgment_attribute(judgment_score, 0.0)
}