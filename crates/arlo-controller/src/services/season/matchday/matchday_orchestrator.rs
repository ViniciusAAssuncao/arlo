use crate::error::{ControllerError, ControllerResult};
use crate::services::season::matchday::due_fixture_finder::find_due_fixtures;
use crate::services::season::matchday::matchday_catalog_cache::get_or_load_matchday_catalogs;
use crate::services::season::matchday::matchday_runner::{
    persist_completed_simulation, simulate_match, CompletedMatchSimulation,
};
use crate::services::season::matchday::matchday_setup_builder::{
    build_matchday_setup, PreparedMatchdayFixture,
};
use arlo_engine::MatchState;
use rayon::prelude::*;
use sqlx::SqlitePool;

pub async fn run_due_matches(
    pool: &SqlitePool,
    year: i64,
    day_of_year: u32,
) -> ControllerResult<u32> {
    let due_fixtures = find_due_fixtures(pool, year, day_of_year).await?;
    if due_fixtures.is_empty() {
        return Ok(0);
    }

    let catalogs = get_or_load_matchday_catalogs(pool).await?;

    let mut prepared_matches: Vec<PreparedMatchdayFixture> = Vec::with_capacity(due_fixtures.len());
    for fixture in &due_fixtures {
        match build_matchday_setup(pool, fixture, &catalogs).await {
            Ok(prep) => prepared_matches.push(prep),
            Err(err) => {
                eprintln!(
                    "Skipping fixture {} due to setup error: {}",
                    fixture.id, err
                );
            }
        }
    }

    if prepared_matches.is_empty() {
        return Ok(0);
    }

    let simulation_results: Vec<Result<CompletedMatchSimulation, ControllerError>> = prepared_matches
        .into_par_iter()
        .map(|prep| {
            let state = MatchState::new(prep.setup_params)
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            simulate_match(
                state,
                prep.persistence_context,
                prep.fixture_row,
                prep.seed,
                prep.stage_id,
            )
        })
        .collect();

    let mut matches_played_count = 0u32;

    for sim_result in simulation_results {
        match sim_result {
            Ok(simulation) => match persist_completed_simulation(pool, simulation).await {
                Ok(_) => {
                    matches_played_count += 1;
                }
                Err(err) => {
                    eprintln!("Failed to persist match simulation result: {}", err);
                }
            },
            Err(err) => {
                eprintln!("Match simulation failed: {}", err);
            }
        }
    }

    Ok(matches_played_count)
}