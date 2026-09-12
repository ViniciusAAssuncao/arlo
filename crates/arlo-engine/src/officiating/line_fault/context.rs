use crate::attributes::RefereeAttributeTable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineFaultEvaluationContext {
    pub receiver_id: Uuid,
    pub receiver_team_id: Uuid,
    pub defender_id: Uuid,
    pub defender_team_id: Uuid,
    pub offside_margin_meters: f64,
    pub head_referee_table: RefereeAttributeTable,
    pub peace_referee_table: RefereeAttributeTable,
}

impl LineFaultEvaluationContext {
    pub fn new(
        receiver_id: Uuid,
        receiver_team_id: Uuid,
        defender_id: Uuid,
        defender_team_id: Uuid,
        offside_margin_meters: f64,
        head_referee_table: RefereeAttributeTable,
        peace_referee_table: RefereeAttributeTable,
    ) -> Self {
        Self {
            receiver_id,
            receiver_team_id,
            defender_id,
            defender_team_id,
            offside_margin_meters,
            head_referee_table,
            peace_referee_table,
        }
    }
}