use crate::error::{ PersistenceError, PersistenceResult };
use crate::models::condition::PlayerInjuryHistoryRow;
use sqlx::{ Sqlite, SqlitePool, Transaction };
use uuid::Uuid;

pub async fn insert(pool: &SqlitePool, row: &PlayerInjuryHistoryRow) -> PersistenceResult<()> {
    sqlx
        ::query(
            r#"INSERT INTO player_injury_history (
            id,
            player_id,
            injury_definition_id,
            body_region,
            severity_grade,
            onset_year,
            onset_day_of_year,
            expected_recovery_days,
            days_remaining,
            observation_days_remaining,
            status,
            is_relapse,
            origin_record_id,
            resolved_at_unix_seconds,
            created_at_unix_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&row.id)
        .bind(&row.player_id)
        .bind(&row.injury_definition_id)
        .bind(&row.body_region)
        .bind(&row.severity_grade)
        .bind(row.onset_year)
        .bind(row.onset_day_of_year)
        .bind(row.expected_recovery_days)
        .bind(row.days_remaining)
        .bind(row.observation_days_remaining)
        .bind(&row.status)
        .bind(row.is_relapse)
        .bind(&row.origin_record_id)
        .bind(row.resolved_at_unix_seconds)
        .bind(row.created_at_unix_seconds)
        .execute(pool).await?;

    Ok(())
}

pub async fn insert_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    row: &PlayerInjuryHistoryRow
) -> PersistenceResult<()> {
    sqlx
        ::query(
            r#"INSERT INTO player_injury_history (
            id,
            player_id,
            injury_definition_id,
            body_region,
            severity_grade,
            onset_year,
            onset_day_of_year,
            expected_recovery_days,
            days_remaining,
            observation_days_remaining,
            status,
            is_relapse,
            origin_record_id,
            resolved_at_unix_seconds,
            created_at_unix_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&row.id)
        .bind(&row.player_id)
        .bind(&row.injury_definition_id)
        .bind(&row.body_region)
        .bind(&row.severity_grade)
        .bind(row.onset_year)
        .bind(row.onset_day_of_year)
        .bind(row.expected_recovery_days)
        .bind(row.days_remaining)
        .bind(row.observation_days_remaining)
        .bind(&row.status)
        .bind(row.is_relapse)
        .bind(&row.origin_record_id)
        .bind(row.resolved_at_unix_seconds)
        .bind(row.created_at_unix_seconds)
        .execute(&mut **tx).await?;

    Ok(())
}

pub async fn get_active_by_player_id(
    pool: &SqlitePool,
    player_id: Uuid
) -> PersistenceResult<Option<PlayerInjuryHistoryRow>> {
    let row = sqlx
        ::query_as::<_, PlayerInjuryHistoryRow>(
            r#"SELECT
            id,
            player_id,
            injury_definition_id,
            body_region,
            severity_grade,
            onset_year,
            onset_day_of_year,
            expected_recovery_days,
            days_remaining,
            observation_days_remaining,
            status,
            is_relapse,
            origin_record_id,
            resolved_at_unix_seconds,
            created_at_unix_seconds
        FROM player_injury_history
        WHERE player_id = ? AND status != 'Resolved'
        ORDER BY created_at_unix_seconds DESC
        LIMIT 1"#
        )
        .bind(player_id.to_string())
        .fetch_optional(pool).await?;

    Ok(row)
}

pub async fn list_all_active(
    pool: &SqlitePool
) -> PersistenceResult<Vec<PlayerInjuryHistoryRow>> {
    let rows = sqlx
        ::query_as::<_, PlayerInjuryHistoryRow>(
            r#"SELECT
            id,
            player_id,
            injury_definition_id,
            body_region,
            severity_grade,
            onset_year,
            onset_day_of_year,
            expected_recovery_days,
            days_remaining,
            observation_days_remaining,
            status,
            is_relapse,
            origin_record_id,
            resolved_at_unix_seconds,
            created_at_unix_seconds
        FROM player_injury_history
        WHERE status != 'Resolved'
        ORDER BY created_at_unix_seconds DESC"#
        )
        .fetch_all(pool).await?;

    Ok(rows)
}

