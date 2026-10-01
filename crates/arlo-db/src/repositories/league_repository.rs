use crate::error::{DbError, DbResult};
use crate::models::LeagueRow;
use crate::repositories::competition_repository;
use crate::repositories::fetch::{fetch_all, fetch_optional_by_param};
use arlo_domain::League;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_competition_id(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> DbResult<Option<League>> {
    let row = fetch_optional_by_param::<LeagueRow>(
        pool,
        "SELECT competition_id, division_index FROM leagues WHERE competition_id = ?",
        &competition_id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let competition = competition_repository::get_by_id(pool, competition_id)
        .await?
        .ok_or_else(|| {
            DbError::NotFound(format!(
                "Competition not found for league {}",
                competition_id
            ))
        })?;

    Ok(Some(row.to_domain(competition)?))
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<League>> {
    let rows =
        fetch_all::<LeagueRow>(pool, "SELECT competition_id, division_index FROM leagues").await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        let competition_id = Uuid::parse_str(&row.competition_id)?;
        let competition = competition_repository::get_by_id(pool, competition_id)
            .await?
            .ok_or_else(|| {
                DbError::NotFound(format!(
                    "Competition not found for league {}",
                    competition_id
                ))
            })?;
        results.push(row.to_domain(competition)?);
    }
    Ok(results)
}
