use crate::error::PersistenceResult;
use crate::models::MatchPlayerAssistRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerAssistRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_assists (
            id,
            match_id,
            player_id,
            goalpoint_assists
        ) VALUES (?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.goalpoint_assists)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerAssistRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