pub async fn list_active_by_player_ids(
    pool: &SqlitePool,
    player_ids: &[Uuid]
) -> PersistenceResult<Vec<PlayerInjuryHistoryRow>> {
    if player_ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat("?").take(player_ids.len()).collect::<Vec<_>>().join(", ");

    let sql =
        format!(r#"SELECT
            id,
            player_id,
            injury_definition_id,
            body_region,
            severity_grade,
            onset_year,
            onset_day_of_year,
            expected_recovery_days,
            days_remaining,
            observation_days_remaining,
            status,
            is_relapse,
            origin_record_id,
            resolved_at_unix_seconds,
            created_at_unix_seconds
        FROM player_injury_history
        WHERE status != 'Resolved' AND player_id IN ({})
        ORDER BY created_at_unix_seconds DESC"#, placeholders);

    let mut query = sqlx::query_as::<_, PlayerInjuryHistoryRow>(&sql);
    for id in player_ids {
        query = query.bind(id.to_string());
    }

    let rows = query.fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_latest_resolved_by_player_id(
    pool: &SqlitePool,
    player_id: Uuid,
    since_unix_seconds: i64
) -> PersistenceResult<Option<PlayerInjuryHistoryRow>> {
    let row = sqlx
        ::query_as::<_, PlayerInjuryHistoryRow>(
            r#"SELECT
            id,
            player_id,
            injury_definition_id,
            body_region,
            severity_grade,
            onset_year,
            onset_day_of_year,
            expected_recovery_days,
            days_remaining,
            observation_days_remaining,
            status,
            is_relapse,
            origin_record_id,
            resolved_at_unix_seconds,
            created_at_unix_seconds
        FROM player_injury_history
        WHERE player_id = ?
          AND status = 'Resolved'
          AND resolved_at_unix_seconds IS NOT NULL
          AND resolved_at_unix_seconds >= ?
        ORDER BY resolved_at_unix_seconds DESC, created_at_unix_seconds DESC
        LIMIT 1"#
        )
        .bind(player_id.to_string())
        .bind(since_unix_seconds)
        .fetch_optional(pool).await?;

    Ok(row)
}

pub async fn list_latest_resolved_by_player_ids(
    pool: &SqlitePool,
    player_ids: &[Uuid],
    since_unix_seconds: i64
) -> PersistenceResult<Vec<PlayerInjuryHistoryRow>> {
    if player_ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat("?").take(player_ids.len()).collect::<Vec<_>>().join(", ");

    let sql = format!(
        r#"SELECT
            id,
            player_id,
            injury_definition_id,
            body_region,
            severity_grade,
            onset_year,
            onset_day_of_year,
            expected_recovery_days,
            days_remaining,
            observation_days_remaining,
            status,
            is_relapse,
            origin_record_id,
            resolved_at_unix_seconds,
            created_at_unix_seconds
        FROM (
            SELECT *,
                ROW_NUMBER() OVER (
                    PARTITION BY player_id
                    ORDER BY resolved_at_unix_seconds DESC, created_at_unix_seconds DESC
                ) AS rn
            FROM player_injury_history
            WHERE status = 'Resolved'
              AND resolved_at_unix_seconds IS NOT NULL
              AND resolved_at_unix_seconds >= ?
              AND player_id IN ({})
        )
        WHERE rn = 1"#,
        placeholders
    );

    let mut query = sqlx::query_as::<_, PlayerInjuryHistoryRow>(&sql);
    query = query.bind(since_unix_seconds);
    for id in player_ids {
        query = query.bind(id.to_string());
    }

    let rows = query.fetch_all(pool).await?;
    Ok(rows)
}

pub async fn update_progress(
    pool: &SqlitePool,
    id: Uuid,
    days_remaining: u32,
    observation_days_remaining: u32,
    status: &str
) -> PersistenceResult<()> {
    let result = sqlx
        ::query(
            r#"UPDATE player_injury_history SET
            days_remaining = ?,
            observation_days_remaining = ?,
            status = ?
        WHERE id = ?"#
        )
        .bind(days_remaining as i32)
        .bind(observation_days_remaining as i32)
        .bind(status)
        .bind(id.to_string())
        .execute(pool).await?;

    if result.rows_affected() == 0 {
        return Err(
            PersistenceError::NotFound(
                format!("Player injury history record {} not found for update_progress", id)
            )
        );
    }

    Ok(())
}

pub async fn update_progress_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
    days_remaining: u32,
    observation_days_remaining: u32,
    status: &str
) -> PersistenceResult<()> {
    let result = sqlx
        ::query(
            r#"UPDATE player_injury_history SET
            days_remaining = ?,
            observation_days_remaining = ?,
            status = ?
        WHERE id = ?"#
        )
        .bind(days_remaining as i32)
        .bind(observation_days_remaining as i32)
        .bind(status)
        .bind(id.to_string())
        .execute(&mut **tx).await?;

    if result.rows_affected() == 0 {
        return Err(
            PersistenceError::NotFound(
                format!("Player injury history record {} not found for update_progress", id)
            )
        );
    }

    Ok(())
}

pub async fn mark_resolved(
    pool: &SqlitePool,
    id: Uuid,
    resolved_at_unix_seconds: i64
) -> PersistenceResult<()> {
    let result = sqlx
        ::query(
            r#"UPDATE player_injury_history SET
            days_remaining = 0,
            observation_days_remaining = 0,
            status = 'Resolved',
            resolved_at_unix_seconds = ?
        WHERE id = ?"#
        )
        .bind(resolved_at_unix_seconds)
        .bind(id.to_string())
        .execute(pool).await?;

    if result.rows_affected() == 0 {
        return Err(
            PersistenceError::NotFound(
                format!("Player injury history record {} not found for mark_resolved", id)
            )
        );
    }

    Ok(())
}

pub async fn mark_resolved_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: Uuid,
    resolved_at_unix_seconds: i64
) -> PersistenceResult<()> {
    let result = sqlx
        ::query(
            r#"UPDATE player_injury_history SET
            days_remaining = 0,
            observation_days_remaining = 0,
            status = 'Resolved',
            resolved_at_unix_seconds = ?
        WHERE id = ?"#
        )
        .bind(resolved_at_unix_seconds)
        .bind(id.to_string())
        .execute(&mut **tx).await?;

    if result.rows_affected() == 0 {
        return Err(
            PersistenceError::NotFound(
                format!("Player injury history record {} not found for mark_resolved", id)
            )
        );
    }

    Ok(())
}