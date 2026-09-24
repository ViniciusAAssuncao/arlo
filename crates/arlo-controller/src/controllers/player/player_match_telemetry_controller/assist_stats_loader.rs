use crate::dto::stats::PlayerAssistStatsDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_assist_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerAssistStatsDto> {
    let assist_row =
        arlo_persistence::repositories::match_player_assists::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    Ok(match assist_row {
        Some(a) => PlayerAssistStatsDto {
            goalpoint_assists: a.goalpoint_assists as u32,
        },
        None => PlayerAssistStatsDto::default(),
    })
}
