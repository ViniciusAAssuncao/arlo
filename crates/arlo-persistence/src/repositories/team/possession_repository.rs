use crate::error::PersistenceResult;
use crate::models::MatchTeamPossessionRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &["id", "match_id", "team_id", "total_possession_seconds"];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTeamPossessionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_team_possession (
            id,
            match_id,
            team_id,
            total_possession_seconds
        ) VALUES (?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(row.total_possession_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTeamPossessionRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_team_possession", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.team_id);
        b.push_bind(row.total_possession_seconds);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchTeamPossessionRow>> {
    let rows = sqlx::query_as::<_, MatchTeamPossessionRow>(
        r#"SELECT
            id,
            match_id,
            team_id,
            total_possession_seconds
        FROM match_team_possession
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
