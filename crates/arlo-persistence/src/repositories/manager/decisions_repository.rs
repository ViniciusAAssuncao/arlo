use crate::error::PersistenceResult;
use crate::models::{
    MatchManagerDecisionRow, MatchManagerPlayCallByCategoryRow, MatchManagerSubstitutionByReasonRow,
};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const DECISION_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "team_id",
    "substitutions_made",
    "time_calls_used",
    "challenges_won",
    "challenges_lost",
    "tactical_profile_switches",
];

const SUBSTITUTION_BY_REASON_COLUMNS: &[&str] =
    &["id", "match_id", "team_id", "reason", "substitutions_count"];

const PLAY_CALL_BY_CATEGORY_COLUMNS: &[&str] =
    &["id", "match_id", "team_id", "category", "play_calls_count"];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchManagerDecisionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_manager_decisions (
            id,
            match_id,
            team_id,
            substitutions_made,
            time_calls_used,
            challenges_won,
            challenges_lost,
            tactical_profile_switches
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(row.substitutions_made)
    .bind(row.time_calls_used)
    .bind(row.challenges_won)
    .bind(row.challenges_lost)
    .bind(row.tactical_profile_switches)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchManagerDecisionRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_manager_decisions",
        DECISION_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.team_id);
            b.push_bind(row.substitutions_made);
            b.push_bind(row.time_calls_used);
            b.push_bind(row.challenges_won);
            b.push_bind(row.challenges_lost);
            b.push_bind(row.tactical_profile_switches);
        },
    )
    .await
}

pub async fn insert_substitutions_by_reason(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchManagerSubstitutionByReasonRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_manager_substitutions_by_reason (
            id,
            match_id,
            team_id,
            reason,
            substitutions_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(&row.reason)
    .bind(row.substitutions_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_substitutions_by_reason_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchManagerSubstitutionByReasonRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_manager_substitutions_by_reason",
        SUBSTITUTION_BY_REASON_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.team_id);
            b.push_bind(&row.reason);
            b.push_bind(row.substitutions_count);
        },
    )
    .await
}

pub async fn insert_play_calls_by_category(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchManagerPlayCallByCategoryRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_manager_play_calls_by_category (
            id,
            match_id,
            team_id,
            category,
            play_calls_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(&row.category)
    .bind(row.play_calls_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_play_calls_by_category_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchManagerPlayCallByCategoryRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_manager_play_calls_by_category",
        PLAY_CALL_BY_CATEGORY_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.team_id);
            b.push_bind(&row.category);
            b.push_bind(row.play_calls_count);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchManagerDecisionRow>> {
    let rows = sqlx::query_as::<_, MatchManagerDecisionRow>(
        r#"SELECT
            id,
            match_id,
            team_id,
            substitutions_made,
            time_calls_used,
            challenges_won,
            challenges_lost,
            tactical_profile_switches
        FROM match_manager_decisions
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_substitutions_by_reason_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchManagerSubstitutionByReasonRow>> {
    let rows = sqlx::query_as::<_, MatchManagerSubstitutionByReasonRow>(
        r#"SELECT
            id,
            match_id,
            team_id,
            reason,
            substitutions_count
        FROM match_manager_substitutions_by_reason
        WHERE match_id = ?
        ORDER BY team_id ASC, reason ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_play_calls_by_category_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchManagerPlayCallByCategoryRow>> {
    let rows = sqlx::query_as::<_, MatchManagerPlayCallByCategoryRow>(
        r#"SELECT
            id,
            match_id,
            team_id,
            category,
            play_calls_count
        FROM match_manager_play_calls_by_category
        WHERE match_id = ?
        ORDER BY team_id ASC, category ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
