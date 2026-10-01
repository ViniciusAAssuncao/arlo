use crate::domain::season::season_instance_status::SeasonInstanceStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonInstance {
    id: Uuid,
    competition_id: Uuid,
    reference_year: i64,
    current_stage_order_index: u32,
    status: SeasonInstanceStatus,
}

impl SeasonInstance {
    pub fn new(
        id: Uuid,
        competition_id: Uuid,
        reference_year: i64,
        current_stage_order_index: u32,
        status: SeasonInstanceStatus,
    ) -> Self {
        Self {
            id,
            competition_id,
            reference_year,
            current_stage_order_index,
            status,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn competition_id(&self) -> Uuid {
        self.competition_id
    }

    pub fn reference_year(&self) -> i64 {
        self.reference_year
    }

    pub fn current_stage_order_index(&self) -> u32 {
        self.current_stage_order_index
    }

    pub fn status(&self) -> SeasonInstanceStatus {
        self.status
    }
}
