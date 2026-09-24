use crate::error::PersistenceResult;
use crate::models::{MatchPlayerArtrineDecisionByKindRow, MatchPlayerArtrineDecisionRow};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const DECISION_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "total_decisions",
    "total_successful_decisions",
    "total_failed_decisions",
    "total_mirins_advanced",
    "total_points_generated",
    "goal_points_generated",
    "field_points_generated",
    "field_goals_generated",
    "success_rate",
    "average_mirins_per_decision",
    "average_points_per_decision",
];

const DECISION_BY_KIND_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "decision_kind",
    "total",
    "successful",
    "failed",
    "mirins_advanced",
    "points_generated",
    "success_rate",
    "average_mirins_advanced",
    "average_points_generated",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerArtrineDecisionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_artrine_decisions (
            id,
            match_id,
            player_id,
            total_decisions,
            total_successful_decisions,
            total_failed_decisions,
            total_mirins_advanced,
            total_points_generated,
            goal_points_generated,
            field_points_generated,
            field_goals_generated,
            success_rate,
            average_mirins_per_decision,
            average_points_per_decision
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.total_decisions)
    .bind(row.total_successful_decisions)
    .bind(row.total_failed_decisions)
    .bind(row.total_mirins_advanced)
    .bind(row.total_points_generated)
    .bind(row.goal_points_generated)
    .bind(row.field_points_generated)
    .bind(row.field_goals_generated)
    .bind(row.success_rate)
    .bind(row.average_mirins_per_decision)
    .bind(row.average_points_per_decision)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerArtrineDecisionRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_player_artrine_decisions",
        DECISION_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(row.total_decisions);
            b.push_bind(row.total_successful_decisions);
            b.push_bind(row.total_failed_decisions);
            b.push_bind(row.total_mirins_advanced);
            b.push_bind(row.total_points_generated);
            b.push_bind(row.goal_points_generated);
            b.push_bind(row.field_points_generated);
            b.push_bind(row.field_goals_generated);
            b.push_bind(row.success_rate);
            b.push_bind(row.average_mirins_per_decision);
            b.push_bind(row.average_points_per_decision);
        },
    )
    .await
}

pub async fn insert_by_kind(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerArtrineDecisionByKindRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_artrine_decisions_by_kind (
            id,
            match_id,
            player_id,
            decision_kind,
            total,
            successful,
            failed,
            mirins_advanced,
            points_generated,
            success_rate,
            average_mirins_advanced,
            average_points_generated
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(&row.decision_kind)
    .bind(row.total)
    .bind(row.successful)
    .bind(row.failed)
    .bind(row.mirins_advanced)
    .bind(row.points_generated)
    .bind(row.success_rate)
    .bind(row.average_mirins_advanced)
    .bind(row.average_points_generated)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_by_kind_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerArtrineDecisionByKindRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_player_artrine_decisions_by_kind",
        DECISION_BY_KIND_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(&row.decision_kind);
            b.push_bind(row.total);
            b.push_bind(row.successful);
            b.push_bind(row.failed);
            b.push_bind(row.mirins_advanced);
            b.push_bind(row.points_generated);
            b.push_bind(row.success_rate);
            b.push_bind(row.average_mirins_advanced);
            b.push_bind(row.average_points_generated);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerArtrineDecisionRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerArtrineDecisionRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            total_decisions,
            total_successful_decisions,
            total_failed_decisions,
            total_mirins_advanced,
            total_points_generated,
            goal_points_generated,
            field_points_generated,
            field_goals_generated,
            success_rate,
            average_mirins_per_decision,
            average_points_per_decision
        FROM match_player_artrine_decisions
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
) -> PersistenceResult<Option<MatchPlayerArtrineDecisionRow>> {
    let row = sqlx::query_as::<_, MatchPlayerArtrineDecisionRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            total_decisions,
            total_successful_decisions,
            total_failed_decisions,
            total_mirins_advanced,
            total_points_generated,
            goal_points_generated,
            field_points_generated,
            field_goals_generated,
            success_rate,
            average_mirins_per_decision,
            average_points_per_decision
        FROM match_player_artrine_decisions
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_kind_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerArtrineDecisionByKindRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerArtrineDecisionByKindRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            decision_kind,
            total,
            successful,
            failed,
            mirins_advanced,
            points_generated,
            success_rate,
            average_mirins_advanced,
            average_points_generated
        FROM match_player_artrine_decisions_by_kind
        WHERE match_id = ?
        ORDER BY player_id ASC, decision_kind ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_by_kind_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerArtrineDecisionByKindRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerArtrineDecisionByKindRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            decision_kind,
            total,
            successful,
            failed,
            mirins_advanced,
            points_generated,
            success_rate,
            average_mirins_advanced,
            average_points_generated
        FROM match_player_artrine_decisions_by_kind
        WHERE match_id = ? AND player_id = ?
        ORDER BY decision_kind ASC"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
