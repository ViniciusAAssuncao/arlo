use crate::error::PersistenceResult;
use crate::models::{MatchKickFoulAwardRow, MatchKickFoulDecisionRow};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const AWARD_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "awarded_team_id",
    "offending_team_id",
    "scoring_tier",
];

const DECISION_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "taker_id",
    "decision",
];

pub async fn insert_award(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchKickFoulAwardRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_kick_foul_awards (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            awarded_team_id,
            offending_team_id,
            scoring_tier
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.awarded_team_id)
    .bind(&row.offending_team_id)
    .bind(&row.scoring_tier)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_awards_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchKickFoulAwardRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_kick_foul_awards",
        AWARD_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(row.sequence_number);
            b.push_bind(row.period);
            b.push_bind(row.seconds_in_period);
            b.push_bind(&row.awarded_team_id);
            b.push_bind(&row.offending_team_id);
            b.push_bind(&row.scoring_tier);
        },
    )
    .await
}

pub async fn insert_decision(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchKickFoulDecisionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_kick_foul_decisions (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            taker_id,
            decision
        ) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.taker_id)
    .bind(&row.decision)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_decisions_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchKickFoulDecisionRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_kick_foul_decisions",
        DECISION_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(row.sequence_number);
            b.push_bind(row.period);
            b.push_bind(row.seconds_in_period);
            b.push_bind(&row.taker_id);
            b.push_bind(&row.decision);
        },
    )
    .await
}

pub async fn list_awards_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchKickFoulAwardRow>> {
    let rows = sqlx::query_as::<_, MatchKickFoulAwardRow>(
        r#"SELECT
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            awarded_team_id,
            offending_team_id,
            scoring_tier
        FROM match_kick_foul_awards
        WHERE match_id = ?
        ORDER BY sequence_number ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_decisions_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchKickFoulDecisionRow>> {
    let rows = sqlx::query_as::<_, MatchKickFoulDecisionRow>(
        r#"SELECT
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            taker_id,
            decision
        FROM match_kick_foul_decisions
        WHERE match_id = ?
        ORDER BY sequence_number ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
