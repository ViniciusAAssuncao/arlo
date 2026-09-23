use crate::error::PersistenceResult;
use crate::models::{MatchPlayerKickFoulByDecisionRow, MatchPlayerKickFoulRow};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const KICK_FOUL_COLUMNS: &[&str] = &["id", "match_id", "player_id", "kick_foul_takes"];

const KICK_FOUL_BY_DECISION_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "decision_kind",
    "takes_count",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerKickFoulRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_kick_fouls (
            id,
            match_id,
            player_id,
            kick_foul_takes
        ) VALUES (?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.kick_foul_takes)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerKickFoulRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_player_kick_fouls",
        KICK_FOUL_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(row.kick_foul_takes);
        },
    )
    .await
}

pub async fn insert_by_decision(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerKickFoulByDecisionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_kick_fouls_by_decision (
            id,
            match_id,
            player_id,
            decision_kind,
            takes_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(&row.decision_kind)
    .bind(row.takes_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_by_decision_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerKickFoulByDecisionRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_player_kick_fouls_by_decision",
        KICK_FOUL_BY_DECISION_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(&row.decision_kind);
            b.push_bind(row.takes_count);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerKickFoulRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerKickFoulRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            kick_foul_takes
        FROM match_player_kick_fouls
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
) -> PersistenceResult<Option<MatchPlayerKickFoulRow>> {
    let row = sqlx::query_as::<_, MatchPlayerKickFoulRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            kick_foul_takes
        FROM match_player_kick_fouls
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_decision_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerKickFoulByDecisionRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerKickFoulByDecisionRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            decision_kind,
            takes_count
        FROM match_player_kick_fouls_by_decision
        WHERE match_id = ?
        ORDER BY player_id ASC, decision_kind ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_by_decision_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerKickFoulByDecisionRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerKickFoulByDecisionRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            decision_kind,
            takes_count
        FROM match_player_kick_fouls_by_decision
        WHERE match_id = ? AND player_id = ?
        ORDER BY decision_kind ASC"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
