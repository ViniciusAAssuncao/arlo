use crate::error::PersistenceResult;
use crate::models::MatchPlayerPunishmentRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "yardage_loss_count",
    "loss_of_down_count",
    "loss_of_drive_count",
    "time_penalty_count",
    "expulsion_count",
    "invalidate_play_count",
    "total_yardage_loss_mirim",
    "total_loss_of_down_count",
    "total_time_penalty_seconds",
    "total_loss_of_drive_count",
];

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
    execute_batch_insert(tx, "match_player_punishments", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.yardage_loss_count);
        b.push_bind(row.loss_of_down_count);
        b.push_bind(row.loss_of_drive_count);
        b.push_bind(row.time_penalty_count);
        b.push_bind(row.expulsion_count);
        b.push_bind(row.invalidate_play_count);
        b.push_bind(row.total_yardage_loss_mirim);
        b.push_bind(row.total_loss_of_down_count);
        b.push_bind(row.total_time_penalty_seconds);
        b.push_bind(row.total_loss_of_drive_count);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerPunishmentRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerPunishmentRow>(
        r#"SELECT
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
        FROM match_player_punishments
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
) -> PersistenceResult<Option<MatchPlayerPunishmentRow>> {
    let row = sqlx::query_as::<_, MatchPlayerPunishmentRow>(
        r#"SELECT
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
        FROM match_player_punishments
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
