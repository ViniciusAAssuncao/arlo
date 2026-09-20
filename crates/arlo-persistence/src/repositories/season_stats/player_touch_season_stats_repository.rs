use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerTouchSeasonStatsRow {
    pub passes_attempted: i64,
    pub passes_received: i64,
    pub drives_recorded: i64,
    pub recoveries: i64,
    pub scoring_attempts: i64,
    pub total_touches: i64,
    pub turnovers_conceded: i64,
}

pub async fn get_player_touch_stats(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerTouchSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerTouchSeasonStatsRow>(
        r#"SELECT
            COALESCE(SUM(t.passes_attempted), 0) as passes_attempted,
            COALESCE(SUM(t.passes_received), 0) as passes_received,
            COALESCE(SUM(t.drives_recorded), 0) as drives_recorded,
            COALESCE(SUM(t.recoveries), 0) as recoveries,
            COALESCE(SUM(t.scoring_attempts), 0) as scoring_attempts,
            COALESCE(SUM(t.total_touches), 0) as total_touches,
            COALESCE(SUM(t.turnovers_conceded), 0) as turnovers_conceded
        FROM match_player_touches t
        JOIN matches m ON t.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE t.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
