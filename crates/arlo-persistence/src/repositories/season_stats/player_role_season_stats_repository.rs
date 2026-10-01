use crate::error::PersistenceResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_player_primary_role(
    pool: &SqlitePool,
    player_id: Uuid,
    season_instance_id: Uuid,
) -> PersistenceResult<Option<String>> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"SELECT s.slot_role
        FROM match_squad_selections s
        JOIN matches m ON s.match_id = m.id
        JOIN fixtures f ON m.fixture_id = f.id
        JOIN season_stages ss ON f.season_stage_id = ss.id
        WHERE s.player_id = ? AND ss.season_instance_id = ? AND s.slot_role IS NOT NULL AND s.slot_role != ''
        GROUP BY s.slot_role
        ORDER BY COUNT(*) DESC
        LIMIT 1"#,
    )
    .bind(player_id.to_string())
    .bind(season_instance_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(r,)| r))
}
