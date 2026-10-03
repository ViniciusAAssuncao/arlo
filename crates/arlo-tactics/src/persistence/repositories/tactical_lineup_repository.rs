mod insert;
mod load;
use crate::error::{TacticsError, TacticsResult};
use crate::lineup::TacticalLineup;
use crate::persistence::models::rows::TacticalLineupRow;
use arlo_db::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
pub use insert::insert;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> TacticsResult<Option<TacticalLineup>> {
    let row = fetch_optional_by_param::<TacticalLineupRow>(
        pool,
        "SELECT id, team_id, formation_id, name, created_at_unix_seconds FROM tactical_lineups WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let formation_id = Uuid::parse_str(&row.formation_id)?;
    let formation = arlo_db::repositories::formation::get_by_id(pool, formation_id)
        .await?
        .ok_or_else(|| {
            TacticsError::NotFound(format!(
                "Formation {} not found for lineup {}",
                formation_id, id
            ))
        })?;

    let assignments = load::assignments(pool, &row.id, &formation).await?;

    Ok(Some(row.to_domain(assignments)?))
}

pub async fn get_latest_by_team_id(
    pool: &SqlitePool,
    team_id: Uuid,
) -> TacticsResult<Option<TacticalLineup>> {
    let id = sqlx::query_scalar::<_, String>(
        "SELECT id FROM tactical_lineups WHERE team_id = ? ORDER BY created_at_unix_seconds DESC, rowid DESC LIMIT 1",
    ).bind(team_id.to_string()).fetch_optional(pool).await?;
    match id {
        Some(id) => get_by_id(pool, Uuid::parse_str(&id)?).await,
        None => Ok(None),
    }
}

pub async fn list_by_team_id(
    pool: &SqlitePool,
    team_id: Uuid,
) -> TacticsResult<Vec<TacticalLineup>> {
    let rows = fetch_all_by_param::<TacticalLineupRow>(
        pool,
        "SELECT id, team_id, formation_id, name, created_at_unix_seconds FROM tactical_lineups WHERE team_id = ? ORDER BY created_at_unix_seconds ASC",
        &team_id.to_string(),
    )
    .await?;

    let mut lineups = Vec::with_capacity(rows.len());
    for row in rows {
        let formation_id = Uuid::parse_str(&row.formation_id)?;
        let formation = arlo_db::repositories::formation::get_by_id(pool, formation_id)
            .await?
            .ok_or_else(|| {
                TacticsError::NotFound(format!(
                    "Formation {} not found for lineup {}",
                    formation_id, row.id
                ))
            })?;

        let assignments = load::assignments(pool, &row.id, &formation).await?;

        lineups.push(row.to_domain(assignments)?);
    }

    Ok(lineups)
}

pub async fn delete(pool: &SqlitePool, id: Uuid) -> TacticsResult<()> {
    sqlx::query("DELETE FROM tactical_lineups WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}
