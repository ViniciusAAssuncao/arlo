use crate::error::PersistenceResult;
use crate::models::{MatchPlayerFoulByOriginRow, MatchPlayerFoulRow};
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerFoulRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_fouls (
            id,
            match_id,
            player_id,
            fouls_committed,
            fouls_drawn,
            correct_calls_committed,
            incorrect_calls_committed
        ) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.fouls_committed)
    .bind(row.fouls_drawn)
    .bind(row.correct_calls_committed)
    .bind(row.incorrect_calls_committed)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerFoulRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_by_origin(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerFoulByOriginRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_fouls_by_origin (
            id,
            match_id,
            player_id,
            origin,
            fouls_count
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(&row.origin)
    .bind(row.fouls_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_by_origin_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerFoulByOriginRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_by_origin(tx, row).await?;
    }
    Ok(())
}
