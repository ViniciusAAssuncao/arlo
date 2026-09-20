use crate::domain::calendar::{CalendarDate, CalendarSystem, ResolvedCalendarDate};
use crate::domain::season::{BracketSeed, Fixture, FixtureStatus, KnockoutTie};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::{date_advancer, date_resolver};
use arlo_domain::{KnockoutLegFormat, SeasonTiming};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedKnockoutBracket {
    pub ties: Vec<KnockoutTie>,
    pub fixtures: Vec<Fixture>,
}

fn compute_leg_scheduled_date(
    calendar: &CalendarSystem,
    anchor_date: &CalendarDate,
    week_offset: i64,
    match_index: usize,
    week_len: i64,
    allowed_weekdays: &[u32],
) -> CalendarDate {
    let week_base_date = date_advancer::advance(
        calendar,
        anchor_date,
        week_offset * week_len,
    );
    let base_resolved = date_resolver::resolve(calendar, &week_base_date);

    let base_weekday = match base_resolved {
        ResolvedCalendarDate::RegularDay { week_day_index, .. } => week_day_index,
        ResolvedCalendarDate::IntercalaryDay { week_day_index, .. } => {
            week_day_index.unwrap_or(0)
        }
    };

    let target_weekday = if !allowed_weekdays.is_empty() {
        allowed_weekdays[match_index % allowed_weekdays.len()]
    } else {
        base_weekday
    };

    let day_offset = ((target_weekday as i64) - (base_weekday as i64)).rem_euclid(week_len);
    date_advancer::advance(calendar, &week_base_date, day_offset)
}

pub fn generate_knockout_bracket(
    calendar: &CalendarSystem,
    timing: &SeasonTiming,
    anchor_date: CalendarDate,
    stage_instance_id: Uuid,
    start_round_index: u32,
    seeds: &[BracketSeed],
    leg_format: KnockoutLegFormat,
) -> ControllerResult<GeneratedKnockoutBracket> {
    if seeds.len() < 2 {
        return Err(ControllerError::Validation(
            "At least 2 seeds are required to generate a knockout bracket".to_string(),
        ));
    }

    if seeds.len() % 2 != 0 {
        return Err(ControllerError::Validation(
            "Knockout bracket requires an even number of seeds".to_string(),
        ));
    }

    let week_len = if !calendar.week_days().is_empty() {
        calendar.week_days().len() as i64
    } else {
        7
    };

    let allowed_weekdays = timing.allowed_weekdays();

    let tie_count = seeds.len() / 2;
    let mut ties = Vec::with_capacity(tie_count);
    let mut fixtures = match leg_format {
        KnockoutLegFormat::SingleLeg => Vec::with_capacity(tie_count),
        KnockoutLegFormat::TwoLegAggregate => Vec::with_capacity(tie_count * 2),
    };

    for i in 0..tie_count {
        let high_seed = seeds[i];
        let low_seed = seeds[seeds.len() - 1 - i];

        match leg_format {
            KnockoutLegFormat::SingleLeg => {
                let round_index = start_round_index;
                let scheduled_date = compute_leg_scheduled_date(
                    calendar,
                    &anchor_date,
                    0,
                    i,
                    week_len,
                    allowed_weekdays,
                );

                let fixture_id = Uuid::new_v4();
                let fixture = Fixture::new(
                    fixture_id,
                    stage_instance_id,
                    round_index,
                    high_seed.team_id(),
                    low_seed.team_id(),
                    false,
                    None,
                    scheduled_date,
                    FixtureStatus::Scheduled,
                    None,
                );

                let tie = KnockoutTie::new(
                    Uuid::new_v4(),
                    stage_instance_id,
                    round_index,
                    i as u32,
                    high_seed,
                    low_seed,
                    fixture_id,
                    None,
                    None,
                );

                fixtures.push(fixture);
                ties.push(tie);
            }
            KnockoutLegFormat::TwoLegAggregate => {
                let leg1_round_index = start_round_index;
                let leg1_scheduled_date = compute_leg_scheduled_date(
                    calendar,
                    &anchor_date,
                    0,
                    i,
                    week_len,
                    allowed_weekdays,
                );

                let leg1_fixture_id = Uuid::new_v4();
                let leg1_fixture = Fixture::new(
                    leg1_fixture_id,
                    stage_instance_id,
                    leg1_round_index,
                    low_seed.team_id(),
                    high_seed.team_id(),
                    false,
                    None,
                    leg1_scheduled_date,
                    FixtureStatus::Scheduled,
                    None,
                );

                let leg2_round_index = start_round_index + 1;
                let leg2_scheduled_date = compute_leg_scheduled_date(
                    calendar,
                    &anchor_date,
                    1,
                    i,
                    week_len,
                    allowed_weekdays,
                );

                let leg2_fixture_id = Uuid::new_v4();
                let leg2_fixture = Fixture::new(
                    leg2_fixture_id,
                    stage_instance_id,
                    leg2_round_index,
                    high_seed.team_id(),
                    low_seed.team_id(),
                    false,
                    None,
                    leg2_scheduled_date,
                    FixtureStatus::Scheduled,
                    None,
                );

                let tie = KnockoutTie::new(
                    Uuid::new_v4(),
                    stage_instance_id,
                    start_round_index,
                    i as u32,
                    high_seed,
                    low_seed,
                    leg1_fixture_id,
                    Some(leg2_fixture_id),
                    None,
                );

                fixtures.push(leg1_fixture);
                fixtures.push(leg2_fixture);
                ties.push(tie);
            }
        }
    }

    Ok(GeneratedKnockoutBracket { ties, fixtures })
}