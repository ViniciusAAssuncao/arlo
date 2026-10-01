use crate::domain::season::stage_status::StageStatus;
use arlo_domain::StageType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonStageInstance {
    id: Uuid,
    season_instance_id: Uuid,
    stage_order_index: u32,
    stage_type: StageType,
    status: StageStatus,
}

impl SeasonStageInstance {
    pub fn new(
        id: Uuid,
        season_instance_id: Uuid,
        stage_order_index: u32,
        stage_type: StageType,
        status: StageStatus,
    ) -> Self {
        Self {
            id,
            season_instance_id,
            stage_order_index,
            stage_type,
            status,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn season_instance_id(&self) -> Uuid {
        self.season_instance_id
    }

    pub fn stage_order_index(&self) -> u32 {
        self.stage_order_index
    }

    pub fn stage_type(&self) -> StageType {
        self.stage_type
    }

    pub fn status(&self) -> StageStatus {
        self.status
    }
}
