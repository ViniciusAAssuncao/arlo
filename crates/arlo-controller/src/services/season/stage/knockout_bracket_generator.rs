use crate::domain::calendar::{ CalendarSystem, ResolvedCalendarDate };
use crate::domain::season::{ BracketSeed, Fixture, FixtureStatus, KnockoutTie };
use crate::error::{ ControllerError, ControllerResult };
use crate::services::calendar::{ date_advancer, date_encoder, date_resolver };
use arlo_domain::{ KnockoutLegFormat, SeasonTiming };
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedKnockoutBracket {
    pub ties: Vec<KnockoutTie>,
    pub fixtures: Vec<Fixture>,
}

pub fn generate_knockout_bracket(
    calendar: &CalendarSystem,
    timing: &SeasonTiming,
    reference_year: i64,
    stage_instance_id: Uuid,
    start_round_index: u32,
    seeds: &[BracketSeed],
    leg_format: KnockoutLegFormat
) -> ControllerResult<GeneratedKnockoutBracket> {
    if seeds.len() < 2 {
        return Err(
            ControllerError::Validation(
                "At least 2 seeds are required to generate a knockout bracket".to_string()
            )
        );
    }

    if seeds.len() % 2 != 0 {
        return Err(
            ControllerError::Validation(
                "Knockout bracket requires an even number of seeds".to_string()
            )
        );
    }

    let start_resolved = ResolvedCalendarDate::RegularDay {
        year: reference_year,
        month_order_index: timing.start_month_order_index(),
        day_of_month: timing.start_day_of_month(),
        week_day_index: 0,
    };
    let start_date = date_encoder::encode(calendar, &start_resolved)?;

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
                let week_number = round_index as i64;
                let week_base_date = date_advancer::advance(
                    calendar,
                    &start_date,
                    week_number * week_len
                );
                let base_resolved = date_resolver::resolve(calendar, &week_base_date);

                let base_weekday = match base_resolved {
                    ResolvedCalendarDate::RegularDay { week_day_index, .. } => week_day_index,
                    ResolvedCalendarDate::IntercalaryDay { week_day_index, .. } =>
                        week_day_index.unwrap_or(0),
                };

                let target_weekday = if !allowed_weekdays.is_empty() {
                    allowed_weekdays[i % allowed_weekdays.len()]
                } else {
                    base_weekday
                };

                let day_offset = ((target_weekday as i64) - (base_weekday as i64)).rem_euclid(
                    week_len
                );
                let scheduled_date = date_advancer::advance(calendar, &week_base_date, day_offset);

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
                    None
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
                    None
                );

                fixtures.push(fixture);
                ties.push(tie);
            }
            KnockoutLegFormat::TwoLegAggregate => {
                let leg1_round_index = start_round_index;
                let leg1_week_number = leg1_round_index as i64;
                let leg1_week_base_date = date_advancer::advance(
                    calendar,
                    &start_date,
                    leg1_week_number * week_len
                );
                let leg1_base_resolved = date_resolver::resolve(calendar, &leg1_week_base_date);

                let leg1_base_weekday = match leg1_base_resolved {
                    ResolvedCalendarDate::RegularDay { week_day_index, .. } => week_day_index,
                    ResolvedCalendarDate::IntercalaryDay { week_day_index, .. } =>
                        week_day_index.unwrap_or(0),
                };

                let leg1_target_weekday = if !allowed_weekdays.is_empty() {
                    allowed_weekdays[i % allowed_weekdays.len()]
                } else {
                    leg1_base_weekday
                };

                let leg1_day_offset = (
                    (leg1_target_weekday as i64) - (leg1_base_weekday as i64)
                ).rem_euclid(week_len);
                let leg1_scheduled_date = date_advancer::advance(
                    calendar,
                    &leg1_week_base_date,
                    leg1_day_offset
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
                    None
                );

                let leg2_round_index = start_round_index + 1;
                let leg2_week_number = leg2_round_index as i64;
                let leg2_week_base_date = date_advancer::advance(
                    calendar,
                    &start_date,
                    leg2_week_number * week_len
                );
                let leg2_base_resolved = date_resolver::resolve(calendar, &leg2_week_base_date);

                let leg2_base_weekday = match leg2_base_resolved {
                    ResolvedCalendarDate::RegularDay { week_day_index, .. } => week_day_index,
                    ResolvedCalendarDate::IntercalaryDay { week_day_index, .. } =>
                        week_day_index.unwrap_or(0),
                };

                let leg2_target_weekday = if !allowed_weekdays.is_empty() {
                    allowed_weekdays[i % allowed_weekdays.len()]
                } else {
                    leg2_base_weekday
                };

                let leg2_day_offset = (
                    (leg2_target_weekday as i64) - (leg2_base_weekday as i64)
                ).rem_euclid(week_len);
                let leg2_scheduled_date = date_advancer::advance(
                    calendar,
                    &leg2_week_base_date,
                    leg2_day_offset
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
                    None
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
                    None
                );

                fixtures.push(leg1_fixture);
                fixtures.push(leg2_fixture);
                ties.push(tie);
            }
        }
    }

    Ok(GeneratedKnockoutBracket { ties, fixtures })
}
