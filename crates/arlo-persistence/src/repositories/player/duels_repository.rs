use crate::error::PersistenceResult;
use crate::models::{MatchPlayerDuelByKindRow, MatchPlayerDuelRow};
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const DUEL_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "total_duels",
    "total_wins",
    "total_losses",
    "win_rate",
    "attacker_duels",
    "attacker_wins",
    "attacker_losses",
    "attacker_win_rate",
    "defender_duels",
    "defender_wins",
    "defender_losses",
    "defender_win_rate",
];

const DUEL_BY_KIND_COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "duel_kind",
    "total",
    "wins",
    "losses",
    "as_attacker_wins",
    "as_attacker_losses",
    "as_defender_wins",
    "as_defender_losses",
    "win_rate",
    "attacker_win_rate",
    "defender_win_rate",
];

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
    execute_batch_insert(tx, "match_player_duels", DUEL_COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.total_duels);
        b.push_bind(row.total_wins);
        b.push_bind(row.total_losses);
        b.push_bind(row.win_rate);
        b.push_bind(row.attacker_duels);
        b.push_bind(row.attacker_wins);
        b.push_bind(row.attacker_losses);
        b.push_bind(row.attacker_win_rate);
        b.push_bind(row.defender_duels);
        b.push_bind(row.defender_wins);
        b.push_bind(row.defender_losses);
        b.push_bind(row.defender_win_rate);
    })
    .await
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
    execute_batch_insert(
        tx,
        "match_player_duels_by_kind",
        DUEL_BY_KIND_COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(&row.player_id);
            b.push_bind(&row.duel_kind);
            b.push_bind(row.total);
            b.push_bind(row.wins);
            b.push_bind(row.losses);
            b.push_bind(row.as_attacker_wins);
            b.push_bind(row.as_attacker_losses);
            b.push_bind(row.as_defender_wins);
            b.push_bind(row.as_defender_losses);
            b.push_bind(row.win_rate);
            b.push_bind(row.attacker_win_rate);
            b.push_bind(row.defender_win_rate);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerDuelRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerDuelRow>(
        r#"SELECT
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
        FROM match_player_duels
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
) -> PersistenceResult<Option<MatchPlayerDuelRow>> {
    let row = sqlx::query_as::<_, MatchPlayerDuelRow>(
        r#"SELECT
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
        FROM match_player_duels
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_kind_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerDuelByKindRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerDuelByKindRow>(
        r#"SELECT
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
        FROM match_player_duels_by_kind
        WHERE match_id = ?
        ORDER BY player_id ASC, duel_kind ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_by_kind_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerDuelByKindRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerDuelByKindRow>(
        r#"SELECT
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
        FROM match_player_duels_by_kind
        WHERE match_id = ? AND player_id = ?
        ORDER BY duel_kind ASC"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
