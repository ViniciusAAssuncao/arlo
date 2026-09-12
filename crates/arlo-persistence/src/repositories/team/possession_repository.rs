use crate::error::PersistenceResult;
use crate::models::MatchTeamPossessionRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTeamPossessionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_team_possession (
            id,
            match_id,
            team_id,
            total_possession_seconds
        ) VALUES (?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(row.total_possession_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTeamPossessionRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}