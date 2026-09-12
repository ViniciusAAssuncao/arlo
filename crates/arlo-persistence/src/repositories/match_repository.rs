use crate::error::PersistenceResult;
use crate::models::MatchRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(tx: &mut Transaction<'_, Sqlite>, row: &MatchRow) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO matches (
            id,
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
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
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
