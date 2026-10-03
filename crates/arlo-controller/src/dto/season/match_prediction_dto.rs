use crate::domain::calendar::CalendarDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchPredictionDto {
    pub fixture_id: Uuid,
    pub home_win: f64,
    pub draw: f64,
    pub away_win: f64,
    pub power_snapshot_id: Option<Uuid>,
    pub calibration_id: Uuid,
    pub model_version: u32,
    pub generated_as_of: CalendarDate,
}
