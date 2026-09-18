use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{sample_action_progression, ActionProgressionKind};
use arlo_domain::{ArtroPlacement, AttributeKey, PitchZone, Player, Position};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossActionResult {
    pub completed: bool,
    pub target_player_id: Option<Uuid>,
    pub target_zone: PitchZone,
    pub turnover: bool,
    pub scoring_attempt_ready: bool,
    pub net_advantage: f64,
    pub win_probability: Probability,
    pub next_pitch_state: PitchState,
    pub duel_outcome: DuelOutcome,
}

pub struct CrossActionRequest<'a> {
    pub crosser: &'a Player,
    pub crosser_table: &'a PlayerAttributeTable,
    pub crosser_fatigue: &'a PhysicalState,
    pub target_finisher: &'a Player,
    pub target_finisher_table: &'a PlayerAttributeTable,
    pub target_finisher_fatigue: &'a PhysicalState,
    pub defender: &'a Player,
    pub defender_table: &'a PlayerAttributeTable,
    pub defender_fatigue: &'a PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub duel_context: &'a DuelContext,
    pub space_rating: &'a TeamSpaceRating,
    pub pitch_state: &'a PitchState,
    pub pitch_length_mirim: f64,
    pub attacker_team_power: Option<f64>,
    pub defender_team_power: Option<f64>,
}

pub fn resolve_cross_action<R: Rng + ?Sized>(
    request: &CrossActionRequest<'_>,
    rng: &mut R,
) -> CrossActionResult {
    let (att_prof, def_prof) = get_duel_profiles(DuelKind::CrossDistribution);

    let crosser_rating = calculate_player_duel_rating_from_table(
        request.crosser,
        Position::WingOffense,
        request.crosser_table,
        att_prof,
        request.crosser_fatigue,
    );
    let defender_rating = calculate_player_duel_rating_from_table(
        request.defender,
        Position::OutsideZonerback,
        request.defender_table,
        def_prof,
        request.defender_fatigue,
    );

    let channel_bonus = match request.pitch_state.channel() {
        ArtroPlacement::LeftLateral | ArtroPlacement::RightLateral => 0.5,
        ArtroPlacement::Central => -0.2,
    };
    let flank_mod = (request.space_rating.flank_openness() - 0.5) * 1.5;
    let attacker_rating = (crosser_rating + channel_bonus + flank_mod).max(0.1);

    let req = DuelResolutionRequest::with_states(
        DuelKind::CrossDistribution,
        attacker_rating,
        defender_rating,
        request.crosser,
        request.defender,
        *request.crosser_fatigue,
        *request.defender_fatigue,
        request.attribute_keys,
        request.duel_context,
    )
    .with_tables(Some(request.crosser_table), Some(request.defender_table))
    .with_team_powers(request.attacker_team_power, request.defender_team_power);

    let duel_outcome = resolve_duel(req, rng);
    let completed = duel_outcome.attacker_won();
    let net_advantage = duel_outcome.net_advantage();
    let win_probability = duel_outcome.win_probability();

    let actual_advance = if completed {
        sample_action_progression(
            ActionProgressionKind::Cross,
            net_advantage,
            1.0,
            rng,
        )
    } else {
        0.0
    };

    let target_zone = PitchZone::FirstZone;
    let (turnover, scoring_attempt_ready) = if completed {
        (false, true)
    } else {
        let to_p = (logistic(-1.2 - 0.20 * net_advantage)
            * request.space_rating.pressure_intensity())
        .clamp(0.05, 0.50);
        (Probability::new_clamped(to_p).sample(rng), false)
    };

    let next_pitch_state = request
        .pitch_state
        .with_advance(actual_advance, request.pitch_length_mirim)
        .with_channel(ArtroPlacement::Central);

    CrossActionResult {
        completed,
        target_player_id: Some(request.target_finisher.id()),
        target_zone,
        turnover,
        scoring_attempt_ready,
        net_advantage,
        win_probability,
        next_pitch_state,
        duel_outcome,
    }
}