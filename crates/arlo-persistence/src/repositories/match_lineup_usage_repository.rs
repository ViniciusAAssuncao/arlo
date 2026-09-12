use crate::error::PersistenceResult;
use crate::models::MatchLineupUsageRow;
use sqlx::{Sqlite, Transaction};

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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
