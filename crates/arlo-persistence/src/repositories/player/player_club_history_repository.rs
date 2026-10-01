use crate::error::PersistenceResult;
use crate::models::player::player_club_history_row::PlayerClubHistoryRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn insert(pool: &SqlitePool, row: &PlayerClubHistoryRow) -> PersistenceResult<()> {
    sqlx::query(
        "INSERT INTO player_club_history (id, player_id, team_id, joined_year, left_year, created_at_unix_seconds) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.id)
    .bind(&row.player_id)
    .bind(&row.team_id)
    .bind(row.joined_year)
    .bind(row.left_year)
    .bind(row.created_at_unix_seconds)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_by_player_id(
    pool: &SqlitePool,
    player_id: Uuid,
) -> PersistenceResult<Vec<PlayerClubHistoryRow>> {
    let rows = sqlx::query_as::<_, PlayerClubHistoryRow>(
        "SELECT id, player_id, team_id, joined_year, left_year, created_at_unix_seconds FROM player_club_history WHERE player_id = ? ORDER BY joined_year ASC, created_at_unix_seconds ASC",
    )
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
