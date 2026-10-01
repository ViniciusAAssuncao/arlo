use crate::domain::season::Fixture;
use crate::domain::season::StandingsEntry;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::season::persistence::map_row_to_fixture;
use crate::services::season::standings::random_tiebreak_resolver::seed_from_uuid;
use crate::services::season::standings::standings_pipeline;
use arlo_persistence::repositories::season::{fixtures, season_instances, season_stages};
use sqlx::SqlitePool;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

static STANDINGS_CACHE: LazyLock<RwLock<HashMap<Uuid, Arc<Vec<StandingsEntry>>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn get_or_compute_standings(
    pool: &SqlitePool,
    stage_id: Uuid,
) -> ControllerResult<Arc<Vec<StandingsEntry>>> {
    get_or_compute_standings_with_fixtures(pool, stage_id, None).await
}

pub async fn get_or_compute_standings_with_fixtures(
    pool: &SqlitePool,
    stage_id: Uuid,
    provided_fixtures: Option<&[Fixture]>,
) -> ControllerResult<Arc<Vec<StandingsEntry>>> {
    {
        let read_guard = STANDINGS_CACHE.read().await;
        if let Some(standings) = read_guard.get(&stage_id) {
            return Ok(standings.clone());
        }
    }

    let stage_row = season_stages::get_by_id(pool, stage_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Season stage {} not found", stage_id)))?;

    let season_instance_id = Uuid::parse_str(&stage_row.season_instance_id)?;
    let season_row = season_instances::get_by_id(pool, season_instance_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!("Season instance {} not found", season_instance_id))
        })?;

    let competition_id = Uuid::parse_str(&season_row.competition_id)?;
    let config = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let (domain_fixtures, team_ids) = if let Some(fixtures_slice) = provided_fixtures {
        let mut team_ids_set = BTreeSet::new();
        for f in fixtures_slice {
            team_ids_set.insert(f.home_team_id());
            team_ids_set.insert(f.away_team_id());
        }
        let team_ids: Vec<Uuid> = if team_ids_set.is_empty() {
            let teams = arlo_db::repositories::team::list_by_league_id(pool, competition_id)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            teams.into_iter().map(|t| t.id()).collect()
        } else {
            team_ids_set.into_iter().collect()
        };
        (fixtures_slice.to_vec(), team_ids)
    } else {
        let fixture_rows = fixtures::list_by_stage_id(pool, stage_id).await?;
        let mut domain_fixtures = Vec::with_capacity(fixture_rows.len());
        let mut team_ids_set = BTreeSet::new();

        for row in &fixture_rows {
            let fixture = map_row_to_fixture(row)?;
            team_ids_set.insert(fixture.home_team_id());
            team_ids_set.insert(fixture.away_team_id());
            domain_fixtures.push(fixture);
        }

        let team_ids: Vec<Uuid> = if team_ids_set.is_empty() {
            let teams = arlo_db::repositories::team::list_by_league_id(pool, competition_id)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            teams.into_iter().map(|t| t.id()).collect()
        } else {
            team_ids_set.into_iter().collect()
        };
        (domain_fixtures, team_ids)
    };

    let seed = seed_from_uuid(stage_id);
    let calculated = standings_pipeline::calculate_and_rank_standings(
        &team_ids,
        &domain_fixtures,
        config.spa_scoring_policy(),
        config.qta_weighting_policy(),
        config.tie_break_criteria(),
        seed,
    );

    let arc_standings = Arc::new(calculated);

    let mut write_guard = STANDINGS_CACHE.write().await;
    write_guard.insert(stage_id, arc_standings.clone());

    Ok(arc_standings)
}

pub async fn invalidate(stage_id: &Uuid) {
    let mut write_guard = STANDINGS_CACHE.write().await;
    write_guard.remove(stage_id);
}

pub async fn invalidate_all() {
    let mut write_guard = STANDINGS_CACHE.write().await;
    write_guard.clear();
}
