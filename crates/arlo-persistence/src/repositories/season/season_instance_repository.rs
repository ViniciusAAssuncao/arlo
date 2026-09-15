use crate::error::PersistenceResult;
use crate::models::season::SeasonInstanceRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &SeasonInstanceRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO season_instances (
            id,
            competition_id,
            reference_year,
            current_stage_order_index,
            status,
            created_at_unix_seconds
        ) VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.competition_id)
    .bind(row.reference_year)
    .bind(row.current_stage_order_index)
    .bind(&row.status)
    .bind(row.created_at_unix_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_by_id(
    pool: &SqlitePool,
    id: Uuid,
) -> PersistenceResult<Option<SeasonInstanceRow>> {
    let row = sqlx::query_as::<_, SeasonInstanceRow>(
        "SELECT id, competition_id, reference_year, current_stage_order_index, status, created_at_unix_seconds FROM season_instances WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_by_competition_and_year(
    pool: &SqlitePool,
    competition_id: Uuid,
    reference_year: i64,
) -> PersistenceResult<Option<SeasonInstanceRow>> {
    let row = sqlx::query_as::<_, SeasonInstanceRow>(
        "SELECT id, competition_id, reference_year, current_stage_order_index, status, created_at_unix_seconds FROM season_instances WHERE competition_id = ? AND reference_year = ?",
    )
    .bind(competition_id.to_string())
    .bind(reference_year)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_competition_id(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> PersistenceResult<Vec<SeasonInstanceRow>> {
    let rows = sqlx::query_as::<_, SeasonInstanceRow>(
        "SELECT id, competition_id, reference_year, current_stage_order_index, status, created_at_unix_seconds FROM season_instances WHERE competition_id = ? ORDER BY reference_year DESC",
    )
    .bind(competition_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
