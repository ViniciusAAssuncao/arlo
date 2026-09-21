use crate::dto::stats::PlayerDriveStatsDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_drive_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerDriveStatsDto> {
    let drive_row = arlo_persistence::repositories::match_player_drives::get_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    Ok(match drive_row {
        Some(r) => PlayerDriveStatsDto {
            total_drives: r.total_drives as u32,
            central_drives: r.central_drives as u32,
            left_lateral_drives: r.left_lateral_drives as u32,
            right_lateral_drives: r.right_lateral_drives as u32,
            lateral_drives: r.lateral_drives as u32,
            max_drives_in_series: r.max_drives_in_series as u32,
        },
        None => PlayerDriveStatsDto::default(),
    })
}
