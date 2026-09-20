use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRow)]
pub struct PlayerAppearancesSeasonStatsRow {
    pub squad_selections: i64,
    pub starts: i64,
    pub appearances: i64,
    pub substitute_appearances: i64,
}

pub async fn get_player_appearances(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<PlayerAppearancesSeasonStatsRow> {
    let row = sqlx::query_as::<_, PlayerAppearancesSeasonStatsRow>(
        r#"SELECT
            COUNT(*) as squad_selections,
            COALESCE(SUM(CASE WHEN s.was_starter = 1 THEN 1 ELSE 0 END), 0) as starts,
            COALESCE(SUM(CASE WHEN s.was_used = 1 THEN 1 ELSE 0 END), 0) as appearances,
            COALESCE(SUM(CASE WHEN s.was_starter = 0 AND s.was_used = 1 THEN 1 ELSE 0 END), 0) as substitute_appearances
        FROM match_squad_selections s
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
