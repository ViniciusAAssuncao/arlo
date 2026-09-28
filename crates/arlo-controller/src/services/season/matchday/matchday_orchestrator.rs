use crate::error::{ControllerError, ControllerResult};
use crate::services::day_simulation::day_progress;
use crate::services::season::matchday::due_fixture_finder::find_due_fixtures;
use crate::services::season::matchday::matchday_catalog_cache::get_or_load_matchday_catalogs;
use crate::services::season::matchday::matchday_runner::{
    persist_completed_simulation, simulate_match, CompletedMatchSimulation,
};
use crate::services::season::matchday::matchday_setup_builder::build_matchday_setup;
use crate::services::season::matchday::walkover_resolver;
use rayon::prelude::*;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn run_due_matches(
    pool: &SqlitePool,
    save_uuid: Uuid,
    year: i64,
    day_of_year: u32,
) -> ControllerResult<u32> {
    let due_fixtures = find_due_fixtures(pool, year, day_of_year).await?;
    if due_fixtures.is_empty() {
        let mut tx = pool.begin().await?;
        day_progress::record_matches_done(&mut tx, save_uuid, 0).await?;
        tx.commit().await?;
        return Ok(0);
    }

    let catalogs = get_or_load_matchday_catalogs(pool).await?;

    let mut prepared_matches = Vec::with_capacity(due_fixtures.len());
    let mut walkovers = Vec::new();

    for fixture in due_fixtures {
        match build_matchday_setup(pool, &fixture, &catalogs).await {
            Ok(prep) => {
                prepared_matches.push(prep);
            }
            Err(error) => {
                let walkover = walkover_resolver::prepare_walkover(pool, &fixture)
                    .await
                    .map_err(|_| error)?;
                walkovers.push(walkover);
            }
        }
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

    let simulations = simulation_results.into_iter()
        .collect::<ControllerResult<Vec<_>>>()?;
    let mut condition_plans = Vec::with_capacity(simulations.len());
    for simulation in &simulations {
        let (match_year, match_day) = match &simulation.persistence_context.completed_fixture {
            Some(fixture) => (fixture.scheduled_year, fixture.scheduled_day_of_year as u32),
            None => return Err(ControllerError::InvalidData("Completed fixture is missing".into())),
        };
        condition_plans.push(
            arlo_recovery::orchestration::prepare_post_match_condition(
                pool, &simulation.input, &simulation.initial_conditions,
                simulation.run_result.raw_sink.events(), match_year, match_day,
            ).await.map_err(|error| ControllerError::InvalidData(error.to_string()))?,
        );
    }
    let matches_played_count = (walkovers.len() + simulations.len()) as u32;
    let mut tx = pool.begin().await?;
    for walkover in &walkovers {
        walkover_resolver::persist_walkover_fixture_with_tx(&mut tx, walkover).await?;
    }
    for (simulation, plan) in simulations.iter().zip(condition_plans) {
        persist_completed_simulation(&mut tx, simulation).await?;
        arlo_recovery::orchestration::persist_post_match_condition(&mut tx, plan)
            .await.map_err(|error| ControllerError::InvalidData(error.to_string()))?;
    }
    day_progress::record_matches_done(&mut tx, save_uuid, matches_played_count).await?;
    tx.commit().await?;

    for walkover in &walkovers {
        if let Ok(stage_id) = Uuid::parse_str(&walkover.season_stage_id) {
            crate::repositories::season::standings_cache::invalidate(&stage_id).await;
        }
    }
    for simulation in &simulations {
        crate::repositories::season::standings_cache::invalidate(&simulation.stage_id).await;
    }

    Ok(matches_played_count)
}
