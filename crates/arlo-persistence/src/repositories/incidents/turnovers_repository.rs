use crate::error::PersistenceResult;
use crate::models::MatchTurnoverRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "previous_offense_team_id",
    "new_offense_team_id",
    "recovering_player_id",
    "lost_by_player_id",
    "in_live_play",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTurnoverRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_turnovers (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            previous_offense_team_id,
            new_offense_team_id,
            recovering_player_id,
            lost_by_player_id,
            in_live_play
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.previous_offense_team_id)
    .bind(&row.new_offense_team_id)
    .bind(&row.recovering_player_id)
    .bind(&row.lost_by_player_id)
    .bind(row.in_live_play)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTurnoverRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_turnovers", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(row.sequence_number);
        b.push_bind(row.period);
        b.push_bind(row.seconds_in_period);
        b.push_bind(&row.previous_offense_team_id);
        b.push_bind(&row.new_offense_team_id);
        b.push_bind(&row.recovering_player_id);
        b.push_bind(&row.lost_by_player_id);
        b.push_bind(row.in_live_play);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchTurnoverRow>> {
    let rows = sqlx::query_as::<_, MatchTurnoverRow>(
        r#"SELECT
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            previous_offense_team_id,
            new_offense_team_id,
            recovering_player_id,
            lost_by_player_id,
            in_live_play
        FROM match_turnovers
        WHERE match_id = ?
        ORDER BY sequence_number ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}