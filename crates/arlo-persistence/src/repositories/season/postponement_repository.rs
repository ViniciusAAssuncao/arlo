use crate::error::PersistenceResult;
use crate::models::season::PostponementRecordRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &PostponementRecordRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO fixture_postponements (
            id,
            fixture_id,
            original_year,
            original_day_of_year,
            new_year,
            new_day_of_year,
            reason,
            created_at_unix_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.fixture_id)
    .bind(row.original_year)
    .bind(row.original_day_of_year)
    .bind(row.new_year)
    .bind(row.new_day_of_year)
    .bind(&row.reason)
    .bind(row.created_at_unix_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[PostponementRecordRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn get_by_id(
    pool: &SqlitePool,
    id: Uuid,
) -> PersistenceResult<Option<PostponementRecordRow>> {
    let row = sqlx::query_as::<_, PostponementRecordRow>(
        "SELECT id, fixture_id, original_year, original_day_of_year, new_year, new_day_of_year, reason, created_at_unix_seconds FROM fixture_postponements WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_fixture_id(
    pool: &SqlitePool,
    fixture_id: Uuid,
) -> PersistenceResult<Vec<PostponementRecordRow>> {
    let rows = sqlx::query_as::<_, PostponementRecordRow>(
        "SELECT id, fixture_id, original_year, original_day_of_year, new_year, new_day_of_year, reason, created_at_unix_seconds FROM fixture_postponements WHERE fixture_id = ? ORDER BY created_at_unix_seconds ASC",
    )
    .bind(fixture_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
