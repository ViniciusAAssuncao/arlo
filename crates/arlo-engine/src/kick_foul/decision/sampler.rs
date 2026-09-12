use crate::attributes::PlayerAttributeTable;
use arlo_domain::sport_constants::decision_steepness_for;
use arlo_domain::{AttributeKey, KickFoulDecisionKind};
use arlo_math::stats::{sample_categorical, softmax_weights};
use rand::Rng;

pub fn sample_kick_foul_decision<R: Rng + ?Sized>(
    kicker_table: &PlayerAttributeTable,
    utilities: &[(KickFoulDecisionKind, f64)],
    rng: &mut R,
) -> KickFoulDecisionKind {
    if utilities.is_empty() {
        return KickFoulDecisionKind::Shoot;
    }
    if utilities.len() == 1 {
        return utilities[0].0;
    }

    let steepness = decision_steepness_for(kicker_table.get(AttributeKey::Decisions));
    let values: Vec<f64> = utilities.iter().map(|(_, u)| *u).collect();
    let weights = softmax_weights(&values, steepness);
    let chosen_index = sample_categorical(&weights, rng).unwrap_or(0);
    utilities[chosen_index].0
}