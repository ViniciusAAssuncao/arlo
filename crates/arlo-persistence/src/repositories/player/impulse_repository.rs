use crate::error::PersistenceResult;
use crate::models::{
    MatchPlayerImpulseRow, MatchPlayerImpulseRunRow, MatchPlayerImpulseShiftByKindRow,
};
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerImpulseRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_impulse (
            id,
            match_id,
            player_id,
            baseline,
            current_value,
            initial_value,
            min_value,
            max_value,
            average_value,
            shifts_count,
            positive_shifts,
            negative_shifts,
            time_below_baseline_seconds,
            critical_reached_count,
            runs_count,
            longest_run_duration_seconds,
            peak_run_value,
            total_integrated_run_intensity,
            average_run_duration_seconds,
            average_run_intensity
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.baseline)
    .bind(row.current_value)
    .bind(row.initial_value)
    .bind(row.min_value)
    .bind(row.max_value)
    .bind(row.average_value)
    .bind(row.shifts_count)
    .bind(row.positive_shifts)
    .bind(row.negative_shifts)
    .bind(row.time_below_baseline_seconds)
    .bind(row.critical_reached_count)
    .bind(row.runs_count)
    .bind(row.longest_run_duration_seconds)
    .bind(row.peak_run_value)
    .bind(row.total_integrated_run_intensity)
    .bind(row.average_run_duration_seconds)
    .bind(row.average_run_intensity)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerImpulseRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_shift_by_kind(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerImpulseShiftByKindRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_impulse_shifts_by_kind (
            id,
            match_id,
            player_id,
            event_kind,
            shifts_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(&row.event_kind)
    .bind(row.shifts_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_shifts_by_kind_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerImpulseShiftByKindRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_shift_by_kind(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_run(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerImpulseRunRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_impulse_runs (
            id,
            match_id,
            player_id,
            run_index,
            start_time_seconds,
            end_time_seconds,
            duration_seconds,
            peak_value,
            integrated_intensity,
            shifts_count,
            average_intensity
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.run_index)
    .bind(row.start_time_seconds)
    .bind(row.end_time_seconds)
    .bind(row.duration_seconds)
    .bind(row.peak_value)
    .bind(row.integrated_intensity)
    .bind(row.shifts_count)
    .bind(row.average_intensity)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_runs_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerImpulseRunRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_run(tx, row).await?;
    }
    Ok(())
}

pub async fn get_latest_by_player_id(
    pool: &SqlitePool,
    player_id: Uuid,
) -> PersistenceResult<Option<MatchPlayerImpulseRow>> {
    let row = sqlx::query_as::<_, MatchPlayerImpulseRow>(
        r#"SELECT
            p.id,
            p.match_id,
            p.player_id,
            p.baseline,
            p.current_value,
            p.initial_value,
            p.min_value,
            p.max_value,
            p.average_value,
            p.shifts_count,
            p.positive_shifts,
            p.negative_shifts,
            p.time_below_baseline_seconds,
            p.critical_reached_count,
            p.runs_count,
            p.longest_run_duration_seconds,
            p.peak_run_value,
            p.total_integrated_run_intensity,
            p.average_run_duration_seconds,
            p.average_run_intensity
        FROM match_player_impulse p
        LEFT JOIN matches m ON p.match_id = m.id
        LEFT JOIN fixtures f ON m.fixture_id = f.id
        WHERE p.player_id = ?
        ORDER BY COALESCE(f.scheduled_year, 0) DESC, COALESCE(f.scheduled_day_of_year, 0) DESC, COALESCE(m.completed_at_unix_seconds, 0) DESC, p.rowid DESC
        LIMIT 1"#,
    )
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_latest_by_player_ids(
    pool: &SqlitePool,
    player_ids: &[Uuid],
) -> PersistenceResult<Vec<MatchPlayerImpulseRow>> {
    if player_ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat("?").take(player_ids.len()).collect::<Vec<_>>().join(", ");

    let sql = format!(
        r#"SELECT
            id,
            match_id,
            player_id,
            baseline,
            current_value,
            initial_value,
            min_value,
            max_value,
            average_value,
            shifts_count,
            positive_shifts,
            negative_shifts,
            time_below_baseline_seconds,
            critical_reached_count,
            runs_count,
            longest_run_duration_seconds,
            peak_run_value,
            total_integrated_run_intensity,
            average_run_duration_seconds,
            average_run_intensity
        FROM (
            SELECT
                p.id,
                p.match_id,
                p.player_id,
                p.baseline,
                p.current_value,
                p.initial_value,
                p.min_value,
                p.max_value,
                p.average_value,
                p.shifts_count,
                p.positive_shifts,
                p.negative_shifts,
                p.time_below_baseline_seconds,
                p.critical_reached_count,
                p.runs_count,
                p.longest_run_duration_seconds,
                p.peak_run_value,
                p.total_integrated_run_intensity,
                p.average_run_duration_seconds,
                p.average_run_intensity,
                ROW_NUMBER() OVER (
                    PARTITION BY p.player_id
                    ORDER BY COALESCE(f.scheduled_year, 0) DESC, COALESCE(f.scheduled_day_of_year, 0) DESC, COALESCE(m.completed_at_unix_seconds, 0) DESC, p.rowid DESC
                ) AS rn
            FROM match_player_impulse p
            LEFT JOIN matches m ON p.match_id = m.id
            LEFT JOIN fixtures f ON m.fixture_id = f.id
            WHERE p.player_id IN ({})
        )
        WHERE rn = 1"#,
        placeholders
    );

    let mut query = sqlx::query_as::<_, MatchPlayerImpulseRow>(&sql);
    for id in player_ids {
        query = query.bind(id.to_string());
    }

    let rows = query.fetch_all(pool).await?;
    Ok(rows)
}