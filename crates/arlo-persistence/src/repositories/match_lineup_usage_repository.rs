use crate::error::PersistenceResult;
use crate::models::MatchLineupUsageRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "team_id",
    "tactical_lineup_id",
    "formation_id",
    "team_tactical_profile_id",
    "manager_id",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchLineupUsageRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_team_lineup_usage (
            id,
            match_id,
            team_id,
            tactical_lineup_id,
            formation_id,
            team_tactical_profile_id,
            manager_id
        ) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(&row.tactical_lineup_id)
    .bind(&row.formation_id)
    .bind(&row.team_tactical_profile_id)
    .bind(&row.manager_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchLineupUsageRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_team_lineup_usage", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.team_id);
        b.push_bind(&row.tactical_lineup_id);
        b.push_bind(&row.formation_id);
        b.push_bind(&row.team_tactical_profile_id);
        b.push_bind(&row.manager_id);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchLineupUsageRow>> {
    let rows = sqlx::query_as::<_, MatchLineupUsageRow>(
        r#"SELECT
            id,
            match_id,
            team_id,
            tactical_lineup_id,
            formation_id,
            team_tactical_profile_id,
            manager_id
        FROM match_team_lineup_usage
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}