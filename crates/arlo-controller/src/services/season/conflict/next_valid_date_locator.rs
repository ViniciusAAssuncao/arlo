use crate::domain::calendar::{BlackoutWindow, CalendarDate, CalendarSystem, ResolvedCalendarDate};
use crate::domain::season::Fixture;
use crate::services::calendar::{date_advancer, date_resolver};
use crate::services::season::conflict::candidate_date_validator::{
    is_in_blackout, is_valid_candidate_date, is_valid_candidate_date_for_team,
};
use arlo_domain::{GamesPerWeekPolicy, RestGapPolicy};
use uuid::Uuid;

pub fn find_next_valid_date_for_fixture(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    from_date: &CalendarDate,
    home_team_id: Uuid,
    away_team_id: Uuid,
    moving_fixture_id: Uuid,
    fixtures: &[Fixture],
    allowed_weekdays: &[u32],
    games_per_week_policy: &GamesPerWeekPolicy,
    rest_gap_policy: &RestGapPolicy,
    blackout_windows: &[BlackoutWindow],
    competition_stage_ids: &[Uuid],
    max_search_weeks: u32,
) -> Option<CalendarDate> {
    let week_len = if !calendar.week_days().is_empty() {
        calendar.week_days().len() as u32
    } else {
        7
    };
    let max_days = max_search_weeks * week_len;

    for day_offset in 1..=max_days {
        let candidate_date = date_advancer::advance(calendar, from_date, day_offset as i64);

        if !allowed_weekdays.is_empty() {
            let resolved = date_resolver::resolve(calendar, &candidate_date);
            let weekday = match resolved {
                ResolvedCalendarDate::RegularDay {
                    week_day_index, ..
                } => week_day_index,
                ResolvedCalendarDate::IntercalaryDay {
                    week_day_index, ..
                } => week_day_index.unwrap_or(0),
            };
            if !allowed_weekdays.contains(&weekday) {
                continue;
            }
        }

        if is_valid_candidate_date(
            calendar,
            season_start_date,
            &candidate_date,
            home_team_id,
            away_team_id,
            moving_fixture_id,
            fixtures,
            games_per_week_policy,
            rest_gap_policy,
            blackout_windows,
            competition_stage_ids,
        ) {
            return Some(candidate_date);
        }
    }

    None
}

pub fn find_next_valid_date_for_team(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    from_date: &CalendarDate,
    team_id: Uuid,
    moving_fixture_id: Uuid,
    fixtures: &[Fixture],
    allowed_weekdays: &[u32],
    games_per_week_policy: &GamesPerWeekPolicy,
    rest_gap_policy: &RestGapPolicy,
    blackout_windows: &[BlackoutWindow],
    competition_stage_ids: &[Uuid],
    max_search_weeks: u32,
) -> Option<CalendarDate> {
    let week_len = if !calendar.week_days().is_empty() {
        calendar.week_days().len() as u32
    } else {
        7
    };
    let max_days = max_search_weeks * week_len;

    for day_offset in 1..=max_days {
        let candidate_date = date_advancer::advance(calendar, from_date, day_offset as i64);

        if !allowed_weekdays.is_empty() {
            let resolved = date_resolver::resolve(calendar, &candidate_date);
            let weekday = match resolved {
                ResolvedCalendarDate::RegularDay {
                    week_day_index, ..
                } => week_day_index,
                ResolvedCalendarDate::IntercalaryDay {
                    week_day_index, ..
                } => week_day_index.unwrap_or(0),
            };
            if !allowed_weekdays.contains(&weekday) {
                continue;
            }
        }

        if !is_in_blackout(&candidate_date, blackout_windows)
            && is_valid_candidate_date_for_team(
                calendar,
                season_start_date,
                &candidate_date,
                team_id,
                moving_fixture_id,
                fixtures,
                games_per_week_policy,
                rest_gap_policy,
                competition_stage_ids,
            )
        {
            return Some(candidate_date);
        }
    }

    None
}