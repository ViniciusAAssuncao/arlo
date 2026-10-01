use crate::error::PersistenceResult;
use crate::models::{MatchPlayerScoringAttemptByPostRow, MatchPlayerScoringAttemptRow};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const SCORING_ATTEMPT_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "attempts",
    "converted",
    "missed",
    "conversion_rate",
    "miss_rate",
    "goal_points_scored",
    "field_points_scored",
    "field_goals_scored",
    "total_points_scored",
];

const SCORING_ATTEMPT_BY_POST_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "scoring_post",
    "attempts",
    "converted",
    "missed",
    "conversion_rate",
];

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
    execute_batch_insert(
        tx,
        "match_player_scoring_attempts",
        SCORING_ATTEMPT_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(row.attempts);
            b.push_bind(row.converted);
            b.push_bind(row.missed);
            b.push_bind(row.conversion_rate);
            b.push_bind(row.miss_rate);
            b.push_bind(row.goal_points_scored);
            b.push_bind(row.field_points_scored);
            b.push_bind(row.field_goals_scored);
            b.push_bind(row.total_points_scored);
        },
    )
    .await
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
    execute_batch_insert(
        tx,
        "match_player_scoring_attempts_by_post",
        SCORING_ATTEMPT_BY_POST_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(&row.scoring_post);
            b.push_bind(row.attempts);
            b.push_bind(row.converted);
            b.push_bind(row.missed);
            b.push_bind(row.conversion_rate);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerScoringAttemptRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerScoringAttemptRow>(
        r#"SELECT
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
        FROM match_player_scoring_attempts
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Option<MatchPlayerScoringAttemptRow>> {
    let row = sqlx::query_as::<_, MatchPlayerScoringAttemptRow>(
        r#"SELECT
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
        FROM match_player_scoring_attempts
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_post_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerScoringAttemptByPostRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerScoringAttemptByPostRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            scoring_post,
            attempts,
            converted,
            missed,
            conversion_rate
        FROM match_player_scoring_attempts_by_post
        WHERE match_id = ?
        ORDER BY player_id ASC, scoring_post ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_by_post_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerScoringAttemptByPostRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerScoringAttemptByPostRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            scoring_post,
            attempts,
            converted,
            missed,
            conversion_rate
        FROM match_player_scoring_attempts_by_post
        WHERE match_id = ? AND player_id = ?
        ORDER BY scoring_post ASC"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
