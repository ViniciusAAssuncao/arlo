use crate::domain::season::{
    Fixture, FixtureResult, FixtureStatus, GroupRankedStandingsEntry, SeasonInstance,
    StandingsEntry,
};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::repositories::season::standings_cache;
use crate::services::season::season_generator::{self, GeneratedSeason};
use crate::services::season::standings::group_rank_annotator;
use arlo_domain::CompetitionGroup;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn trigger_season_generation(
    pool: &SqlitePool,
    competition_id: Uuid,
    calendar_system_id: Uuid,
    reference_year: i64,
) -> ControllerResult<GeneratedSeason> {
    season_generator::generate_season_for_league(
        pool,
        competition_id,
        calendar_system_id,
        reference_year,
    )
    .await
}

pub fn get_season_instance(
    season_instances: &[SeasonInstance],
    id: Uuid,
) -> Option<SeasonInstance> {
    season_instances.iter().find(|s| s.id() == id).cloned()
}

pub fn list_fixtures_for_team(
    fixtures: &[Fixture],
    team_id: Uuid,
) -> Vec<Fixture> {
    fixtures
        .iter()
        .filter(|f| f.home_team_id() == team_id || f.away_team_id() == team_id)
        .copied()
        .collect()
}

pub async fn get_standings(
    pool: &SqlitePool,
    stage_id: Uuid,
) -> ControllerResult<Arc<Vec<StandingsEntry>>> {
    standings_cache::get_or_compute_standings(pool, stage_id).await
}

pub async fn get_group_ranked_standings(
    pool: &SqlitePool,
    stage_id: Uuid,
    groups: &[CompetitionGroup],
) -> ControllerResult<Vec<GroupRankedStandingsEntry>> {
    let standings = get_standings(pool, stage_id).await?;
    if groups.is_empty() {
        let stage_row = arlo_persistence::repositories::season::season_stages::get_by_id(pool, stage_id)
            .await?
            .ok_or_else(|| {
                ControllerError::NotFound(format!("Stage {} not found", stage_id))
            })?;
        let season_id = Uuid::parse_str(&stage_row.season_instance_id)?;
        let season_row = arlo_persistence::repositories::season::season_instances::get_by_id(pool, season_id)
            .await?
            .ok_or_else(|| {
                ControllerError::NotFound(format!("Season instance {} not found", season_id))
            })?;
        let comp_id = Uuid::parse_str(&season_row.competition_id)?;
        let config = get_or_load_league_calendar_config(pool, comp_id)
            .await?
            .ok_or_else(|| {
                ControllerError::NotFound(format!(
                    "League calendar config for competition {} not found",
                    comp_id
                ))
            })?;
        Ok(group_rank_annotator::annotate_group_ranks(
            &standings,
            config.groups(),
        ))
    } else {
        Ok(group_rank_annotator::annotate_group_ranks(
            &standings,
            groups,
        ))
    }
}

pub async fn record_fixture_result(
    fixture: &Fixture,
    result: FixtureResult,
) -> Fixture {
    standings_cache::invalidate(&fixture.season_stage_id()).await;
    Fixture::new(
        fixture.id(),
        fixture.season_stage_id(),
        fixture.round_index(),
        fixture.home_team_id(),
        fixture.away_team_id(),
        fixture.is_neutral_venue(),
        fixture.scheduled_date(),
        FixtureStatus::Completed,
        Some(result),
    )
}