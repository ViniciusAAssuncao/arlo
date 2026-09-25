use crate::error::{ControllerError, ControllerResult};
use crate::services::season::matchday::due_fixture_finder::find_due_fixtures;
use crate::services::season::matchday::matchday_catalog_cache::get_or_load_matchday_catalogs;
use crate::services::season::matchday::matchday_runner::{
    persist_completed_simulation, simulate_match, CompletedMatchSimulation,
};
use crate::services::season::matchday::matchday_setup_builder::build_matchday_setup;
use crate::services::season::matchday::walkover_resolver;
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

    let mut prepared_matches = Vec::with_capacity(due_fixtures.len());
    let mut matches_played_count = 0u32;

    for fixture in due_fixtures {
        match build_matchday_setup(pool, &fixture, &catalogs).await {
            Ok(prep) => {
                prepared_matches.push(prep);
            }
            Err(err) => {
                eprintln!(
                    "build_matchday_setup failed for fixture {}: {}",
                    fixture.id, err
                );
                if walkover_resolver::handle_walkover(pool, &fixture)
                    .await
                    .is_ok()
                {
                    matches_played_count += 1;
                }
            }
        }
    }

    if prepared_matches.is_empty() {
        return Ok(matches_played_count);
    }

    let simulation_results: Vec<Result<CompletedMatchSimulation, ControllerError>> =
        prepared_matches
            .into_par_iter()
            .map(|prep| {
                simulate_match(
                    prep.input,
                    prep.persistence_context,
                    prep.fixture_row,
                    prep.stage_id,
                    prep.initial_conditions,
                )
            })
            .collect();

    let mut tx = pool.begin().await?;
    let mut persisted_simulations = Vec::new();

    for sim_result in simulation_results {
        match sim_result {
            Ok(simulation) => match persist_completed_simulation(&mut tx, &simulation).await {
                Ok(_) => {
                    matches_played_count += 1;
                    persisted_simulations.push(simulation);
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

    tx.commit().await?;

    for simulation in &persisted_simulations {
        let (match_year, match_day) = match &simulation.persistence_context.completed_fixture {
            Some(f) => (f.scheduled_year, f.scheduled_day_of_year as u32),
            None => (0, 0),
        };

        if let Err(error) = arlo_recovery::orchestration::capture_post_match_condition(
            pool,
            &simulation.input,
            &simulation.initial_conditions,
            simulation.run_result.raw_sink.events(),
            match_year,
            match_day,
        )
        .await {
            eprintln!("Failed to capture post-match condition for match {}: {}", simulation.input.match_id(), error);
        }

        crate::repositories::season::standings_cache::invalidate(&simulation.stage_id).await;
    }

    Ok(matches_played_count)
}
