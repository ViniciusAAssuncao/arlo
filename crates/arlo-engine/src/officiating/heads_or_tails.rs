use crate::ai::cognitive::action_probability;
use crate::attributes::RefereeAttributeTable;
use arlo_domain::sport_constants::{
    PEACE_REFEREE_AUTHORITY_SCALE, PEACE_REFEREE_BASE_SENSITIVITY,
    PEACE_REFEREE_INTERVENTION_STEEPNESS,
};
use arlo_domain::AttributeKey;
use arlo_math::Probability;
use rand::Rng;

pub fn flip_officiating_coin<R: Rng + ?Sized>(rng: &mut R) -> bool {
    Probability::new_clamped(0.5).sample(rng)
}

pub fn resolve_peace_referee_review<R: Rng + ?Sized>(
    stimulus: f64,
    peace_referee_table: &RefereeAttributeTable,
    rng: &mut R,
) -> (bool, bool) {
    let original_call_correct = flip_officiating_coin(rng);
    let intervention_prob = action_probability(
        stimulus,
        peace_referee_table.get(AttributeKey::Authority),
        PEACE_REFEREE_BASE_SENSITIVITY,
        PEACE_REFEREE_AUTHORITY_SCALE,
        PEACE_REFEREE_INTERVENTION_STEEPNESS,
    );
    let peace_referee_intervened = intervention_prob.sample(rng);
    (original_call_correct, peace_referee_intervened)
}
