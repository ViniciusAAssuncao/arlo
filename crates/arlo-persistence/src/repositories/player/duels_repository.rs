use crate::error::PersistenceResult;
use crate::models::{MatchPlayerDuelByKindRow, MatchPlayerDuelRow};
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerDuelRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_duels (
            id,
            match_id,
            player_id,
            total_duels,
            total_wins,
            total_losses,
            win_rate,
            attacker_duels,
            attacker_wins,
            attacker_losses,
            attacker_win_rate,
            defender_duels,
            defender_wins,
            defender_losses,
            defender_win_rate
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.total_duels)
    .bind(row.total_wins)
    .bind(row.total_losses)
    .bind(row.win_rate)
    .bind(row.attacker_duels)
    .bind(row.attacker_wins)
    .bind(row.attacker_losses)
    .bind(row.attacker_win_rate)
    .bind(row.defender_duels)
    .bind(row.defender_wins)
    .bind(row.defender_losses)
    .bind(row.defender_win_rate)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerDuelRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn insert_by_kind(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerDuelByKindRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_duels_by_kind (
            id,
            match_id,
            player_id,
            duel_kind,
            total,
            wins,
            losses,
            as_attacker_wins,
            as_attacker_losses,
            as_defender_wins,
            as_defender_losses,
            win_rate,
            attacker_win_rate,
            defender_win_rate
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(&row.duel_kind)
    .bind(row.total)
    .bind(row.wins)
    .bind(row.losses)
    .bind(row.as_attacker_wins)
    .bind(row.as_attacker_losses)
    .bind(row.as_defender_wins)
    .bind(row.as_defender_losses)
    .bind(row.win_rate)
    .bind(row.attacker_win_rate)
    .bind(row.defender_win_rate)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_by_kind_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerDuelByKindRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert_by_kind(tx, row).await?;
    }
    Ok(())
}
