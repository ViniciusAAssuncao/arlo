use crate::instructions::axes::{
    Aeriality, Directness, Mentality, PassingRange, Physicality, ScoringPatience, Width,
};
use crate::playcall::decision_emphasis::DecisionEmphasis;
use crate::playcall::decision_emphasis_defaults_constants::*;
use arlo_math::stats::contrast::logistic_scaled;

pub fn derive_default_decision_emphasis(
    mentality: Mentality,
    directness: Directness,
    width: Width,
    passing_range: PassingRange,
    aeriality: Aeriality,
    physicality: Physicality,
    scoring_patience: ScoringPatience,
) -> DecisionEmphasis {
    let util_self_carry = (physicality.value() - 0.5) * SELF_CARRY_PHYSICALITY_SENSITIVITY
        - aeriality.value() * SELF_CARRY_AERIALITY_SENSITIVITY
        - passing_range.value() * SELF_CARRY_PASSING_RANGE_SENSITIVITY
        - directness.value() * SELF_CARRY_DIRECTNESS_SENSITIVITY;

    let util_short_pass = -passing_range.value() * SHORT_PASS_PASSING_RANGE_SENSITIVITY
        - width.value() * SHORT_PASS_NARROW_WIDTH_SENSITIVITY
        - directness.value() * SHORT_PASS_DIRECTNESS_SENSITIVITY;

    let util_long_launch = passing_range.value() * LONG_LAUNCH_PASSING_RANGE_SENSITIVITY
        + directness.value() * LONG_LAUNCH_DIRECTNESS_SENSITIVITY
        + mentality.value() * LONG_LAUNCH_MENTALITY_SENSITIVITY;

    let util_cross = aeriality.value() * CROSS_AERIALITY_SENSITIVITY
        + width.value() * CROSS_WIDE_WIDTH_SENSITIVITY
        + passing_range.value() * CROSS_PASSING_RANGE_SENSITIVITY;

    let util_self_finish = mentality.value() * SELF_FINISH_MENTALITY_SENSITIVITY
        + (0.5 - scoring_patience.value()) * SELF_FINISH_IMPATIENCE_SENSITIVITY;

    let self_carry = logistic_scaled(util_self_carry, DECISION_EMPHASIS_LOGIT_STEEPNESS);
    let short_pass = logistic_scaled(util_short_pass, DECISION_EMPHASIS_LOGIT_STEEPNESS);
    let long_launch = logistic_scaled(util_long_launch, DECISION_EMPHASIS_LOGIT_STEEPNESS);
    let cross = logistic_scaled(util_cross, DECISION_EMPHASIS_LOGIT_STEEPNESS);
    let self_finish = logistic_scaled(util_self_finish, DECISION_EMPHASIS_LOGIT_STEEPNESS);

    DecisionEmphasis::new_clamped(self_carry, short_pass, long_launch, cross, self_finish)
}
