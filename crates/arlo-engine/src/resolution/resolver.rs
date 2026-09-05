use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{calculate_side_rating, calculate_side_rating_from_index};
use crate::resolution::outcome::DuelOutcome;
use arlo_domain::sport_constants::{
    CONTRAST_LOGISTIC_SLOPE, GAUSSIAN_NOISE_STD_DEV, HOME_FIELD_ADVANTAGE_LOGIT,
};
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::stats::contrast::logistic;
use arlo_math::stats::noise::sample_gaussian_noise;
use arlo_math::Probability;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_duel<R: Rng + ?Sized>(
    kind: DuelKind,
    attacker_rating: f64,
    defender_rating: f64,
    context: &DuelContext,
    rng: &mut R,
) -> DuelOutcome {
    let noise_a = sample_gaussian_noise(GAUSSIAN_NOISE_STD_DEV, rng);
    let noise_b = sample_gaussian_noise(GAUSSIAN_NOISE_STD_DEV, rng);

    let mut hfa_logit = 0.0;
    if context.attacker_is_home() {
        hfa_logit += HOME_FIELD_ADVANTAGE_LOGIT;
    }
    if context.defender_is_home() {
        hfa_logit -= HOME_FIELD_ADVANTAGE_LOGIT;
    }

    let diff = (attacker_rating + noise_a) - (defender_rating + noise_b);
    let scaled_logit = CONTRAST_LOGISTIC_SLOPE * diff + hfa_logit;
    let win_prob = Probability::new_clamped(logistic(scaled_logit));

    let attacker_won = win_prob.sample(rng);
    let net_advantage = attacker_rating - defender_rating;

    DuelOutcome::new(
        kind,
        attacker_won,
        attacker_rating,
        defender_rating,
        win_prob,
        net_advantage,
    )
}

pub fn resolve_duel_for_participants<R: Rng + ?Sized>(
    kind: DuelKind,
    attackers: &[(&Player, Position)],
    defenders: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    rng: &mut R,
) -> DuelOutcome {
    let (attacker_profile, defender_profile) = get_duel_profiles(kind);
    let attacker_rating = calculate_side_rating(attackers, attribute_keys, &attacker_profile);
    let defender_rating = calculate_side_rating(defenders, attribute_keys, &defender_profile);

    resolve_duel(kind, attacker_rating, defender_rating, context, rng)
}

pub fn resolve_duel_for_participants_from_index<R: Rng + ?Sized>(
    kind: DuelKind,
    attackers: &[&Player],
    attacker_positions: &HashMap<Uuid, Position>,
    defenders: &[&Player],
    defender_positions: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    rng: &mut R,
) -> DuelOutcome {
    let (attacker_profile, defender_profile) = get_duel_profiles(kind);
    let attacker_rating = calculate_side_rating_from_index(
        attackers,
        attacker_positions,
        attribute_keys,
        &attacker_profile,
    );
    let defender_rating = calculate_side_rating_from_index(
        defenders,
        defender_positions,
        attribute_keys,
        &defender_profile,
    );

    resolve_duel(kind, attacker_rating, defender_rating, context, rng)
}
