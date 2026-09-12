use crate::error::PersistenceResult;
use crate::models::MatchPlayerTouchesRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerTouchesRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_touches (
            id,
            match_id,
            player_id,
            passes_attempted,
            passes_received,
            drives_recorded,
            recoveries,
            scoring_attempts,
            total_touches,
            turnovers_conceded
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.passes_attempted)
    .bind(row.passes_received)
    .bind(row.drives_recorded)
    .bind(row.recoveries)
    .bind(row.scoring_attempts)
    .bind(row.total_touches)
    .bind(row.turnovers_conceded)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerTouchesRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
