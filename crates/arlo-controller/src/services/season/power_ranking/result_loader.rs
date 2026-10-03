use crate::domain::calendar::CalendarDate;
use crate::error::{ControllerError, ControllerResult};
use arlo_analytics::PowerMatchResult;
use arlo_persistence::models::season::PowerRankingFixtureRow;
use arlo_persistence::repositories::season::power_rankings;
use sqlx::SqlitePool;
use uuid::Uuid;

pub(super) async fn load_results(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    through: CalendarDate,
) -> ControllerResult<Vec<PowerMatchResult>> {
    let rows = power_rankings::list_completed_fixtures(
        pool,
        season_instance_id,
        through.year(),
        through.day_of_year(),
    )
    .await?;
    rows.into_iter().map(map_result).collect()
}

fn map_result(row: PowerRankingFixtureRow) -> ControllerResult<PowerMatchResult> {
    let fixture_id = Uuid::parse_str(&row.fixture_id)?;
    let played_day_of_year = u32::try_from(row.played_day_of_year).map_err(|_| {
        ControllerError::InvalidData(format!("fixture {fixture_id} has an invalid played day"))
    })?;
    let home_score = score(row.home_score, fixture_id, "home")?;
    let away_score = score(row.away_score, fixture_id, "away")?;
    Ok(PowerMatchResult {
        fixture_id,
        played_year: row.played_year,
        played_day_of_year,
        home_team_id: Uuid::parse_str(&row.home_team_id)?,
        away_team_id: Uuid::parse_str(&row.away_team_id)?,
        home_score,
        away_score,
        neutral_venue: row.neutral_venue,
    })
}

fn score(value: Option<i64>, fixture_id: Uuid, side: &str) -> ControllerResult<u32> {
    value
        .and_then(|score| u32::try_from(score).ok())
        .ok_or_else(|| {
            ControllerError::InvalidData(format!(
                "fixture {fixture_id} has an invalid {side} score"
            ))
        })
}
