use crate::dto::player::PlayerSeasonStatsDto;
use crate::error::ControllerResult;
use crate::services::stats::player_season_stats_service;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_player_season_stats(
    pool: &SqlitePool,
    player_id: Uuid,
) -> ControllerResult<PlayerSeasonStatsDto> {
    player_season_stats_service::build_player_season_stats(pool, player_id).await
}
