use crate::error::PersistenceResult;
use crate::models::MatchPlayerDrivesRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerDrivesRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_drives (
            id,
            match_id,
            player_id,
            total_drives,
            central_drives,
            left_lateral_drives,
            right_lateral_drives,
            lateral_drives,
            max_drives_in_series
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.total_drives)
    .bind(row.central_drives)
    .bind(row.left_lateral_drives)
    .bind(row.right_lateral_drives)
    .bind(row.lateral_drives)
    .bind(row.max_drives_in_series)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerDrivesRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
