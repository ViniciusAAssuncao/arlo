use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreparedPlanIntent {
    plan_id: Uuid,
}

impl PreparedPlanIntent {
    pub fn new(plan_id: Uuid) -> Self {
        Self { plan_id }
    }
    pub fn plan_id(&self) -> Uuid {
        self.plan_id
    }
}
