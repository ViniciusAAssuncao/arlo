use crate::domain::season::{
    Fixture, FixtureResult, FixtureStatus, GroupRankedStandingsEntry, SeasonInstance,
    StandingsEntry,
};
use crate::dto::season::{FixtureSummaryDto, LeagueOverviewDto, StandingsEntryDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::repositories::season::standings_cache;
use crate::services::season::season_generator::{self, GeneratedSeason};
use crate::services::season::standings::group_rank_annotator;
use arlo_domain::CompetitionGroup;
use sqlx::SqlitePool;
use std::collections::HashMap;
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
        fixture.venue_id(),
        fixture.scheduled_date(),
        FixtureStatus::Completed,
        Some(result),
    )
}

pub async fn get_league_overview(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> ControllerResult<LeagueOverviewDto> {
    let season_instances =
        arlo_persistence::repositories::season::season_instances::list_by_competition_id(
            pool,
            competition_id,
        )
        .await?;

    let active_season = match season_instances
        .iter()
        .find(|s| s.status == "Active" || s.status == "Pending")
        .or_else(|| season_instances.first())
    {
        Some(s) => s,
        None => {
            return Ok(LeagueOverviewDto {
                has_active_season: false,
                standings: Vec::new(),
                fixtures: Vec::new(),
            });
        }
    };

    let season_instance_id = Uuid::parse_str(&active_season.id)?;
    let stages =
        arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
            pool,
            season_instance_id,
        )
        .await?;

    let current_stage = match stages
        .iter()
        .find(|s| s.stage_order_index == active_season.current_stage_order_index)
    {
        Some(st) => st,
        None => {
            return Ok(LeagueOverviewDto {
                has_active_season: false,
                standings: Vec::new(),
                fixtures: Vec::new(),
            });
        }
    };

    let stage_id = Uuid::parse_str(&current_stage.id)?;
    let standings_entries = get_standings(pool, stage_id).await?;
    let fixture_rows =
        arlo_persistence::repositories::season::fixtures::list_by_stage_id(pool, stage_id)
            .await?;

    let teams = arlo_db::repositories::team::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let team_name_map: HashMap<Uuid, String> = teams
        .into_iter()
        .map(|t| (t.id(), t.name().to_string()))
        .collect();

    let standings = standings_entries
        .iter()
        .map(|entry| {
            let team_name = team_name_map
                .get(&entry.team_id())
                .cloned()
                .unwrap_or_else(|| "Time Desconhecido".to_string());

            StandingsEntryDto {
                team_id: entry.team_id().to_string(),
                team_name,
                played: entry.played(),
                won: entry.won(),
                drawn: entry.drawn(),
                lost: entry.lost(),
                goal_points_for: entry.goal_points_for(),
                goal_points_against: entry.goal_points_against(),
                ispa: entry.spa_metrics().ispa(),
                qta: entry.qta(),
            }
        })
        .collect();

    let mut fixtures = Vec::with_capacity(fixture_rows.len());
    for row in fixture_rows {
        let home_id = Uuid::parse_str(&row.home_team_id).unwrap_or_default();
        let away_id = Uuid::parse_str(&row.away_team_id).unwrap_or_default();
        let home_name = team_name_map
            .get(&home_id)
            .cloned()
            .unwrap_or_else(|| "Mandante".to_string());
        let away_name = team_name_map
            .get(&away_id)
            .cloned()
            .unwrap_or_else(|| "Visitante".to_string());

        fixtures.push(FixtureSummaryDto {
            id: row.id,
            round_index: row.round_index as u32,
            home_team_name: home_name,
            away_team_name: away_name,
            status: row.status,
            home_score: row.home_score,
            away_score: row.away_score,
            scheduled_year: row.scheduled_year,
            scheduled_day_of_year: row.scheduled_day_of_year as u32,
        });
    }

    fixtures.sort_by_key(|f| (f.round_index, f.scheduled_day_of_year));

    Ok(LeagueOverviewDto {
        has_active_season: true,
        standings,
        fixtures,
    })
}