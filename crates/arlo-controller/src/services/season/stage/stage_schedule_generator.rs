use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::domain::season::{
    BracketSeed, Fixture, FixtureStatus, KnockoutTie, SeasonStageInstance, StageStatus,
};
use crate::error::{ControllerError, ControllerResult};
use crate::services::season::grouped_schedule::generate_grouped_schedule;
use crate::services::season::round_robin::{
    assign_dates, expand_double_round_robin, generate_single_round_robin, resolve_neutral_opener,
    RoundRobinMatch,
};
use crate::services::season::stage::knockout_bracket_generator::generate_knockout_bracket;
use arlo_domain::{LeagueCalendarConfig, ScheduleAlgorithmKind, StageDefinition, StageType};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedStageSchedule {
    pub stage_instance: SeasonStageInstance,
    pub fixtures: Vec<Fixture>,
    pub knockout_ties: Vec<KnockoutTie>,
}

pub fn generate_stage_schedule(
    calendar: &CalendarSystem,
    config: &LeagueCalendarConfig,
    stage_def: &StageDefinition,
    season_instance_id: Uuid,
    stage_instance_id: Uuid,
    participating_teams: &[Uuid],
    anchor_date: CalendarDate,
    start_round_index: u32,
) -> ControllerResult<GeneratedStageSchedule> {
    match stage_def.stage_type() {
        StageType::RoundRobinTable => {
            if participating_teams.len() < 2 {
                return Err(ControllerError::Validation(
                    "At least 2 teams are required for a round robin stage".to_string(),
                ));
            }

            let single_leg_matches = generate_single_round_robin(participating_teams);

            let mut matches = match config.algorithm() {
                ScheduleAlgorithmKind::RoundRobinSingleLeg => single_leg_matches,
                ScheduleAlgorithmKind::RoundRobinDoubleLeg => {
                    expand_double_round_robin(&single_leg_matches)
                }
            };

            if stage_def.stage_order_index() == 0 {
                resolve_neutral_opener(&mut matches, &config.neutral_opener());
            }

            let matches_with_offset: Vec<RoundRobinMatch> = if start_round_index > 0 {
                matches
                    .into_iter()
                    .map(|m| RoundRobinMatch {
                        round_index: m.round_index + start_round_index,
                        home_team_id: m.home_team_id,
                        away_team_id: m.away_team_id,
                        is_neutral_venue: m.is_neutral_venue,
                    })
                    .collect()
            } else {
                matches
            };

            let scheduled_matches = assign_dates(
                calendar,
                config.timing(),
                anchor_date,
                &matches_with_offset,
            )?;

            let mut fixtures = Vec::with_capacity(scheduled_matches.len());
            for sm in scheduled_matches {
                fixtures.push(Fixture::new(
                    Uuid::new_v4(),
                    stage_instance_id,
                    sm.round_index,
                    sm.home_team_id,
                    sm.away_team_id,
                    sm.is_neutral_venue,
                    None,
                    sm.scheduled_date,
                    FixtureStatus::Scheduled,
                    None,
                ));
            }

            let stage_instance = SeasonStageInstance::new(
                stage_instance_id,
                season_instance_id,
                stage_def.stage_order_index(),
                stage_def.stage_type(),
                StageStatus::Pending,
            );

            Ok(GeneratedStageSchedule {
                stage_instance,
                fixtures,
                knockout_ties: Vec::new(),
            })
        }
        StageType::KnockoutBracket => {
            let leg_format = stage_def.knockout_leg_format().ok_or_else(|| {
                ControllerError::Validation(
                    "Knockout bracket stage requires knockout_leg_format".to_string(),
                )
            })?;

            let seeds: Vec<BracketSeed> = participating_teams
                .iter()
                .enumerate()
                .map(|(idx, &team_id)| BracketSeed::new((idx + 1) as u32, team_id))
                .collect();

            let generated_knockout = generate_knockout_bracket(
                calendar,
                config.timing(),
                anchor_date,
                stage_instance_id,
                start_round_index,
                &seeds,
                leg_format,
            )?;

            let stage_instance = SeasonStageInstance::new(
                stage_instance_id,
                season_instance_id,
                stage_def.stage_order_index(),
                stage_def.stage_type(),
                StageStatus::Pending,
            );

            Ok(GeneratedStageSchedule {
                stage_instance,
                fixtures: generated_knockout.fixtures,
                knockout_ties: generated_knockout.ties,
            })
        }
        StageType::GroupedCompetitionTable => generate_grouped_schedule(
            calendar,
            config,
            stage_def,
            season_instance_id,
            stage_instance_id,
            anchor_date,
            start_round_index,
        ),
    }
}

pub fn generate_stage_schedule_from_seeds(
    calendar: &CalendarSystem,
    config: &LeagueCalendarConfig,
    stage_def: &StageDefinition,
    season_instance_id: Uuid,
    stage_instance_id: Uuid,
    seeds: &[BracketSeed],
    anchor_date: CalendarDate,
    start_round_index: u32,
) -> ControllerResult<GeneratedStageSchedule> {
    let team_ids: Vec<Uuid> = seeds.iter().map(|s| s.team_id()).collect();
    generate_stage_schedule(
        calendar,
        config,
        stage_def,
        season_instance_id,
        stage_instance_id,
        &team_ids,
        anchor_date,
        start_round_index,
    )
}