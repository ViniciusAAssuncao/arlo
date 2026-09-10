use crate::resolution::DuelOutcome;
use arlo_math::stats::uncertainty_from_probability;
use arlo_math::Probability;

pub fn ambiguity_from_duel_outcome(outcome: &DuelOutcome) -> Probability {
    uncertainty_from_probability(outcome.win_probability())
}
