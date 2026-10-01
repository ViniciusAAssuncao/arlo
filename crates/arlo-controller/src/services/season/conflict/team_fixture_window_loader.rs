use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::domain::season::{Fixture, FixtureStatus};
use crate::services::calendar::total_days_in_year;
use arlo_domain::GamesPerWeekConflictScope;
use uuid::Uuid;

pub fn days_between(calendar: &CalendarSystem, from: &CalendarDate, to: &CalendarDate) -> i64 {
    if from.year() == to.year() {
        return to.day_of_year() as i64 - from.day_of_year() as i64;
    }
    if from.year() < to.year() {
        let mut days =
            (total_days_in_year(calendar, from.year()) as i64) - from.day_of_year() as i64;
        for y in (from.year() + 1)..to.year() {
            days += total_days_in_year(calendar, y) as i64;
        }
        days += to.day_of_year() as i64;
        days
    } else {
        -days_between(calendar, to, from)
    }
}

pub fn calculate_fixture_week_index(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    fixture_date: &CalendarDate,
) -> Option<u32> {
    let days = days_between(calendar, season_start_date, fixture_date);
    if days < 0 {
        return None;
    }
    let week_len = if !calendar.week_days().is_empty() {
        calendar.week_days().len() as i64
    } else {
        7
    };
    Some((days / week_len) as u32)
}

pub fn filter_active_team_fixtures<'a>(
    fixtures: &'a [Fixture],
    team_id: Uuid,
    scope: GamesPerWeekConflictScope,
    competition_stage_ids: &[Uuid],
) -> Vec<&'a Fixture> {
    fixtures
        .iter()
        .filter(|f| {
            if f.status() == FixtureStatus::Cancelled {
                return false;
            }
            if f.home_team_id() != team_id && f.away_team_id() != team_id {
                return false;
            }
            match scope {
                GamesPerWeekConflictScope::SameCompetitionOnly => {
                    competition_stage_ids.contains(&f.season_stage_id())
                }
                GamesPerWeekConflictScope::AcrossAllCompetitions => true,
            }
        })
        .collect()
}

pub fn load_team_fixtures_in_window<'a>(
    fixtures: &'a [Fixture],
    team_id: Uuid,
    start_date: &CalendarDate,
    end_date: &CalendarDate,
    scope: GamesPerWeekConflictScope,
    competition_stage_ids: &[Uuid],
) -> Vec<&'a Fixture> {
    fixtures
        .iter()
        .filter(|f| {
            if f.status() == FixtureStatus::Cancelled {
                return false;
            }
            if f.home_team_id() != team_id && f.away_team_id() != team_id {
                return false;
            }
            let scheduled = f.scheduled_date();
            if scheduled < *start_date || scheduled > *end_date {
                return false;
            }
            match scope {
                GamesPerWeekConflictScope::SameCompetitionOnly => {
                    competition_stage_ids.contains(&f.season_stage_id())
                }
                GamesPerWeekConflictScope::AcrossAllCompetitions => true,
            }
        })
        .collect()
}
