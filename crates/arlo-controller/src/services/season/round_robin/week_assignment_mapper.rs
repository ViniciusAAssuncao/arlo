use crate::domain::calendar::{CalendarDate, CalendarSystem, ResolvedCalendarDate};
use crate::error::ControllerResult;
use crate::services::calendar::{date_advancer, date_encoder, date_resolver};
use crate::services::season::round_robin::circle_method_generator::RoundRobinMatch;
use arlo_domain::SeasonTiming;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledMatch {
    pub round_index: u32,
    pub home_team_id: Uuid,
    pub away_team_id: Uuid,
    pub is_neutral_venue: bool,
    pub scheduled_date: CalendarDate,
}

pub fn assign_dates(
    calendar: &CalendarSystem,
    timing: &SeasonTiming,
    reference_year: i64,
    matches: &[RoundRobinMatch],
) -> ControllerResult<Vec<ScheduledMatch>> {
    if matches.is_empty() {
        return Ok(Vec::new());
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

    let mut current_round = u32::MAX;
    let mut match_idx_in_round: usize = 0;
    let mut scheduled = Vec::with_capacity(matches.len());

    for m in matches {
        if m.round_index != current_round {
            current_round = m.round_index;
            match_idx_in_round = 0;
        }

        let week_number = m.round_index as i64;
        let week_base_date = date_advancer::advance(calendar, &start_date, week_number * week_len);
        let base_resolved = date_resolver::resolve(calendar, &week_base_date);

        let base_weekday = match base_resolved {
            ResolvedCalendarDate::RegularDay {
                week_day_index, ..
            } => week_day_index,
            ResolvedCalendarDate::IntercalaryDay {
                week_day_index, ..
            } => week_day_index.unwrap_or(0),
        };

        let target_weekday = if !allowed_weekdays.is_empty() {
            allowed_weekdays[match_idx_in_round % allowed_weekdays.len()]
        } else {
            base_weekday
        };

        let day_offset = (target_weekday as i64 - base_weekday as i64).rem_euclid(week_len);
        let scheduled_date = date_advancer::advance(calendar, &week_base_date, day_offset);

        scheduled.push(ScheduledMatch {
            round_index: m.round_index,
            home_team_id: m.home_team_id,
            away_team_id: m.away_team_id,
            is_neutral_venue: m.is_neutral_venue,
            scheduled_date,
        });

        match_idx_in_round += 1;
    }

    Ok(scheduled)
}
