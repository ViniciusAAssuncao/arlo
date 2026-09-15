use crate::domain::calendar::CalendarSystem;
use crate::domain::season::{ Fixture, FixtureStatus, SeasonStageInstance, StageStatus };
use crate::error::{ ControllerError, ControllerResult };
use crate::services::season::grouped_schedule::cross_group_pairing_generator::generate_cross_group_pairings;
use crate::services::season::grouped_schedule::random_pool_round_generator::generate_random_pool_rounds;
use crate::services::season::round_robin::{
    assign_dates,
    expand_double_round_robin,
    generate_single_round_robin,
    resolve_neutral_opener,
    RoundRobinMatch,
};
use crate::services::season::stage::stage_schedule_generator::GeneratedStageSchedule;
use arlo_domain::{
    CompetitionGroup,
    LeagueCalendarConfig,
    ScheduleAlgorithmKind,
    ScheduleBlock,
    StageDefinition,
};
use uuid::Uuid;

pub fn generate_grouped_schedule(
    calendar: &CalendarSystem,
    config: &LeagueCalendarConfig,
    stage_def: &StageDefinition,
    season_instance_id: Uuid,
    stage_instance_id: Uuid,
    reference_year: i64,
    start_round_index: u32
) -> ControllerResult<GeneratedStageSchedule> {
    let schedule_blocks = stage_def
        .schedule_blocks()
        .ok_or_else(|| {
            ControllerError::Validation(
                "StageDefinition must have schedule_blocks for GroupedCompetitionTable".to_string()
            )
        })?;

    if schedule_blocks.is_empty() {
        return Err(
            ControllerError::Validation(
                "schedule_blocks must not be empty for GroupedCompetitionTable".to_string()
            )
        );
    }

    let find_group = |group_id: Uuid| -> ControllerResult<&CompetitionGroup> {
        config
            .groups()
            .iter()
            .find(|g| g.id() == group_id)
            .ok_or_else(|| {
                ControllerError::Validation(format!("Competition group {} not found", group_id))
            })
    };

    let mut current_round_index = start_round_index;
    let mut all_matches: Vec<RoundRobinMatch> = Vec::new();

    let mut i = 0;
    while i < schedule_blocks.len() {
        match &schedule_blocks[i] {
            ScheduleBlock::GroupRoundRobin { .. } => {
                let mut max_rounds_in_batch = 0;
                while i < schedule_blocks.len() {
                    if
                        let ScheduleBlock::GroupRoundRobin { group_id, algorithm } =
                            &schedule_blocks[i]
                    {
                        let group = find_group(*group_id)?;
                        let single_leg = generate_single_round_robin(group.team_ids());
                        let group_matches = match algorithm {
                            ScheduleAlgorithmKind::RoundRobinSingleLeg => single_leg,
                            ScheduleAlgorithmKind::RoundRobinDoubleLeg => {
                                expand_double_round_robin(&single_leg)
                            }
                        };
                        let rounds_used = group_matches
                            .iter()
                            .map(|m| m.round_index)
                            .max()
                            .map(|max_idx| max_idx + 1)
                            .unwrap_or(0);
                        if rounds_used > max_rounds_in_batch {
                            max_rounds_in_batch = rounds_used;
                        }
                        for m in group_matches {
                            all_matches.push(RoundRobinMatch {
                                round_index: current_round_index + m.round_index,
                                home_team_id: m.home_team_id,
                                away_team_id: m.away_team_id,
                                is_neutral_venue: m.is_neutral_venue,
                            });
                        }
                        i += 1;
                    } else {
                        break;
                    }
                }
                current_round_index += max_rounds_in_batch;
            }
            ScheduleBlock::CrossGroupPairing { group_a_id, group_b_id, mirrored } => {
                let group_a = find_group(*group_a_id)?;
                let group_b = find_group(*group_b_id)?;
                let pairings = generate_cross_group_pairings(
                    group_a.team_ids(),
                    group_b.team_ids(),
                    current_round_index,
                    *mirrored
                )?;
                let rounds_used = if *mirrored { 2 } else { 1 };
                current_round_index += rounds_used;
                all_matches.extend(pairings);
                i += 1;
            }
            ScheduleBlock::RandomPoolRounds { pool, rounds_count } => {
                let team_ids = pool.resolve(config.groups());
                let pool_matches = generate_random_pool_rounds(
                    &team_ids,
                    current_round_index,
                    *rounds_count
                );
                current_round_index += *rounds_count;
                all_matches.extend(pool_matches);
                i += 1;
            }
        }
    }

    if stage_def.stage_order_index() == 0 {
        resolve_neutral_opener(&mut all_matches, &config.neutral_opener());
    }

    all_matches.sort_by_key(|m| m.round_index);

    let scheduled_matches = assign_dates(calendar, config.timing(), reference_year, &all_matches)?;

    let mut fixtures = Vec::with_capacity(scheduled_matches.len());
    for sm in scheduled_matches {
        fixtures.push(
            Fixture::new(
                Uuid::new_v4(),
                stage_instance_id,
                sm.round_index,
                sm.home_team_id,
                sm.away_team_id,
                sm.is_neutral_venue,
                None,
                sm.scheduled_date,
                FixtureStatus::Scheduled,
                None
            )
        );
    }

    let stage_instance = SeasonStageInstance::new(
        stage_instance_id,
        season_instance_id,
        stage_def.stage_order_index(),
        stage_def.stage_type(),
        StageStatus::Pending
    );

    Ok(GeneratedStageSchedule {
        stage_instance,
        fixtures,
        knockout_ties: Vec::new(),
    })
}
