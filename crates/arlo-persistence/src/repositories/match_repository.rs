use crate::error::PersistenceResult;
use crate::models::MatchRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(tx: &mut Transaction<'_, Sqlite>, row: &MatchRow) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO matches (
            id,
            fixture_id,
            home_team_id,
            away_team_id,
            venue_id,
            pitch_length_mirim,
            pitch_width_mirim,
            match_seed,
            format_regulation_periods,
            format_regulation_period_duration_seconds,
            format_allows_overtime,
            format_overtime_periods,
            format_overtime_period_duration_seconds,
            head_referee_id,
            peace_referee_id,
            final_period,
            went_to_overtime,
            completed_at_unix_seconds,
            created_at_unix_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.fixture_id)
    .bind(&row.home_team_id)
    .bind(&row.away_team_id)
    .bind(&row.venue_id)
    .bind(row.pitch_length_mirim)
    .bind(row.pitch_width_mirim)
    .bind(row.match_seed)
    .bind(row.format_regulation_periods)
    .bind(row.format_regulation_period_duration_seconds)
    .bind(row.format_allows_overtime)
    .bind(row.format_overtime_periods)
    .bind(row.format_overtime_period_duration_seconds)
    .bind(&row.head_referee_id)
    .bind(&row.peace_referee_id)
    .bind(row.final_period)
    .bind(row.went_to_overtime)
    .bind(row.completed_at_unix_seconds)
    .bind(row.created_at_unix_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_by_id(pool: &SqlitePool, match_id: Uuid) -> PersistenceResult<Option<MatchRow>> {
    let row = sqlx::query_as::<_, MatchRow>(
        r#"SELECT
            id,
            fixture_id,
            home_team_id,
            away_team_id,
            venue_id,
            pitch_length_mirim,
            pitch_width_mirim,
            match_seed,
            format_regulation_periods,
            format_regulation_period_duration_seconds,
            format_allows_overtime,
            format_overtime_periods,
            format_overtime_period_duration_seconds,
            head_referee_id,
            peace_referee_id,
            final_period,
            went_to_overtime,
            completed_at_unix_seconds,
            created_at_unix_seconds
        FROM matches
        WHERE id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_by_fixture_id(
    pool: &SqlitePool,
    fixture_id: Uuid,
) -> PersistenceResult<Option<MatchRow>> {
    let row = sqlx::query_as::<_, MatchRow>(
        r#"SELECT
            id,
            fixture_id,
            home_team_id,
            away_team_id,
            venue_id,
            pitch_length_mirim,
            pitch_width_mirim,
            match_seed,
            format_regulation_periods,
            format_regulation_period_duration_seconds,
            format_allows_overtime,
            format_overtime_periods,
            format_overtime_period_duration_seconds,
            head_referee_id,
            peace_referee_id,
            final_period,
            went_to_overtime,
            completed_at_unix_seconds,
            created_at_unix_seconds
        FROM matches
        WHERE fixture_id = ?"#,
    )
    .bind(fixture_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_fixture_ids(
    pool: &SqlitePool,
    fixture_ids: &[Uuid],
) -> PersistenceResult<Vec<MatchRow>> {
    if fixture_ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat("?")
        .take(fixture_ids.len())
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        r#"SELECT
            id,
            fixture_id,
            home_team_id,
            away_team_id,
            venue_id,
            pitch_length_mirim,
            pitch_width_mirim,
            match_seed,
            format_regulation_periods,
            format_regulation_period_duration_seconds,
            format_allows_overtime,
            format_overtime_periods,
            format_overtime_period_duration_seconds,
            head_referee_id,
            peace_referee_id,
            final_period,
            went_to_overtime,
            completed_at_unix_seconds,
            created_at_unix_seconds
        FROM matches
        WHERE fixture_id IN ({})"#,
        placeholders
    );

    let mut query = sqlx::query_as::<_, MatchRow>(&sql);
    for id in fixture_ids {
        query = query.bind(id.to_string());
    }

    let rows = query.fetch_all(pool).await?;
    Ok(rows)
}
