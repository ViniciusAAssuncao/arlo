use crate::domain::season::{
    Fixture, FixtureResult, FixtureStatus, GroupRankedStandingsEntry, SeasonInstance,
    StandingsEntry,
};
use crate::dto::season::{FixtureSummaryDto, LeagueOverviewDto, StandingsEntryDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::repositories::season::standings_cache;
use crate::services::calendar::date_resolver;
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
    {
        Some(s) => s.clone(),
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

    let metadata = arlo_db::repositories::save_metadata::get(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound("Metadados da gravação não encontrados".to_string()))?;

    let save_calendar_row = arlo_persistence::repositories::calendar::save_calendar_state::get_by_save_uuid(
        pool,
        metadata.save_uuid(),
    )
    .await?;

    let calendar_system_id = if let Some(row) = save_calendar_row {
        Uuid::parse_str(&row.calendar_system_id)?
    } else {
        let systems = arlo_db::repositories::calendar_system::list_all(pool)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        let first = systems.into_iter().next().ok_or_else(|| {
            ControllerError::NotFound("Nenhum sistema de calendário cadastrado".to_string())
        })?;
        Uuid::parse_str(&first.id)?
    };

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })?;

    let teams = arlo_db::repositories::team::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let team_name_map: HashMap<Uuid, String> = teams
        .iter()
        .map(|t| (t.id(), t.name().to_string()))
        .collect();

    let team_home_venue_map: HashMap<Uuid, Option<Uuid>> = teams
        .iter()
        .map(|t| (t.id(), t.home_venue_id()))
        .collect();

    let all_venues = arlo_db::repositories::venue::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let venue_name_map: HashMap<Uuid, String> = all_venues
        .into_iter()
        .map(|v| (v.id(), v.name().to_string()))
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
                field_goals_for: entry.field_goals_for(),
                field_goals_against: entry.field_goals_against(),
                field_points_for: entry.field_points_for(),
                field_points_against: entry.field_points_against(),
                total_points_for: entry.total_points_for(),
                total_points_against: entry.total_points_against(),
                home_won: entry.home_away().home_won(),
                home_drawn: entry.home_away().home_drawn(),
                home_lost: entry.home_away().home_lost(),
                away_won: entry.home_away().away_won(),
                away_drawn: entry.home_away().away_drawn(),
                away_lost: entry.home_away().away_lost(),
                pb: entry.spa_metrics().pb(),
                feo: entry.spa_metrics().feo(),
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

        let fixture_cal_date = crate::domain::calendar::CalendarDate::new(
            row.scheduled_year,
            row.scheduled_day_of_year as u32,
        );

        let resolved_date = date_resolver::resolve(calendar, &fixture_cal_date);

        let (scheduled_month_name, scheduled_day_of_month, scheduled_week_day_name) = match resolved_date {
            crate::domain::calendar::ResolvedCalendarDate::RegularDay {
                month_order_index,
                day_of_month,
                week_day_index,
                ..
            } => {
                let m_name = calendar
                    .months()
                    .iter()
                    .find(|m| m.order_index() == month_order_index)
                    .map(|m| m.name().to_string())
                    .unwrap_or_default();
                let w_name = calendar
                    .week_days()
                    .iter()
                    .find(|w| w.order_index() == week_day_index)
                    .map(|w| w.name().to_string())
                    .unwrap_or_default();
                (m_name, day_of_month, w_name)
            }
            crate::domain::calendar::ResolvedCalendarDate::IntercalaryDay {
                intercalary_index,
                week_day_index,
                ..
            } => {
                let w_name = week_day_index
                    .and_then(|idx| {
                        calendar
                            .week_days()
                            .iter()
                            .find(|w| w.order_index() == idx)
                            .map(|w| w.name().to_string())
                    })
                    .unwrap_or_default();
                ("Sirdápis".to_string(), intercalary_index + 1, w_name)
            }
        };

        let target_venue_id = if row.is_neutral_venue {
            row.venue_id
                .as_deref()
                .and_then(|vid| Uuid::parse_str(vid).ok())
        } else {
            team_home_venue_map
                .get(&home_id)
                .copied()
                .flatten()
                .or_else(|| {
                    row.venue_id
                        .as_deref()
                        .and_then(|vid| Uuid::parse_str(vid).ok())
                })
        };

        let venue_name = target_venue_id.and_then(|vid| venue_name_map.get(&vid).cloned());

        fixtures.push(FixtureSummaryDto {
            id: row.id,
            round_index: row.round_index as u32,
            home_team_id: row.home_team_id.clone(),
            away_team_id: row.away_team_id.clone(),
            home_team_name: home_name,
            away_team_name: away_name,
            status: row.status,
            home_score: row.home_score,
            away_score: row.away_score,
            home_goal_points: row.home_goal_points,
            away_goal_points: row.away_goal_points,
            home_field_goals: row.home_field_goals,
            away_field_goals: row.away_field_goals,
            home_field_points: row.home_field_points,
            away_field_points: row.away_field_points,
            scheduled_year: row.scheduled_year,
            scheduled_day_of_year: row.scheduled_day_of_year as u32,
            scheduled_month_name,
            scheduled_day_of_month,
            scheduled_week_day_name,
            venue_name,
        });
    }

    fixtures.sort_by_key(|f| (f.round_index, f.scheduled_day_of_year));

    Ok(LeagueOverviewDto {
        has_active_season: true,
        standings,
        fixtures,
    })
}