use crate::error::PersistenceResult;
use crate::models::{MatchPlayerScoringAttemptByPostRow, MatchPlayerScoringAttemptRow};
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerScoringAttemptRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_scoring_attempts (
            id,
            match_id,
            player_id,
            attempts,
            converted,
            missed,
            conversion_rate,
            miss_rate,
            goal_points_scored,
            field_points_scored,
            field_goals_scored,
            total_points_scored
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.attempts)
    .bind(row.converted)
    .bind(row.missed)
    .bind(row.conversion_rate)
    .bind(row.miss_rate)
    .bind(row.goal_points_scored)
    .bind(row.field_points_scored)
    .bind(row.field_goals_scored)
    .bind(row.total_points_scored)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerScoringAttemptRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_by_post(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerScoringAttemptByPostRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_scoring_attempts_by_post (
            id,
            match_id,
            player_id,
            scoring_post,
            attempts,
            converted,
            missed,
            conversion_rate
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(&row.scoring_post)
    .bind(row.attempts)
    .bind(row.converted)
    .bind(row.missed)
    .bind(row.conversion_rate)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_by_post_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerScoringAttemptByPostRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_by_post(tx, row).await?;
    }
    Ok(())
}
