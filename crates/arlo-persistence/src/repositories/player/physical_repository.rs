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
            high_intensity_distance,
            low_intensity_distance,
            metabolic_energy_joules,
            peak_speed_meters_per_sec,
            intra_match_recovery_amount,
            distance_first_zone,
            distance_second_zone,
            distance_corridors,
            distance_central
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.end_energy_level)
    .bind(row.peak_anaerobic_depletion)
    .bind(row.total_distance_covered)
    .bind(row.high_intensity_distance)
    .bind(row.low_intensity_distance)
    .bind(row.metabolic_energy_joules)
    .bind(row.peak_speed_meters_per_sec)
    .bind(row.intra_match_recovery_amount)
    .bind(row.distance_first_zone)
    .bind(row.distance_second_zone)
    .bind(row.distance_corridors)
    .bind(row.distance_central)
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
