use arlo_events::ScoringPost;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScoringKind {
    GoalPoint,
    FieldPoint,
    FieldGoal(ScoringPost),
}
