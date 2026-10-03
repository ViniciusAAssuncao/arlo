use super::TacticalAssignment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticalPlanActivated {
    pub team_id: Uuid,
    pub plan_id: Uuid,
    pub plan_name: String,
    pub formation_id: Uuid,
    pub profile_id: Uuid,
    pub assignments: Vec<TacticalAssignment>,
}
