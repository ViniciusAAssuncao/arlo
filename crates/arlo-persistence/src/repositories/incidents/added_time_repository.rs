use crate::error::PersistenceResult;
use crate::models::incidents::MatchAddedTimeRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "added_time_seconds",
    "foul_count",
    "injury_count",
    "challenge_count",
    "time_call_count",
    "kick_foul_count",
    "scoring_count",
    "accumulated_dead_ball_seconds",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchAddedTimeRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_added_time (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            added_time_seconds,
            foul_count,
            injury_count,
            challenge_count,
            time_call_count,
            kick_foul_count,
            scoring_count,
            accumulated_dead_ball_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(row.added_time_seconds)
    .bind(row.foul_count)
    .bind(row.injury_count)
    .bind(row.challenge_count)
    .bind(row.time_call_count)
    .bind(row.kick_foul_count)
    .bind(row.scoring_count)
    .bind(row.accumulated_dead_ball_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchAddedTimeRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_added_time", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(row.sequence_number);
        b.push_bind(row.period);
        b.push_bind(row.seconds_in_period);
        b.push_bind(row.added_time_seconds);
        b.push_bind(row.foul_count);
        b.push_bind(row.injury_count);
        b.push_bind(row.challenge_count);
        b.push_bind(row.time_call_count);
        b.push_bind(row.kick_foul_count);
        b.push_bind(row.scoring_count);
        b.push_bind(row.accumulated_dead_ball_seconds);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchAddedTimeRow>> {
    let rows = sqlx::query_as::<_, MatchAddedTimeRow>(
        r#"SELECT
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            added_time_seconds,
            foul_count,
            injury_count,
            challenge_count,
            time_call_count,
            kick_foul_count,
            scoring_count,
            accumulated_dead_ball_seconds
        FROM match_added_time
        WHERE match_id = ?
        ORDER BY sequence_number ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
