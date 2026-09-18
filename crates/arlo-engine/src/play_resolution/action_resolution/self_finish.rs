use crate::attributes::PlayerAttributeTable;
use crate::match_decision::scoring::{
    duel_kind_for_opportunity, field_goal_points, field_point_points, goal_point_points,
    ScoringOpportunity,
};
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use arlo_domain::sport_constants::GOAL_POINT_REQUIRED_DRIVES;
use arlo_domain::{AttributeKey, PitchZone, Player, Position};
use arlo_events::ScoringPost;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishActionResult {
    pub scored: bool,
    pub opportunity: ScoringOpportunity,
    pub post: ScoringPost,
    pub points_awarded: u32,
    pub win_probability: Probability,
    pub net_advantage: f64,
    pub next_pitch_state: PitchState,
    pub duel_outcome: DuelOutcome,
}

pub struct SelfFinishActionRequest<'a> {
    pub finisher: &'a Player,
    pub finisher_table: &'a PlayerAttributeTable,
    pub finisher_fatigue: &'a PhysicalState,
    pub goalguard: &'a Player,
    pub goalguard_table: &'a PlayerAttributeTable,
    pub goalguard_fatigue: &'a PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub duel_context: &'a DuelContext,
    pub space_rating: &'a TeamSpaceRating,
    pub pitch_state: &'a PitchState,
    pub is_home_offense: bool,
    pub pitch_length_mirim: f64,
    pub attacker_team_power: Option<f64>,
    pub defender_team_power: Option<f64>,
}

pub fn resolve_self_finish_action<R: Rng + ?Sized>(
    request: &SelfFinishActionRequest<'_>,
    rng: &mut R,
) -> FinishActionResult {
    let opportunity = if request.pitch_state.is_bonus_phase() {
        ScoringOpportunity::FieldGoal(ScoringPost::Goalpost)
    } else if request.pitch_state.drives_in_series() >= GOAL_POINT_REQUIRED_DRIVES {
        ScoringOpportunity::GoalPoint
    } else {
        ScoringOpportunity::FieldPoint
    };

    let duel_kind = duel_kind_for_opportunity(opportunity);
    let (att_prof, def_prof) = get_duel_profiles(duel_kind);

    let raw_fin_rating = calculate_player_duel_rating_from_table(
        request.finisher,
        Position::CenterOffense,
        request.finisher_table,
        att_prof,
        request.finisher_fatigue,
    );
    let raw_gg_rating = calculate_player_duel_rating_from_table(
        request.goalguard,
        Position::Goalguard,
        request.goalguard_table,
        def_prof,
        request.goalguard_fatigue,
    );

    let zone_multiplier = match request.pitch_state.zone() {
        PitchZone::FirstZone => 1.85,
        PitchZone::SecondZone => 1.45,
        _ => 0.90,
    };
    let effective_fin_rating =
        raw_fin_rating * zone_multiplier * request.space_rating.lane_clearance();

    let req = DuelResolutionRequest::with_states(
        duel_kind,
        effective_fin_rating,
        raw_gg_rating,
        request.finisher,
        request.goalguard,
        *request.finisher_fatigue,
        *request.goalguard_fatigue,
        request.attribute_keys,
        request.duel_context,
    )
    .with_tables(Some(request.finisher_table), Some(request.goalguard_table))
    .with_team_powers(request.attacker_team_power, request.defender_team_power);

    let duel_outcome = resolve_duel(req, rng);
    let scored = duel_outcome.attacker_won();
    let win_probability = duel_outcome.win_probability();
    let net_advantage = duel_outcome.net_advantage();

    let post = match opportunity {
        ScoringOpportunity::GoalPoint | ScoringOpportunity::FieldGoal(ScoringPost::Goalpost) => {
            ScoringPost::Goalpost
        }
        _ => ScoringPost::Fieldpost,
    };

    let points_awarded = if scored {
        match opportunity {
            ScoringOpportunity::GoalPoint => goal_point_points(),
            ScoringOpportunity::FieldPoint => field_point_points(),
            ScoringOpportunity::FieldGoal(p) => field_goal_points(p),
            ScoringOpportunity::None => 0,
        }
    } else {
        0
    };

    let next_pitch_state = if scored {
        request.pitch_state.reset_for_new_series(
            0.50,
            matches!(opportunity, ScoringOpportunity::GoalPoint),
        )
    } else {
        request
            .pitch_state
            .with_advance(0.0, request.pitch_length_mirim)
    };

    FinishActionResult {
        scored,
        opportunity,
        post,
        points_awarded,
        win_probability,
        net_advantage,
        next_pitch_state,
        duel_outcome,
    }
}