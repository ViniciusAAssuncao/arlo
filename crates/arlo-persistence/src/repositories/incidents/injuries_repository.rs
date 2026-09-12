use crate::error::PersistenceResult;
use crate::models::MatchInjuryRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchInjuryRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_injuries (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            player_id,
            team_id,
            mechanism,
            body_region,
            severity_grade,
            injury_definition_id,
            trigger_probability
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.player_id)
    .bind(&row.team_id)
    .bind(&row.mechanism)
    .bind(&row.body_region)
    .bind(&row.severity_grade)
    .bind(&row.injury_definition_id)
    .bind(row.trigger_probability)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchInjuryRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
