use crate::error::PersistenceResult;
use crate::models::MatchTeamScoreRow;
use sqlx::{Sqlite, Transaction};

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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
