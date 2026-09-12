use crate::error::PersistenceResult;
use crate::models::incidents::MatchAddedTimeRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchAddedTimeRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"
        INSERT INTO match_added_time (
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
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
