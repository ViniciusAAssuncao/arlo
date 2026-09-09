use crate::physical::PhysicalState;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::{logistic_slope_for, DuelKind};
use crate::resolution::duel_noise::sample_player_noise;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_side_rating_from_index_with_fatigue;
use crate::resolution::outcome::DuelOutcome;
use arlo_domain::sport_constants::HOME_FIELD_ADVANTAGE_LOGIT;
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::stats::contrast::bradley_terry_with_offset;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_duel_with_fatigue<R: Rng + ?Sized>(
    kind: DuelKind,
    attacker_rating: f64,
    defender_rating: f64,
    attacker_primary: &Player,
    defender_primary: &Player,
    attacker_state: &PhysicalState,
    defender_state: &PhysicalState,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    rng: &mut R,
) -> DuelOutcome {
    let noise_a = sample_player_noise(attacker_primary, attribute_keys, attacker_state, rng);
    let noise_b = sample_player_noise(defender_primary, attribute_keys, defender_state, rng);

    let mut hfa_logit = 0.0;
    if context.attacker_is_home() {
        hfa_logit += HOME_FIELD_ADVANTAGE_LOGIT;
    }
    if context.defender_is_home() {
        hfa_logit -= HOME_FIELD_ADVANTAGE_LOGIT;
    }
    hfa_logit += context.aggression_logit_offset();
    hfa_logit += context.physicality_logit_offset();
    hfa_logit += context.misdirection_logit_offset();

    let noisy_attacker = attacker_rating + noise_a;
    let noisy_defender = defender_rating + noise_b;
    let slope = logistic_slope_for(kind);

    let win_prob = bradley_terry_with_offset(noisy_attacker, noisy_defender, slope, hfa_logit);

    let attacker_won = win_prob.sample(rng);
    let net_advantage = attacker_rating - defender_rating;

    let outcome = DuelOutcome::new(
        kind,
        attacker_won,
        attacker_rating,
        defender_rating,
        win_prob,
        net_advantage,
    );

    crate::psychology::systems::instrumentation::instrument_duel_outcome(
        &outcome,
        attacker_primary.id(),
        defender_primary.id(),
    );

    outcome
}

pub fn resolve_duel<R: Rng + ?Sized>(
    kind: DuelKind,
    attacker_rating: f64,
    defender_rating: f64,
    attacker_primary: &Player,
    defender_primary: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    rng: &mut R,
) -> DuelOutcome {
    resolve_duel_with_fatigue(
        kind,
        attacker_rating,
        defender_rating,
        attacker_primary,
        defender_primary,
        &PhysicalState::initial(),
        &PhysicalState::initial(),
        attribute_keys,
        context,
        rng,
    )
}

pub fn resolve_duel_for_participants_with_fatigue<F, R>(
    kind: DuelKind,
    attacker_primary: &Player,
    attackers: &[(&Player, Position)],
    defender_primary: &Player,
    defenders: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> DuelOutcome
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    let (attacker_profile, defender_profile) = get_duel_profiles(kind);
    let attacker_rating = crate::resolution::group_rating::calculate_side_rating_with_fatigue(
        attackers,
        attribute_keys,
        &attacker_profile,
        fatigue_for,
    );
    let defender_rating = crate::resolution::group_rating::calculate_side_rating_with_fatigue(
        defenders,
        attribute_keys,
        &defender_profile,
        fatigue_for,
    );

    let att_state = fatigue_for(&attacker_primary.id());
    let def_state = fatigue_for(&defender_primary.id());

    resolve_duel_with_fatigue(
        kind,
        attacker_rating,
        defender_rating,
        attacker_primary,
        defender_primary,
        &att_state,
        &def_state,
        attribute_keys,
        context,
        rng,
    )
}

pub fn resolve_duel_for_participants<R: Rng + ?Sized>(
    kind: DuelKind,
    attacker_primary: &Player,
    attackers: &[(&Player, Position)],
    defender_primary: &Player,
    defenders: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    rng: &mut R,
) -> DuelOutcome {
    resolve_duel_for_participants_with_fatigue(
        kind,
        attacker_primary,
        attackers,
        defender_primary,
        defenders,
        attribute_keys,
        context,
        &|_| PhysicalState::initial(),
        rng,
    )
}

pub fn resolve_duel_for_participants_from_index_with_fatigue<F, R>(
    kind: DuelKind,
    attacker_primary: &Player,
    attackers: &[&Player],
    attacker_positions: &HashMap<Uuid, Position>,
    defender_primary: &Player,
    defenders: &[&Player],
    defender_positions: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> DuelOutcome
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    let (attacker_profile, defender_profile) = get_duel_profiles(kind);
    let attacker_rating = calculate_side_rating_from_index_with_fatigue(
        attackers,
        attacker_positions,
        attribute_keys,
        &attacker_profile,
        fatigue_for,
    );
    let defender_rating = calculate_side_rating_from_index_with_fatigue(
        defenders,
        defender_positions,
        attribute_keys,
        &defender_profile,
        fatigue_for,
    );

    let att_state = fatigue_for(&attacker_primary.id());
    let def_state = fatigue_for(&defender_primary.id());

    resolve_duel_with_fatigue(
        kind,
        attacker_rating,
        defender_rating,
        attacker_primary,
        defender_primary,
        &att_state,
        &def_state,
        attribute_keys,
        context,
        rng,
    )
}

pub fn resolve_duel_for_participants_from_index<R: Rng + ?Sized>(
    kind: DuelKind,
    attacker_primary: &Player,
    attackers: &[&Player],
    attacker_positions: &HashMap<Uuid, Position>,
    defender_primary: &Player,
    defenders: &[&Player],
    defender_positions: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    context: &DuelContext,
    rng: &mut R,
) -> DuelOutcome {
    resolve_duel_for_participants_from_index_with_fatigue(
        kind,
        attacker_primary,
        attackers,
        attacker_positions,
        defender_primary,
        defenders,
        defender_positions,
        attribute_keys,
        context,
        &|_| PhysicalState::initial(),
        rng,
    )
}