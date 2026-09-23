use crate::error::{ControllerError, ControllerResult};
use crate::repositories::season::standings_cache;
use arlo_engine::MatchState;
use arlo_match_runner::MatchRunResult;
use arlo_persistence::models::season::FixtureRow;
use arlo_persistence::persister::{MatchPersistenceContext, MatchPersister};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn complete_and_persist_match(
    pool: &SqlitePool,
    fixture_id: Uuid,
    state: &MatchState,
    run_result: &MatchRunResult,
    context: MatchPersistenceContext,
) -> ControllerResult<Uuid> {
    let fixture_row = arlo_persistence::repositories::season::fixtures::get_by_id(pool, fixture_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Fixture {} not found", fixture_id)))?;

    let stage_id = Uuid::parse_str(&fixture_row.season_stage_id)?;

    let updated_fixture_row = FixtureRow {
        id: fixture_row.id,
        season_stage_id: fixture_row.season_stage_id,
        round_index: fixture_row.round_index,
        home_team_id: fixture_row.home_team_id,
        away_team_id: fixture_row.away_team_id,
        is_neutral_venue: fixture_row.is_neutral_venue,
        venue_id: context
            .venue_id
            .map(|v| v.to_string())
            .or(fixture_row.venue_id),
        scheduled_year: fixture_row.scheduled_year,
        scheduled_day_of_year: fixture_row.scheduled_day_of_year,
        status: "Completed".to_string(),
        home_score: Some(state.home_score().total_points as i32),
        away_score: Some(state.away_score().total_points as i32),
        home_goal_points: Some(state.home_score().goal_points as i32),
        away_goal_points: Some(state.away_score().goal_points as i32),
        home_field_goals: Some(state.home_score().field_goals as i32),
        away_field_goals: Some(state.away_score().field_goals as i32),
        home_field_points: Some(state.home_score().field_points as i32),
        away_field_points: Some(state.away_score().field_points as i32),
    };

    let context = context.with_completed_fixture(updated_fixture_row);

    let mut tx = pool.begin().await?;
    let match_id =
        MatchPersister::persist_completed_match(&mut tx, state, run_result, &context).await?;
    tx.commit().await?;

    standings_cache::invalidate(&stage_id).await;

    Ok(match_id)
}
