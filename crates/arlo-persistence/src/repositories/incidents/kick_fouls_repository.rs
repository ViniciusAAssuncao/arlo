use crate::error::PersistenceResult;
use crate::models::{MatchKickFoulAwardRow, MatchKickFoulDecisionRow};
use sqlx::{Sqlite, Transaction};

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
            scoring_tier,
            spot_x_mirim,
            spot_y_mirim
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.awarded_team_id)
    .bind(&row.offending_team_id)
    .bind(&row.scoring_tier)
    .bind(row.spot_x_mirim)
    .bind(row.spot_y_mirim)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_awards_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchKickFoulAwardRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_award(tx, row).await?;
    }
    Ok(())
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
    for row in rows {
        insert_decision(tx, row).await?;
    }
    Ok(())
}
