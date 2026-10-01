use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct PlayerInjuryHistoryRow {
    pub id: String,
    pub player_id: String,
    pub injury_definition_id: String,
    pub body_region: String,
    pub severity_grade: String,
    pub injury_extent: Option<String>,
    pub treatment_kind: String,
    pub onset_year: i64,
    pub onset_day_of_year: i32,
    pub expected_recovery_days: i32,
    pub days_remaining: i32,
    pub observation_days_remaining: i32,
    pub status: String,
    pub is_relapse: bool,
    pub origin_record_id: Option<String>,
    pub resolved_at_unix_seconds: Option<i64>,
    pub created_at_unix_seconds: i64,
}

impl PlayerInjuryHistoryRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        player_id: Uuid,
        injury_definition_id: Uuid,
        body_region: impl Into<String>,
        severity_grade: impl Into<String>,
        injury_extent: Option<String>,
        treatment_kind: impl Into<String>,
        onset_year: i64,
        onset_day_of_year: u32,
        expected_recovery_days: u32,
        days_remaining: u32,
        observation_days_remaining: u32,
        status: impl Into<String>,
        is_relapse: bool,
        origin_record_id: Option<Uuid>,
        resolved_at_unix_seconds: Option<i64>,
        created_at_unix_seconds: i64,
    ) -> Self {
        Self {
            id: id.to_string(),
            player_id: player_id.to_string(),
            injury_definition_id: injury_definition_id.to_string(),
            body_region: body_region.into(),
            severity_grade: severity_grade.into(),
            injury_extent,
            treatment_kind: treatment_kind.into(),
            onset_year,
            onset_day_of_year: onset_day_of_year as i32,
            expected_recovery_days: expected_recovery_days as i32,
            days_remaining: days_remaining as i32,
            observation_days_remaining: observation_days_remaining as i32,
            status: status.into(),
            is_relapse,
            origin_record_id: origin_record_id.map(|id| id.to_string()),
            resolved_at_unix_seconds,
            created_at_unix_seconds,
        }
    }
}
