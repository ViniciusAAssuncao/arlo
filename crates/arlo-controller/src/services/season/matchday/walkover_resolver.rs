use crate::error::ControllerResult;
use crate::repositories::season::standings_cache;
use arlo_persistence::models::season::FixtureRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub fn apply_walkover(fixture: &FixtureRow, fault_team_id: Option<Uuid>) -> FixtureRow {
    let home_team_id = Uuid::parse_str(&fixture.home_team_id).ok();
    let away_team_id = Uuid::parse_str(&fixture.away_team_id).ok();

    let (home_score, away_score, home_gp, away_gp) = match fault_team_id {
        Some(fault_id) if Some(fault_id) == home_team_id && Some(fault_id) != away_team_id => {
            (0, 3, 0, 3)
        }
        Some(fault_id) if Some(fault_id) == away_team_id && Some(fault_id) != home_team_id => {
            (3, 0, 3, 0)
        }
        _ => (0, 0, 0, 0),
    };

    FixtureRow {
        id: fixture.id.clone(),
        season_stage_id: fixture.season_stage_id.clone(),
        round_index: fixture.round_index,
        home_team_id: fixture.home_team_id.clone(),
        away_team_id: fixture.away_team_id.clone(),
        is_neutral_venue: fixture.is_neutral_venue,
        venue_id: fixture.venue_id.clone(),
        scheduled_year: fixture.scheduled_year,
        scheduled_day_of_year: fixture.scheduled_day_of_year,
        status: "Walkover".to_string(),
        home_score: Some(home_score),
        away_score: Some(away_score),
        home_goal_points: Some(home_gp),
        away_goal_points: Some(away_gp),
    }
}

pub async fn handle_walkover(
    pool: &SqlitePool,
    fixture: &FixtureRow,
) -> ControllerResult<FixtureRow> {
    let home_id = Uuid::parse_str(&fixture.home_team_id).ok();
    let away_id = Uuid::parse_str(&fixture.away_team_id).ok();

    let home_ready = match home_id {
        Some(id) => is_team_ready(pool, id).await,
        None => false,
    };
    let away_ready = match away_id {
        Some(id) => is_team_ready(pool, id).await,
        None => false,
    };

    let fault_team_id = match (home_ready, away_ready) {
        (false, true) => home_id,
        (true, false) => away_id,
        _ => None,
    };

    let completed = apply_walkover(fixture, fault_team_id);
    persist_walkover_fixture(pool, &completed).await?;
    Ok(completed)
}

pub async fn persist_walkover_fixture(
    pool: &SqlitePool,
    fixture_row: &FixtureRow,
) -> ControllerResult<()> {
    sqlx::query(
        "UPDATE fixtures SET status = ?, home_score = ?, away_score = ?, home_goal_points = ?, away_goal_points = ? WHERE id = ?",
    )
    .bind(&fixture_row.status)
    .bind(fixture_row.home_score)
    .bind(fixture_row.away_score)
    .bind(fixture_row.home_goal_points)
    .bind(fixture_row.away_goal_points)
    .bind(&fixture_row.id)
    .execute(pool)
    .await?;

    if let Ok(stage_id) = Uuid::parse_str(&fixture_row.season_stage_id) {
        standings_cache::invalidate(&stage_id).await;
    }

    Ok(())
}

async fn is_team_ready(pool: &SqlitePool, team_id: Uuid) -> bool {
    let players = match arlo_db::repositories::player::list_by_team_id(pool, team_id).await {
        Ok(p) => p,
        Err(_) => return false,
    };
    let managers = match arlo_db::repositories::manager::list_by_team_id(pool, team_id).await {
        Ok(m) => m,
        Err(_) => return false,
    };
    if managers.is_empty() || players.is_empty() {
        return false;
    }
    let formations = match arlo_db::repositories::formation::list_all(pool).await {
        Ok(f) => f,
        Err(_) => return false,
    };
    if formations.is_empty() {
        return false;
    }
    players.len() >= formations[0].slots().len()
}