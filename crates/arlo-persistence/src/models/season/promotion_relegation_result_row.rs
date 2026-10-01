use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct PromotionRelegationResultRow {
    pub id: String,
    pub season_instance_id: String,
    pub team_id: String,
    pub movement_kind: String,
    pub source_league_id: String,
    pub destination_league_id: Option<String>,
    pub final_standing_position: Option<i32>,
    pub created_at_unix_seconds: i64,
}

impl PromotionRelegationResultRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        season_instance_id: Uuid,
        team_id: Uuid,
        movement_kind: impl Into<String>,
        source_league_id: Uuid,
        destination_league_id: Option<Uuid>,
        final_standing_position: Option<u32>,
        created_at_unix_seconds: i64,
    ) -> Self {
        Self {
            id: id.to_string(),
            season_instance_id: season_instance_id.to_string(),
            team_id: team_id.to_string(),
            movement_kind: movement_kind.into(),
            source_league_id: source_league_id.to_string(),
            destination_league_id: destination_league_id.map(|id| id.to_string()),
            final_standing_position: final_standing_position.map(|p| p as i32),
            created_at_unix_seconds,
        }
    }
}
