use crate::dto::team::RosterEntryDto;
use crate::error::ControllerResult;
use crate::services::team::roster_builder_service::build_team_roster;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_team_roster(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<Vec<RosterEntryDto>> {
    build_team_roster(pool, team_id).await
}
