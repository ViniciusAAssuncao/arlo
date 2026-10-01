use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerScoringSeasonStatsRow {
    pub attempts: i64,
    pub converted: i64,
    pub missed: i64,
    pub goal_points_scored: i64,
    pub field_points_scored: i64,
    pub field_goals_scored: i64,
    pub total_points_scored: i64,
}

pub async fn get_player_scoring_stats(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerScoringSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerScoringSeasonStatsRow>(
        r#"SELECT
            COALESCE(SUM(s.attempts), 0) as attempts,
            COALESCE(SUM(s.converted), 0) as converted,
            COALESCE(SUM(s.missed), 0) as missed,
            COALESCE(SUM(s.goal_points_scored), 0) as goal_points_scored,
            COALESCE(SUM(s.field_points_scored), 0) as field_points_scored,
            COALESCE(SUM(s.field_goals_scored), 0) as field_goals_scored,
            COALESCE(SUM(s.total_points_scored), 0) as total_points_scored
        FROM match_player_scoring_attempts s
        JOIN matches m ON s.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE s.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
