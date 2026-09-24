use crate::error::ControllerResult;
use crate::services::season::season_generator::{self, GeneratedSeason};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn trigger_season_generation(
    pool: &SqlitePool,
    competition_id: Uuid,
    calendar_system_id: Uuid,
    reference_year: i64,
) -> ControllerResult<GeneratedSeason> {
    season_generator::generate_season_for_league(
        pool,
        competition_id,
        calendar_system_id,
        reference_year,
    )
    .await
}
