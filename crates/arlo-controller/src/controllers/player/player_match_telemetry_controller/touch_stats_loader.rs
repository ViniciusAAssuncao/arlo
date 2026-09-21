use crate::dto::stats::PlayerTouchStatsDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_touch_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerTouchStatsDto> {
    let touch_row = arlo_persistence::repositories::match_player_touches::get_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    Ok(match touch_row {
        Some(r) => PlayerTouchStatsDto {
            total_touches: r.total_touches as u32,
            passes_attempted: r.passes_attempted as u32,
            passes_received: r.passes_received as u32,
            drives_recorded: r.drives_recorded as u32,
            recoveries: r.recoveries as u32,
            scoring_attempts: r.scoring_attempts as u32,
            turnovers_conceded: r.turnovers_conceded as u32,
        },
        None => PlayerTouchStatsDto::default(),
    })
}
