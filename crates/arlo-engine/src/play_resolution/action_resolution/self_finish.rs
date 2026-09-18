use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::match_decision::scoring::{
    duel_kind_for_opportunity,
    evaluate_scoring_opportunity,
    field_goal_points,
    field_point_points,
    goal_point_points,
    ScoringOpportunity,
};
use crate::physical::PhysicalState;
use crate::possession::PitchState;
use crate::resolution::context::DuelContext;
use crate::resolution::finish_distance_multiplier;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::resolver::{ resolve_duel, DuelResolutionRequest };
use crate::resolution::DuelKind;
use arlo_domain::{ AttributeKey, Player, Position };
use arlo_events::ScoringPost;
use arlo_math::Probability;
use rand::Rng;
use serde::{ Deserialize, Serialize };
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
    pub finisher_table: &'a crate::attributes::PlayerAttributeTable,
    pub finisher_fatigue: &'a PhysicalState,
    pub goalguard: &'a Player,
    pub goalguard_table: &'a crate::attributes::PlayerAttributeTable,
    pub goalguard_fatigue: &'a PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub duel_context: &'a DuelContext,
    pub pitch_state: &'a PitchState,
    pub is_home_offense: bool,
    pub pitch_length_mirim: f64,
    pub attacker_team_power: Option<f64>,
    pub defender_team_power: Option<f64>,
}

pub fn resolve_self_finish_action<R: Rng + ?Sized>(
    request: &SelfFinishActionRequest<'_>,
    rng: &mut R
) -> FinishActionResult {
    let (att_prof, def_prof) = get_duel_profiles(DuelKind::FinishingAttempt);

    let raw_fin_rating = calculate_player_duel_rating_from_table(
        request.finisher,
        Position::CenterOffense,
        request.finisher_table,
        &att_prof,
        request.finisher_fatigue
    );
    let raw_gg_rating = calculate_player_duel_rating_from_table(
        request.goalguard,
        Position::Goalguard,
        request.goalguard_table,
        &def_prof,
        request.goalguard_fatigue
    );

    let territory_advance = (10.0 - request.pitch_state.remaining_advance_mirim()).max(0.0);
    let opportunity = evaluate_scoring_opportunity(
        request.pitch_state.is_bonus_phase(),
        request.pitch_state.drives_in_series(),
        territory_advance,
        request.pitch_state.normalized_proximity(),
        raw_fin_rating
    );

    if opportunity == ScoringOpportunity::None {
        let duel_outcome = DuelOutcome::new(
            DuelKind::FinishingAttempt,
            false,
            raw_fin_rating,
            raw_gg_rating,
            Probability::new_clamped(0.0),
            -5.0
        );
        return FinishActionResult {
            scored: false,
            opportunity: ScoringOpportunity::None,
            post: ScoringPost::Fieldpost,
            points_awarded: 0,
            win_probability: Probability::new_clamped(0.0),
            net_advantage: -5.0,
            next_pitch_state: request.pitch_state.with_advance(0.0, request.pitch_length_mirim),
            duel_outcome,
        };
    }

    let duel_kind = duel_kind_for_opportunity(opportunity);
    let distance_multiplier = finish_distance_multiplier(
        request.pitch_state.normalized_proximity()
    );
    let effective_fin_rating = raw_fin_rating * distance_multiplier;

    let req = DuelResolutionRequest::with_states(
        duel_kind,
        effective_fin_rating,
        raw_gg_rating,
        request.finisher,
        request.goalguard,
        *request.finisher_fatigue,
        *request.goalguard_fatigue,
        request.attribute_keys,
        request.duel_context
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
            0.5,
            matches!(opportunity, ScoringOpportunity::GoalPoint)
        )
    } else {
        request.pitch_state.with_advance(0.0, request.pitch_length_mirim)
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
