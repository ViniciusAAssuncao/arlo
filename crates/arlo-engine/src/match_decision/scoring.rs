use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM,
    FIELD_POINT_REQUIRED_DRIVES, FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use arlo_events::ScoringPost;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScoringOpportunity {
    GoalPoint,
    FieldPoint,
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
