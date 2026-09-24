use crate::error::PersistenceResult;
use crate::models::season::SeasonStageRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &SeasonStageRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO season_stages (
            id,
            season_instance_id,
            stage_order_index,
            stage_type,
            status
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.season_instance_id)
    .bind(row.stage_order_index)
    .bind(&row.stage_type)
    .bind(&row.status)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[SeasonStageRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> PersistenceResult<Option<SeasonStageRow>> {
    let row = sqlx::query_as::<_, SeasonStageRow>(
        "SELECT id, season_instance_id, stage_order_index, stage_type, status FROM season_stages WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_season_instance_id(
    pool: &SqlitePool,
    season_instance_id: Uuid,
) -> PersistenceResult<Vec<SeasonStageRow>> {
    let rows = sqlx::query_as::<_, SeasonStageRow>(
        "SELECT id, season_instance_id, stage_order_index, stage_type, status FROM season_stages WHERE season_instance_id = ? ORDER BY stage_order_index ASC",
    )
    .bind(season_instance_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
