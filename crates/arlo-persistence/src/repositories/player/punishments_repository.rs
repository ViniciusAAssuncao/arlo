use crate::error::PersistenceResult;
use crate::models::MatchPlayerPunishmentRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerPunishmentRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_punishments (
            id,
            match_id,
            player_id,
            yardage_loss_count,
            loss_of_down_count,
            loss_of_drive_count,
            time_penalty_count,
            expulsion_count,
            invalidate_play_count,
            total_yardage_loss_mirim,
            total_loss_of_down_count,
            total_time_penalty_seconds,
            total_loss_of_drive_count
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.yardage_loss_count)
    .bind(row.loss_of_down_count)
    .bind(row.loss_of_drive_count)
    .bind(row.time_penalty_count)
    .bind(row.expulsion_count)
    .bind(row.invalidate_play_count)
    .bind(row.total_yardage_loss_mirim)
    .bind(row.total_loss_of_down_count)
    .bind(row.total_time_penalty_seconds)
    .bind(row.total_loss_of_drive_count)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerPunishmentRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
