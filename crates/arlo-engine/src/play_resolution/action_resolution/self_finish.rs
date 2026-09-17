use crate::attributes::PlayerAttributeTable;
use crate::match_decision::scoring::{
    field_goal_points, field_point_points, goal_point_points, ScoringOpportunity,
};
use crate::physical::PhysicalState;
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::space_index::TeamSpaceRating;
use arlo_domain::sport_constants::GOAL_POINT_REQUIRED_DRIVES;
use arlo_domain::{AttributeKey, PitchZone};
use arlo_events::ScoringPost;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinishActionResult {
    pub scored: bool,
    pub opportunity: ScoringOpportunity,
    pub post: ScoringPost,
    pub points_awarded: u32,
    pub win_probability: Probability,
    pub net_advantage: f64,
    pub next_pitch_state: PitchState,
}

pub fn resolve_self_finish_action<R: Rng + ?Sized>(
    finisher_table: &PlayerAttributeTable,
    finisher_fatigue: &PhysicalState,
    goalguard_table: &PlayerAttributeTable,
    goalguard_fatigue: &PhysicalState,
    space_rating: &TeamSpaceRating,
    pitch_state: &PitchState,
    is_home_offense: bool,
    pitch_length_mirim: f64,
    rng: &mut R,
) -> FinishActionResult {
    let finishing = finisher_table.get(AttributeKey::Finishing);
    let technique = finisher_table.get(AttributeKey::Technique);
    let composure = finisher_table.get(AttributeKey::Composure);
    let anticipation = finisher_table.get(AttributeKey::Anticipation);

    let finisher_fatigue_mod = finisher_fatigue.energy().clamp(0.5, 1.0);
    let raw_fin_rating = (finishing * 0.45 + technique * 0.25 + composure * 0.20 + anticipation * 0.10)
        * finisher_fatigue_mod;

    let reflexes = goalguard_table.get(AttributeKey::Reflexes);
    let positioning = goalguard_table.get(AttributeKey::Positioning);
    let handling = goalguard_table.get(AttributeKey::Handling);
    let agility = goalguard_table.get(AttributeKey::Agility);

    let gg_fatigue_mod = goalguard_fatigue.energy().clamp(0.5, 1.0);
    let raw_gg_rating =
        (reflexes * 0.40 + positioning * 0.30 + handling * 0.20 + agility * 0.10) * gg_fatigue_mod;

    let zone_multiplier = match pitch_state.zone() {
        PitchZone::FirstZone => 1.85,
        PitchZone::SecondZone => 1.45,
        _ => 0.90,
    };

    let opportunity = if pitch_state.is_bonus_phase() {
        ScoringOpportunity::FieldGoal(ScoringPost::Goalpost)
    } else if pitch_state.drives_in_series() >= GOAL_POINT_REQUIRED_DRIVES {
        ScoringOpportunity::GoalPoint
    } else {
        ScoringOpportunity::FieldPoint
    };

    let post = match opportunity {
        ScoringOpportunity::GoalPoint
        | ScoringOpportunity::FieldGoal(ScoringPost::Goalpost) => ScoringPost::Goalpost,
        _ => ScoringPost::Fieldpost,
    };

    let effective_fin_rating = raw_fin_rating * zone_multiplier * space_rating.lane_clearance();
    let net_advantage = effective_fin_rating - raw_gg_rating;

    let attack_strength = effective_fin_rating.max(0.1);
    let defense_strength = raw_gg_rating.max(0.1);
    let total_strength = attack_strength + defense_strength;
    let base_prob = attack_strength / total_strength;

    let hfa = if is_home_offense { 0.03 } else { -0.03 };
    let win_prob_val = (base_prob + hfa).clamp(0.05, 0.95);
    let win_probability = Probability::new_clamped(win_prob_val);
    let scored = win_probability.sample(rng);

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
        pitch_state.reset_for_new_series(
            0.50,
            matches!(opportunity, ScoringOpportunity::GoalPoint),
        )
    } else {
        pitch_state.with_advance(0.0, pitch_length_mirim)
    };

    FinishActionResult {
        scored,
        opportunity,
        post,
        points_awarded,
        win_probability,
        net_advantage,
        next_pitch_state,
    }
}