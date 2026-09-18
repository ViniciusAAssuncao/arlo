use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::team_identity::passing_style::long_launch_advance_multiplier;
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use arlo_tactics::PassingRange;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LongLaunchActionResult {
    pub completed: bool,
    pub is_aerial: bool,
    pub advance_mirim: f64,
    pub turnover: bool,
    pub interception: bool,
    pub target_player_id: Option<Uuid>,
    pub net_advantage: f64,
    pub win_probability: Probability,
    pub next_pitch_state: PitchState,
    pub duel_outcome: DuelOutcome,
}

pub struct LongLaunchActionRequest<'a> {
    pub passer: &'a Player,
    pub passer_table: &'a PlayerAttributeTable,
    pub passer_fatigue: &'a PhysicalState,
    pub receiver: &'a Player,
    pub receiver_table: &'a PlayerAttributeTable,
    pub receiver_fatigue: &'a PhysicalState,
    pub defender: &'a Player,
    pub defender_table: &'a PlayerAttributeTable,
    pub defender_fatigue: &'a PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub duel_context: &'a DuelContext,
    pub space_rating: &'a TeamSpaceRating,
    pub pitch_state: &'a PitchState,
    pub passing_range: PassingRange,
    pub pitch_length_mirim: f64,
    pub attacker_team_power: Option<f64>,
    pub defender_team_power: Option<f64>,
}

pub fn resolve_long_launch_action<R: Rng + ?Sized>(
    request: &LongLaunchActionRequest<'_>,
    rng: &mut R,
) -> LongLaunchActionResult {
    let (att_prof, def_prof) = get_duel_profiles(DuelKind::LongDistribution);

    let passer_rating = calculate_player_duel_rating_from_table(
        request.passer,
        Position::Passer,
        request.passer_table,
        att_prof,
        request.passer_fatigue,
    );
    let defender_rating = calculate_player_duel_rating_from_table(
        request.defender,
        Position::OutsideZonerback,
        request.defender_table,
        def_prof,
        request.defender_fatigue,
    );

    let space_mod = (request.space_rating.space_index() - 0.5) * 1.5;
    let attacker_rating = (passer_rating + space_mod).max(0.1);

    let req = DuelResolutionRequest::with_states(
        DuelKind::LongDistribution,
        attacker_rating,
        defender_rating,
        request.passer,
        request.defender,
        *request.passer_fatigue,
        *request.defender_fatigue,
        request.attribute_keys,
        request.duel_context,
    )
    .with_tables(Some(request.passer_table), Some(request.defender_table))
    .with_team_powers(request.attacker_team_power, request.defender_team_power);

    let duel_outcome = resolve_duel(req, rng);
    let completed = duel_outcome.attacker_won();
    let net_advantage = duel_outcome.net_advantage();
    let win_probability = duel_outcome.win_probability();

    let pass_mult = long_launch_advance_multiplier(request.passing_range);
    let actual_advance = if completed {
        let strategy = AggregateProgressionStrategy::new(2.5, 20.0 * pass_mult, 0.60, 10.0);
        strategy.resolve_progression(&duel_outcome, rng)
    } else {
        0.0
    };

    let (turnover, interception) = if completed {
        (false, false)
    } else {
        let int_p = (logistic(-1.8 - 0.25 * net_advantage)
            * request.space_rating.pressure_intensity())
        .clamp(0.04, 0.45);
        let is_int = Probability::new_clamped(int_p).sample(rng);
        (is_int, is_int)
    };

    let next_pitch_state = request
        .pitch_state
        .with_advance(actual_advance, request.pitch_length_mirim);

    LongLaunchActionResult {
        completed,
        is_aerial: true,
        advance_mirim: actual_advance,
        turnover,
        interception,
        target_player_id: Some(request.receiver.id()),
        net_advantage,
        win_probability,
        next_pitch_state,
        duel_outcome,
    }
}