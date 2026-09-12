use crate::error::PersistenceResult;
use crate::models::{MatchTeamImpulseRow, MatchTeamImpulseRunRow};
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTeamImpulseRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_team_impulse (
            id,
            match_id,
            team_id,
            average_baseline,
            current_average_value,
            min_average_value,
            max_average_value,
            average_value,
            time_below_baseline_seconds,
            runs_count,
            longest_run_duration_seconds,
            peak_run_average_value,
            total_integrated_run_intensity,
            average_run_duration_seconds,
            average_run_intensity
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(row.average_baseline)
    .bind(row.current_average_value)
    .bind(row.min_average_value)
    .bind(row.max_average_value)
    .bind(row.average_value)
    .bind(row.time_below_baseline_seconds)
    .bind(row.runs_count)
    .bind(row.longest_run_duration_seconds)
    .bind(row.peak_run_average_value)
    .bind(row.total_integrated_run_intensity)
    .bind(row.average_run_duration_seconds)
    .bind(row.average_run_intensity)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTeamImpulseRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_run(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTeamImpulseRunRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_team_impulse_runs (
            id,
            match_id,
            team_id,
            run_index,
            start_time_seconds,
            end_time_seconds,
            duration_seconds,
            peak_average_value,
            integrated_intensity,
            average_intensity
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(row.run_index)
    .bind(row.start_time_seconds)
    .bind(row.end_time_seconds)
    .bind(row.duration_seconds)
    .bind(row.peak_average_value)
    .bind(row.integrated_intensity)
    .bind(row.average_intensity)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_runs_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTeamImpulseRunRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_run(tx, row).await?;
    }
    Ok(())
}