use crate::domain::calendar::CalendarSystem;
use crate::domain::season::{
    Fixture, FixtureStatus, SeasonInstance, SeasonInstanceStatus, SeasonStageInstance, StageStatus,
};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::season::grouped_schedule::generate_grouped_schedule;
use crate::services::season::persistence::persist_generated_season;
use crate::services::season::round_robin::{
    assign_dates, expand_double_round_robin, generate_single_round_robin, resolve_neutral_opener,
};
use arlo_domain::{LeagueCalendarConfig, ScheduleAlgorithmKind, StageType};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedSeason {
    pub season_instance: SeasonInstance,
    pub stage_instance: SeasonStageInstance,
    pub fixtures: Vec<Fixture>,
}

pub fn generate_season(
    calendar: &CalendarSystem,
    config: &LeagueCalendarConfig,
    team_ids: &[Uuid],
    reference_year: i64,
) -> ControllerResult<GeneratedSeason> {
    if team_ids.len() < 2 {
        return Err(ControllerError::Validation(
            "At least 2 teams are required to generate a season".to_string(),
        ));
    }

    let stage_0_def = config
        .stages()
        .iter()
        .find(|s| s.stage_order_index() == 0)
        .ok_or_else(|| {
            ControllerError::Validation(
                "League calendar config must contain a stage with stage_order_index 0".to_string(),
            )
        })?;

    match stage_0_def.stage_type() {
        StageType::GroupedCompetitionTable => {
            let season_instance_id = Uuid::new_v4();
            let stage_instance_id = Uuid::new_v4();

            let stage_schedule = generate_grouped_schedule(
                calendar,
                config,
                stage_0_def,
                season_instance_id,
                stage_instance_id,
                reference_year,
                0,
            )?;

            let season_instance = SeasonInstance::new(
                season_instance_id,
                config.league_id(),
                reference_year,
                0,
                SeasonInstanceStatus::Pending,
            );

            Ok(GeneratedSeason {
                season_instance,
                stage_instance: stage_schedule.stage_instance,
                fixtures: stage_schedule.fixtures,
            })
        }
        _ => {
            let single_leg_matches = generate_single_round_robin(team_ids);

            let mut matches = match config.algorithm() {
                ScheduleAlgorithmKind::RoundRobinSingleLeg => single_leg_matches,
                ScheduleAlgorithmKind::RoundRobinDoubleLeg => {
                    expand_double_round_robin(&single_leg_matches)
                }
            };

            resolve_neutral_opener(&mut matches, &config.neutral_opener());

            let scheduled_matches =
                assign_dates(calendar, config.timing(), reference_year, &matches)?;

            let season_instance_id = Uuid::new_v4();
            let season_instance = SeasonInstance::new(
                season_instance_id,
                config.league_id(),
                reference_year,
                0,
                SeasonInstanceStatus::Pending,
            );

            let stage_instance_id = Uuid::new_v4();
            let stage_instance = SeasonStageInstance::new(
                stage_instance_id,
                season_instance_id,
                0,
                stage_0_def.stage_type(),
                StageStatus::Pending,
            );

            let mut fixtures = Vec::with_capacity(scheduled_matches.len());
            for sm in scheduled_matches {
                fixtures.push(Fixture::new(
                    Uuid::new_v4(),
                    stage_instance_id,
                    sm.round_index,
                    sm.home_team_id,
                    sm.away_team_id,
                    sm.is_neutral_venue,
                    sm.scheduled_date,
                    FixtureStatus::Scheduled,
                    None,
                ));
            }

            Ok(GeneratedSeason {
                season_instance,
                stage_instance,
                fixtures,
            })
        }
    }
}

pub async fn generate_season_for_league(
    pool: &SqlitePool,
    competition_id: Uuid,
    calendar_system_id: Uuid,
    reference_year: i64,
) -> ControllerResult<GeneratedSeason> {
    let config_arc = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!(
            "Calendar system {} not found",
            calendar_system_id
        ))
    })?;

    let teams = arlo_db::repositories::team::list_by_league_id(pool, competition_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let team_ids: Vec<Uuid> = teams.iter().map(|t| t.id()).collect();

    let generated = generate_season(calendar, &config_arc, &team_ids, reference_year)?;

    persist_generated_season(pool, &generated).await?;

    Ok(generated)
}
