use crate::error::PersistenceResult;
use crate::models::MatchRefereePerformanceRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "referee_id",
    "role",
    "calls_made",
    "calls_correct",
    "calls_incorrect",
    "peace_referee_interventions",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchRefereePerformanceRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_referee_performance (
            id,
            match_id,
            referee_id,
            role,
            calls_made,
            calls_correct,
            calls_incorrect,
            peace_referee_interventions
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.referee_id)
    .bind(&row.role)
    .bind(row.calls_made)
    .bind(row.calls_correct)
    .bind(row.calls_incorrect)
    .bind(row.peace_referee_interventions)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchRefereePerformanceRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_referee_performance", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.referee_id);
        b.push_bind(&row.role);
        b.push_bind(row.calls_made);
        b.push_bind(row.calls_correct);
        b.push_bind(row.calls_incorrect);
        b.push_bind(row.peace_referee_interventions);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchRefereePerformanceRow>> {
    let rows = sqlx::query_as::<_, MatchRefereePerformanceRow>(
        r#"SELECT
            id,
            match_id,
            referee_id,
            role,
            calls_made,
            calls_correct,
            calls_incorrect,
            peace_referee_interventions
        FROM match_referee_performance
        WHERE match_id = ?
        ORDER BY role ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}