use crate::error::PersistenceResult;
use crate::models::{MatchPlayerKickFoulByDecisionRow, MatchPlayerKickFoulRow};
use sqlx::{Sqlite, Transaction};

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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
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
    for row in rows {
        insert_by_decision(tx, row).await?;
    }
    Ok(())
}
