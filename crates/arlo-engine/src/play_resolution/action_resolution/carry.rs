use crate::ai::cognitive::RiskProfile;
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
use arlo_domain::sport_constants::ARTRO_ROW_SPACING_MIRIM;
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarryActionResult {
    pub mirins_advanced: f64,
    pub drives_crossed: u32,
    pub success: bool,
    pub turnover: bool,
    pub contact_severity: f64,
    pub net_advantage: f64,
    pub win_probability: Probability,
    pub next_pitch_state: PitchState,
    pub duel_outcome: DuelOutcome,
}

pub struct CarryActionRequest<'a> {
    pub carrier: &'a Player,
    pub carrier_table: &'a PlayerAttributeTable,
    pub carrier_fatigue: &'a PhysicalState,
    pub defender: &'a Player,
    pub defender_table: &'a PlayerAttributeTable,
    pub defender_fatigue: &'a PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub duel_context: &'a DuelContext,
    pub risk_profile: &'a RiskProfile,
    pub space_rating: &'a TeamSpaceRating,
    pub pitch_state: &'a PitchState,
    pub pitch_length_mirim: f64,
    pub is_true_artrine: bool,
    pub attacker_team_power: Option<f64>,
    pub defender_team_power: Option<f64>,
}

pub fn resolve_carry_action<R: Rng + ?Sized>(
    request: &CarryActionRequest<'_>,
    rng: &mut R,
) -> CarryActionResult {
    let duel_kind = if request.is_true_artrine {
        DuelKind::ArtroBreakthrough
    } else {
        DuelKind::RunBreakthrough
    };

    let (att_prof, def_prof) = get_duel_profiles(duel_kind);
    let base_att_rating = calculate_player_duel_rating_from_table(
        request.carrier,
        Position::CenterOffense,
        request.carrier_table,
        att_prof,
        request.carrier_fatigue,
    );
    let base_def_rating = calculate_player_duel_rating_from_table(
        request.defender,
        Position::Centerback,
        request.defender_table,
        def_prof,
        request.defender_fatigue,
    );

    let space_mod = (request.space_rating.space_index() - 0.5) * 2.0;
    let attacker_rating = (base_att_rating + space_mod).max(0.1);
    let defender_rating = base_def_rating.max(0.1);

    let req = DuelResolutionRequest::with_states(
        duel_kind,
        attacker_rating,
        defender_rating,
        request.carrier,
        request.defender,
        *request.carrier_fatigue,
        *request.defender_fatigue,
        request.attribute_keys,
        request.duel_context,
    )
    .with_tables(Some(request.carrier_table), Some(request.defender_table))
    .with_team_powers(request.attacker_team_power, request.defender_team_power);

    let duel_outcome = resolve_duel(req, rng);
    let success = duel_outcome.attacker_won();
    let net_advantage = duel_outcome.net_advantage();
    let win_probability = duel_outcome.win_probability();

    let (shape, base_mean, adv_factor, min_mean) = if success {
        (2.5, 12.0, 0.45, 4.0)
    } else {
        (2.0, 1.8, 0.15, 0.2)
    };
    let strategy = AggregateProgressionStrategy::new(shape, base_mean, adv_factor, min_mean);
    let actual_advance = strategy.resolve_progression(&duel_outcome, rng);

    let drives_crossed = if request.is_true_artrine && actual_advance >= ARTRO_ROW_SPACING_MIRIM {
        (actual_advance / ARTRO_ROW_SPACING_MIRIM).floor() as u32
    } else {
        0
    };

    let contact_severity = (0.50 - 0.05 * net_advantage).clamp(0.05, 1.0);

    let to_base = if success { -3.5 } else { -1.5 };
    let to_p = (logistic(to_base - 0.20 * net_advantage) / request.risk_profile.tolerance_index())
        .clamp(0.005, 0.45);
    let turnover = Probability::new_clamped(to_p).sample(rng);

    let next_pitch_state = request
        .pitch_state
        .with_advance(actual_advance, request.pitch_length_mirim)
        .with_drive_increment(drives_crossed);

    CarryActionResult {
        mirins_advanced: actual_advance,
        drives_crossed,
        success,
        turnover,
        contact_severity,
        net_advantage,
        win_probability,
        next_pitch_state,
        duel_outcome,
    }
}