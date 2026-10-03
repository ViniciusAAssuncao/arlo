use crate::error::PersistenceResult;
use crate::models::MatchTacticalRealignmentRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTacticalRealignmentRow,
) -> PersistenceResult<()> {
    sqlx::query("INSERT INTO match_tactical_realignments (match_id, sequence_number, team_id, period, seconds_in_period, total_elapsed_seconds, assignments_json) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(&row.match_id).bind(row.sequence_number).bind(&row.team_id)
        .bind(row.period).bind(row.seconds_in_period).bind(row.total_elapsed_seconds)
        .bind(sqlx::types::Json(&row.assignments)).execute(&mut **tx).await?;
    Ok(())
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchTacticalRealignmentRow>> {
    Ok(sqlx::query_as::<_, MatchTacticalRealignmentRow>("SELECT match_id, sequence_number, team_id, period, seconds_in_period, total_elapsed_seconds, assignments_json AS assignments FROM match_tactical_realignments WHERE match_id = ? ORDER BY sequence_number")
        .bind(match_id.to_string()).fetch_all(pool).await?)
}
