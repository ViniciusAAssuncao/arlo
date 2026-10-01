use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TieBreakCriterion {
    IspaTotal,
    QtaScore,
    GoalDifference,
    GoalPointsTotal,
    HeadToHead,
    Random,
}
