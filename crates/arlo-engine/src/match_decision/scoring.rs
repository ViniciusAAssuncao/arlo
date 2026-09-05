use crate::resolution::calculate_player_duel_rating;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{DuelContext, DuelKind, DuelOutcome};
use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE,
    FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST,
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM,
    FIELD_POINT_REQUIRED_DRIVES, FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use arlo_domain::{AttributeKey, Player, Position};
use arlo_events::ScoringPost;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScoringOpportunity {
    GoalPoint,
    FieldPoint,
    FieldGoal(ScoringPost),
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScoringDecision {
    GoalPoint {
        team_id: Uuid,
        scorer_id: Uuid,
        artrine_id: Uuid,
        drives_completed: u32,
        points: u32,
        post: ScoringPost,
    },
    FieldPoint {
        team_id: Uuid,
        scorer_id: Uuid,
        territory_advance_mirim: f64,
        drives_completed: u32,
        points: u32,
        post: ScoringPost,
    },
    FieldGoal {
        team_id: Uuid,
        scorer_id: Uuid,
        points: u32,
        post: ScoringPost,
    },
    Missed {
        team_id: Uuid,
        scorer_id: Uuid,
        attempted_post: ScoringPost,
    },
    NoOpportunity,
}

impl ScoringDecision {
    pub fn is_scored(&self) -> bool {
        matches!(
            self,
            Self::GoalPoint { .. } | Self::FieldPoint { .. } | Self::FieldGoal { .. }
        )
    }

    pub fn points(&self) -> u32 {
        match self {
            Self::GoalPoint { points, .. } => *points,
            Self::FieldPoint { points, .. } => *points,
            Self::FieldGoal { points, .. } => *points,
            Self::Missed { .. } | Self::NoOpportunity => 0,
        }
    }
}

pub fn can_attempt_goal_point(drives_in_series: u32) -> bool {
    drives_in_series >= GOAL_POINT_REQUIRED_DRIVES
}

pub fn can_attempt_field_point(drives_in_series: u32, territory_advance_mirim: f64) -> bool {
    drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
        && territory_advance_mirim >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
}

pub fn can_attempt_field_goal(territory_advance_mirim: f64) -> bool {
    territory_advance_mirim >= FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST
}

pub fn evaluate_scoring_opportunity(
    drives_in_series: u32,
    territory_advance_mirim: f64,
) -> ScoringOpportunity {
    if can_attempt_goal_point(drives_in_series) {
        ScoringOpportunity::GoalPoint
    } else if can_attempt_field_point(drives_in_series, territory_advance_mirim) {
        ScoringOpportunity::FieldPoint
    } else {
        ScoringOpportunity::None
    }
}

pub fn goal_point_points() -> u32 {
    GOAL_POINT_VALUE as u32
}

pub fn field_point_points() -> u32 {
    FIELD_POINT_VALUE as u32
}

pub fn field_goal_points(post: ScoringPost) -> u32 {
    match post {
        ScoringPost::Goalpost => FIELD_GOAL_GOALPOST_VALUE as u32,
        ScoringPost::Fieldpost => FIELD_GOAL_FIELDPOST_VALUE as u32,
    }
}

pub fn resolve_scoring_attempt<R: Rng + ?Sized>(
    finisher: &Player,
    goalguard: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    team_id: Uuid,
    artrine_id: Uuid,
    opportunity: ScoringOpportunity,
    drives_completed: u32,
    territory_advance_mirim: f64,
    context: &DuelContext,
    rng: &mut R,
) -> (ScoringDecision, DuelOutcome) {
    let (attacker_profile, defender_profile) = get_duel_profiles(DuelKind::FinishingAttempt);
    let mut attacker_rating = calculate_player_duel_rating(
        finisher,
        Position::CenterOffense,
        attribute_keys,
        &attacker_profile,
    );
    let defender_rating = calculate_player_duel_rating(
        goalguard,
        Position::Goalguard,
        attribute_keys,
        &defender_profile,
    );

    let distance_adjustment = match opportunity {
        ScoringOpportunity::GoalPoint => 0.5,
        ScoringOpportunity::FieldPoint => ((territory_advance_mirim - 8.0) * 0.25).clamp(-1.0, 1.0),
        ScoringOpportunity::FieldGoal(ScoringPost::Goalpost) => {
            ((territory_advance_mirim - 5.0) * 0.35 - 0.5).clamp(-3.0, 0.5)
        }
        ScoringOpportunity::FieldGoal(ScoringPost::Fieldpost) => {
            ((territory_advance_mirim - 3.0) * 0.35 - 1.0).clamp(-3.5, 0.0)
        }
        ScoringOpportunity::None => -5.0,
    };

    attacker_rating += distance_adjustment;

    let outcome = resolve_duel(
        DuelKind::FinishingAttempt,
        attacker_rating,
        defender_rating,
        finisher,
        goalguard,
        attribute_keys,
        context,
        rng,
    );

    let decision = if outcome.attacker_won() {
        match opportunity {
            ScoringOpportunity::GoalPoint => {
                let pts = goal_point_points();
                ScoringDecision::GoalPoint {
                    team_id,
                    scorer_id: finisher.id(),
                    artrine_id,
                    drives_completed,
                    points: pts,
                    post: ScoringPost::Goalpost,
                }
            }
            ScoringOpportunity::FieldPoint => {
                let pts = field_point_points();
                ScoringDecision::FieldPoint {
                    team_id,
                    scorer_id: finisher.id(),
                    territory_advance_mirim,
                    drives_completed,
                    points: pts,
                    post: ScoringPost::Fieldpost,
                }
            }
            ScoringOpportunity::FieldGoal(post) => {
                let pts = field_goal_points(post);
                ScoringDecision::FieldGoal {
                    team_id,
                    scorer_id: finisher.id(),
                    points: pts,
                    post,
                }
            }
            ScoringOpportunity::None => ScoringDecision::NoOpportunity,
        }
    } else {
        let attempted_post = match opportunity {
            ScoringOpportunity::GoalPoint => ScoringPost::Goalpost,
            ScoringOpportunity::FieldPoint => ScoringPost::Fieldpost,
            ScoringOpportunity::FieldGoal(post) => post,
            ScoringOpportunity::None => ScoringPost::Fieldpost,
        };
        ScoringDecision::Missed {
            team_id,
            scorer_id: finisher.id(),
            attempted_post,
        }
    };

    (decision, outcome)
}