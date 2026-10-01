use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerDriveSeasonStatsRow {
    pub total_drives: i64,
    pub central_drives: i64,
    pub left_lateral_drives: i64,
    pub right_lateral_drives: i64,
    pub lateral_drives: i64,
    pub max_drives_in_series: i64,
}

pub async fn get_player_drive_stats(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerDriveSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerDriveSeasonStatsRow>(
        r#"SELECT
            COALESCE(SUM(d.total_drives), 0) as total_drives,
            COALESCE(SUM(d.central_drives), 0) as central_drives,
            COALESCE(SUM(d.left_lateral_drives), 0) as left_lateral_drives,
            COALESCE(SUM(d.right_lateral_drives), 0) as right_lateral_drives,
            COALESCE(SUM(d.lateral_drives), 0) as lateral_drives,
            COALESCE(MAX(d.max_drives_in_series), 0) as max_drives_in_series
        FROM match_player_drives d
        JOIN matches m ON d.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE d.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
