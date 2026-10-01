use crate::controllers::season::overview_calendar::resolve_overview_calendar;
use crate::controllers::season::overview_fixtures::build_overview_fixtures;
use crate::controllers::season::overview_spots::compute_stage_spots;
use crate::controllers::season::overview_standings::build_overview_standings;
use crate::controllers::season::standings::get_standings;
use crate::dto::season::{KnockoutTieDto, LeagueOverviewDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

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

    let target_season = match season_instances.into_iter().next() {
        Some(s) => s,
        None => {
            return Ok(LeagueOverviewDto {
                has_active_season: false,
                current_stage_type: "RoundRobinTable".to_string(),
                standings: Vec::new(),
                fixtures: Vec::new(),
                knockout_ties: Vec::new(),
                promotion_spots: 0,
                relegation_spots: 0,
                qualification_spots: 0,
                is_final_stage: false,
            });
        }
    };

    let season_instance_id = Uuid::parse_str(&target_season.id)?;
    let stages = arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
        pool,
        season_instance_id,
    )
    .await?;

    let current_stage = match stages
        .iter()
        .find(|s| s.stage_order_index == target_season.current_stage_order_index)
    {
        Some(st) => st,
        None => {
            return Ok(LeagueOverviewDto {
                has_active_season: false,
                current_stage_type: "RoundRobinTable".to_string(),
                standings: Vec::new(),
                fixtures: Vec::new(),
                knockout_ties: Vec::new(),
                promotion_spots: 0,
                relegation_spots: 0,
                qualification_spots: 0,
                is_final_stage: false,
            });
        }
    };

    let stage_id = Uuid::parse_str(&current_stage.id)?;
    let standings_entries = get_standings(pool, stage_id).await.unwrap_or_default();
    let fixture_rows =
        arlo_persistence::repositories::season::fixtures::list_by_stage_id(pool, stage_id).await?;
        
    let tie_rows =
        arlo_persistence::repositories::season::knockout_ties::list_by_season_stage_id(pool, stage_id).await?;

    let knockout_ties: Vec<KnockoutTieDto> = tie_rows
        .into_iter()
        .map(|r| KnockoutTieDto {
            id: r.id,
            round_index: r.round_index as u32,
            tie_index: r.tie_index as u32,
            high_seed_team_id: r.high_seed_team_id,
            high_seed_number: r.high_seed_number as u32,
            low_seed_team_id: r.low_seed_team_id,
            low_seed_number: r.low_seed_number as u32,
            leg_one_fixture_id: r.leg_one_fixture_id,
            leg_two_fixture_id: r.leg_two_fixture_id,
            aggregate_winner_team_id: r.aggregate_winner_team_id,
        })
        .collect();

    let league_config = get_or_load_league_calendar_config(pool, competition_id).await?;
    let spots = compute_stage_spots(
        league_config.as_deref(),
        current_stage.stage_order_index as u32,
    );

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = resolve_overview_calendar(pool, &catalog).await?;

    let teams = arlo_db::repositories::team::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let team_name_map: HashMap<Uuid, String> = teams
        .iter()
        .map(|t| (t.id(), t.name().to_string()))
        .collect();

    let team_home_venue_map: HashMap<Uuid, Option<Uuid>> =
        teams.iter().map(|t| (t.id(), t.home_venue_id())).collect();

    let all_venues = arlo_db::repositories::venue::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let venue_name_map: HashMap<Uuid, String> = all_venues
        .into_iter()
        .map(|v| (v.id(), v.name().to_string()))
        .collect();

    let standings = build_overview_standings(&standings_entries, &team_name_map);
    let fixtures = build_overview_fixtures(
        pool,
        fixture_rows,
        calendar,
        &team_name_map,
        &team_home_venue_map,
        &venue_name_map,
    )
    .await?;

    Ok(LeagueOverviewDto {
        has_active_season: true,
        current_stage_type: current_stage.stage_type.clone(),
        standings,
        fixtures,
        knockout_ties,
        promotion_spots: spots.promotion_spots,
        relegation_spots: spots.relegation_spots,
        qualification_spots: spots.qualification_spots,
        is_final_stage: spots.is_final_stage,
    })
}