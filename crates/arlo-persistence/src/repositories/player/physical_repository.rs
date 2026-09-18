use crate::error::PersistenceResult;
use crate::models::MatchPlayerPhysicalRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerPhysicalRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_physical (
            id,
            match_id,
            player_id,
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            intra_match_recovery_amount
        ) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.end_energy_level)
    .bind(row.peak_anaerobic_depletion)
    .bind(row.total_distance_covered)
    .bind(row.intra_match_recovery_amount)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerPhysicalRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
