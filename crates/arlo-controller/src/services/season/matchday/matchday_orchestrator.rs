use crate::error::{ ControllerError, ControllerResult };
use crate::services::season::matchday::due_fixture_finder::find_due_fixtures;
use crate::services::season::matchday::matchday_catalog_cache::get_or_load_matchday_catalogs;
use crate::services::season::matchday::matchday_runner::{
    persist_completed_simulation,
    simulate_match,
    CompletedMatchSimulation,
};
use crate::services::season::matchday::matchday_setup_builder::{ build_matchday_setup };
use crate::services::season::matchday::walkover_resolver;
use arlo_engine::MatchState;
use rayon::prelude::*;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::task::JoinSet;

pub async fn run_due_matches(
    pool: &SqlitePool,
    year: i64,
    day_of_year: u32
) -> ControllerResult<u32> {
    let due_fixtures = find_due_fixtures(pool, year, day_of_year).await?;
    if due_fixtures.is_empty() {
        return Ok(0);
    }

    let catalogs = get_or_load_matchday_catalogs(pool).await?;

    let mut join_set = JoinSet::new();

    for fixture in due_fixtures {
        let pool = pool.clone();
        let catalogs = Arc::clone(&catalogs);
        join_set.spawn(async move {
            let res = build_matchday_setup(&pool, &fixture, &catalogs).await;
            (fixture, res)
        });
    }

    let mut prepared_matches = Vec::new();
    let mut matches_played_count = 0u32;

    while let Some(join_res) = join_set.join_next().await {
        match join_res {
            Ok((_, Ok(prep))) => {
                prepared_matches.push(prep);
            }
            Ok((fixture, Err(err))) => {
                eprintln!("build_matchday_setup failed for fixture {}: {}", fixture.id, err);
                if walkover_resolver::handle_walkover(pool, &fixture).await.is_ok() {
                    matches_played_count += 1;
                }
            }
            Err(join_err) => {
                return Err(
                    ControllerError::InvalidData(
                        format!("Task join error during match setup: {}", join_err)
                    )
                );
            }
        }
    }

    if prepared_matches.is_empty() {
        return Ok(matches_played_count);
    }

    let simulation_results: Vec<
        Result<CompletedMatchSimulation, ControllerError>
    > = prepared_matches
        .into_par_iter()
        .map(|prep| {
            let mut state = MatchState::new(prep.setup_params).map_err(|e|
                ControllerError::InvalidData(e.to_string())
            )?;
            arlo_recovery::orchestration::match_condition_bridge::seed_match_state(
                &mut state,
                &prep.initial_conditions
            );
            simulate_match(
                state,
                prep.persistence_context,
                prep.fixture_row,
                prep.seed,
                prep.stage_id
            )
        })
        .collect();

    for sim_result in simulation_results {
        match sim_result {
            Ok(simulation) =>
                match persist_completed_simulation(pool, simulation).await {
                    Ok(_) => {
                        matches_played_count += 1;
                    }
                    Err(err) => {
                        eprintln!("Failed to persist match simulation result: {}", err);
                    }
                }
            Err(err) => {
                eprintln!("Match simulation failed: {}", err);
            }
        }
    }

    Ok(matches_played_count)
}
