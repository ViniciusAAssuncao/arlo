use crate::error::PersistenceResult;
use crate::models::{MatchTeamImpulseRow, MatchTeamImpulseRunRow};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const IMPULSE_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "team_id",
    "average_baseline",
    "current_average_value",
    "min_average_value",
    "max_average_value",
    "average_value",
    "time_below_baseline_seconds",
    "runs_count",
    "longest_run_duration_seconds",
    "peak_run_average_value",
    "total_integrated_run_intensity",
    "average_run_duration_seconds",
    "average_run_intensity",
];

const RUN_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "team_id",
    "run_index",
    "start_time_seconds",
    "end_time_seconds",
    "duration_seconds",
    "peak_average_value",
    "integrated_intensity",
    "average_intensity",
];

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
    execute_batch_insert(tx, "match_team_impulse", IMPULSE_COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.team_id);
        b.push_bind(row.average_baseline);
        b.push_bind(row.current_average_value);
        b.push_bind(row.min_average_value);
        b.push_bind(row.max_average_value);
        b.push_bind(row.average_value);
        b.push_bind(row.time_below_baseline_seconds);
        b.push_bind(row.runs_count);
        b.push_bind(row.longest_run_duration_seconds);
        b.push_bind(row.peak_run_average_value);
        b.push_bind(row.total_integrated_run_intensity);
        b.push_bind(row.average_run_duration_seconds);
        b.push_bind(row.average_run_intensity);
    })
    .await
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
    execute_batch_insert(tx, "match_team_impulse_runs", RUN_COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.team_id);
        b.push_bind(row.run_index);
        b.push_bind(row.start_time_seconds);
        b.push_bind(row.end_time_seconds);
        b.push_bind(row.duration_seconds);
        b.push_bind(row.peak_average_value);
        b.push_bind(row.integrated_intensity);
        b.push_bind(row.average_intensity);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchTeamImpulseRow>> {
    let rows = sqlx::query_as::<_, MatchTeamImpulseRow>(
        r#"SELECT
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
        FROM match_team_impulse
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_runs_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchTeamImpulseRunRow>> {
    let rows = sqlx::query_as::<_, MatchTeamImpulseRunRow>(
        r#"SELECT
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
        FROM match_team_impulse_runs
        WHERE match_id = ?
        ORDER BY team_id ASC, run_index ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}