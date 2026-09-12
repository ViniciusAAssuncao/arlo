use crate::error::PersistenceResult;
use crate::models::{MatchPlayerArtrineDecisionByKindRow, MatchPlayerArtrineDecisionRow};
use sqlx::{Sqlite, Transaction};

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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
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
    for row in rows {
        insert_by_kind(tx, row).await?;
    }
    Ok(())
}
