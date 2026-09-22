use crate::attributes::RefereeAttributeTable;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use arlo_domain::sport_constants::{
    PEACE_REFEREE_AUTHORITY_SCALE, PEACE_REFEREE_BASE_SENSITIVITY,
    PEACE_REFEREE_INTERVENTION_STEEPNESS,
};
use arlo_domain::AttributeKey;
use rand::Rng;

const PEACE_REFEREE_INTERVENTION_THRESHOLD: f64 = 0.65;

pub fn flip_officiating_coin<R: Rng + ?Sized>(rng: &mut R) -> bool {
    let context = DuelContext::neutral();
    let req = DuelResolutionRequest::for_contest(
        DuelKind::PassProtection,
        10.0,
        10.0,
        &context,
    )
    .with_slope(0.25);
    resolve_duel(req, rng).attacker_won()
}

pub fn resolve_peace_referee_review<R: Rng + ?Sized>(
    stimulus: f64,
    peace_referee_table: &RefereeAttributeTable,
    rng: &mut R,
) -> (bool, bool) {
    let original_call_correct = flip_officiating_coin(rng);

    if stimulus < PEACE_REFEREE_INTERVENTION_THRESHOLD {
        return (original_call_correct, false);
    }

    let authority = peace_referee_table.get(AttributeKey::Authority);
    let stimulus_drive = stimulus.clamp(0.0, 1.0) * PEACE_REFEREE_BASE_SENSITIVITY * 10.0;
    let authority_bonus = authority * PEACE_REFEREE_AUTHORITY_SCALE * 5.0;
    let attacker_rating = stimulus_drive + authority_bonus;
    let defender_rating = 10.0;

    let context = DuelContext::neutral();
    let req = DuelResolutionRequest::for_contest(
        DuelKind::PassProtection,
        attacker_rating,
        defender_rating,
        &context,
    )
    .with_slope(PEACE_REFEREE_INTERVENTION_STEEPNESS);

    let peace_referee_intervened = resolve_duel(req, rng).attacker_won();

    (original_call_correct, peace_referee_intervened)
}