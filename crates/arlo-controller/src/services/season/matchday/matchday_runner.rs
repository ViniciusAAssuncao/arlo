use crate::error::{ControllerError, ControllerResult};
use arlo_engine::{MatchInput, MatchState};
use arlo_match_runner::{
    build_default_aggregator_registry, run_match_with_registry_and_play_calls, MatchRunResult,
};
use arlo_persistence::models::season::FixtureRow;
use arlo_persistence::persister::{MatchPersistenceContext, MatchPersister};
use arlo_tactics::PlayCall;
use arlo_recovery::PlayerCondition;
use sqlx::{Sqlite, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub struct CompletedMatchSimulation {
    pub input: MatchInput,
    pub state: MatchState,
    pub run_result: MatchRunResult,
    pub persistence_context: MatchPersistenceContext,
    pub stage_id: Uuid,
    pub initial_conditions: HashMap<Uuid, PlayerCondition>,
}

pub fn simulate_match(
    input: MatchInput,
    context: MatchPersistenceContext,
    fixture_row: FixtureRow,
    stage_id: Uuid,
    initial_conditions: HashMap<Uuid, PlayerCondition>,
    play_calls: Vec<PlayCall>,
) -> ControllerResult<CompletedMatchSimulation> {
    let mut state = MatchState::new(&input);
    let registry = build_default_aggregator_registry(&input)
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let run_result = run_match_with_registry_and_play_calls(
        &input,
        &mut state,
        registry,
        &play_calls,
        arlo_match_runner::DEFAULT_MAX_SEGMENTS,
    )
    .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

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
        home_score: Some(state.home().score().total_points() as i32),
        away_score: Some(state.away().score().total_points() as i32),
        home_goal_points: Some(state.home().score().goal_points() as i32),
        away_goal_points: Some(state.away().score().goal_points() as i32),
        home_field_goals: Some(state.home().score().field_goals() as i32),
        away_field_goals: Some(state.away().score().field_goals() as i32),
        home_field_points: Some(state.home().score().field_points() as i32),
        away_field_points: Some(state.away().score().field_points() as i32),
    };

    let context = context.with_completed_fixture(updated_fixture_row);

    Ok(CompletedMatchSimulation {
        input,
        state,
        run_result,
        persistence_context: context,
        stage_id,
        initial_conditions,
    })
}

pub async fn persist_completed_simulation(
    tx: &mut Transaction<'_, Sqlite>,
    simulation: &CompletedMatchSimulation,
) -> ControllerResult<Uuid> {
    let match_id = MatchPersister::persist_completed_match(
        tx,
        &simulation.input,
        &simulation.state,
        &simulation.run_result,
        &simulation.persistence_context,
    )
    .await
    .map_err(ControllerError::Persistence)?;

    Ok(match_id)
}
