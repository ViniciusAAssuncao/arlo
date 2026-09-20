use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerAssistSeasonStatsRow {
    pub goalpoint_assists: i64,
}

pub async fn get_player_assist_stats(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerAssistSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerAssistSeasonStatsRow>(
        r#"SELECT
            COALESCE(SUM(a.goalpoint_assists), 0) as goalpoint_assists
        FROM match_player_assists a
        JOIN matches m ON a.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE a.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
