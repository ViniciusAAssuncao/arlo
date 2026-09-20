use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, FromRow)]
pub struct PlayerArtrineDecisionSeasonStatsRow {
    pub total_decisions: i64,
    pub total_successful_decisions: i64,
    pub total_failed_decisions: i64,
    pub total_mirins_advanced: f64,
    pub total_points_generated: i64,
    pub goal_points_generated: i64,
    pub field_points_generated: i64,
    pub field_goals_generated: i64,
}

pub async fn get_player_artrine_decision_stats(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerArtrineDecisionSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerArtrineDecisionSeasonStatsRow>(
        r#"SELECT
            COALESCE(SUM(ad.total_decisions), 0) as total_decisions,
            COALESCE(SUM(ad.total_successful_decisions), 0) as total_successful_decisions,
            COALESCE(SUM(ad.total_failed_decisions), 0) as total_failed_decisions,
            COALESCE(SUM(ad.total_mirins_advanced), 0.0) as total_mirins_advanced,
            COALESCE(SUM(ad.total_points_generated), 0) as total_points_generated,
            COALESCE(SUM(ad.goal_points_generated), 0) as goal_points_generated,
            COALESCE(SUM(ad.field_points_generated), 0) as field_points_generated,
            COALESCE(SUM(ad.field_goals_generated), 0) as field_goals_generated
        FROM match_player_artrine_decisions ad
        JOIN matches m ON ad.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE ad.player_id = ? AND ss.season_instance_id = ?"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_one(pool)
    .await?;

    Ok(row)
}
