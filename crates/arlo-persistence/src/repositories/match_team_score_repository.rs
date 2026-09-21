use crate::error::PersistenceResult;
use crate::models::MatchTeamScoreRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "team_id",
    "is_home",
    "goal_points",
    "field_goals",
    "field_points",
    "total_points",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTeamScoreRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_team_scores (
            id,
            match_id,
            team_id,
            is_home,
            goal_points,
            field_goals,
            field_points,
            total_points
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(row.is_home)
    .bind(row.goal_points)
    .bind(row.field_goals)
    .bind(row.field_points)
    .bind(row.total_points)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchTeamScoreRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_team_scores", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.team_id);
        b.push_bind(row.is_home);
        b.push_bind(row.goal_points);
        b.push_bind(row.field_goals);
        b.push_bind(row.field_points);
        b.push_bind(row.total_points);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchTeamScoreRow>> {
    let rows = sqlx::query_as::<_, MatchTeamScoreRow>(
        r#"SELECT
            id,
            match_id,
            team_id,
            is_home,
            goal_points,
            field_goals,
            field_points,
            total_points
        FROM match_team_scores
        WHERE match_id = ?
        ORDER BY is_home DESC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}