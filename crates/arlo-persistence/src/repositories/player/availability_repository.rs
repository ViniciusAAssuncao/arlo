use crate::error::PersistenceResult;
use crate::models::MatchPlayerAvailabilityRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerAvailabilityRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_availability (
            id,
            match_id,
            player_id,
            total_suspended_seconds,
            expulsion_count,
            is_currently_expelled
        ) VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.total_suspended_seconds)
    .bind(row.expulsion_count)
    .bind(row.is_currently_expelled)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerAvailabilityRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
