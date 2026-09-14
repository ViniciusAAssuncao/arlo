use crate::domain::calendar::{CalendarDate, CalendarSystem, ResolvedCalendarDate};
use crate::domain::season::Fixture;
use crate::services::calendar::{date_advancer, date_resolver};
use crate::services::season::conflict::team_fixture_window_loader::{
    calculate_fixture_week_index, filter_active_team_fixtures,
};
use arlo_domain::GamesPerWeekConflictScope;
use std::collections::HashMap;
use uuid::Uuid;

pub fn count_team_games_in_week(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    fixtures: &[Fixture],
    team_id: Uuid,
    target_week_index: u32,
    scope: GamesPerWeekConflictScope,
    competition_stage_ids: &[Uuid],
) -> usize {
    let team_fixtures =
        filter_active_team_fixtures(fixtures, team_id, scope, competition_stage_ids);
    team_fixtures
        .iter()
        .filter(|f| {
            calculate_fixture_week_index(calendar, season_start_date, &f.scheduled_date())
                == Some(target_week_index)
        })
        .count()
}

pub fn find_next_bye_week_for_team(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    fixtures: &[Fixture],
    team_id: Uuid,
    from_week_index: u32,
    max_search_weeks: u32,
    max_games_per_week: u32,
    scope: GamesPerWeekConflictScope,
    competition_stage_ids: &[Uuid],
) -> Option<u32> {
    let team_fixtures =
        filter_active_team_fixtures(fixtures, team_id, scope, competition_stage_ids);
    let mut week_counts: HashMap<u32, usize> = HashMap::new();
    for f in team_fixtures {
        if let Some(w) =
            calculate_fixture_week_index(calendar, season_start_date, &f.scheduled_date())
        {
            *week_counts.entry(w).or_default() += 1;
        }
    }

    let max_allowed = max_games_per_week as usize;
    let start_week = from_week_index + 1;
    let end_week = from_week_index + max_search_weeks;

    for week_idx in start_week..=end_week {
        let count = week_counts.get(&week_idx).copied().unwrap_or(0);
        if count < max_allowed {
            return Some(week_idx);
        }
    }

    None
}

pub fn find_next_bye_week_for_fixture(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    fixtures: &[Fixture],
    home_team_id: Uuid,
    away_team_id: Uuid,
    from_week_index: u32,
    max_search_weeks: u32,
    max_games_per_week: u32,
    scope: GamesPerWeekConflictScope,
    competition_stage_ids: &[Uuid],
) -> Option<u32> {
    let home_fixtures =
        filter_active_team_fixtures(fixtures, home_team_id, scope, competition_stage_ids);
    let mut home_counts: HashMap<u32, usize> = HashMap::new();
    for f in home_fixtures {
        if let Some(w) =
            calculate_fixture_week_index(calendar, season_start_date, &f.scheduled_date())
        {
            *home_counts.entry(w).or_default() += 1;
        }
    }

    let away_fixtures =
        filter_active_team_fixtures(fixtures, away_team_id, scope, competition_stage_ids);
    let mut away_counts: HashMap<u32, usize> = HashMap::new();
    for f in away_fixtures {
        if let Some(w) =
            calculate_fixture_week_index(calendar, season_start_date, &f.scheduled_date())
        {
            *away_counts.entry(w).or_default() += 1;
        }
    }

    let max_allowed = max_games_per_week as usize;
    let start_week = from_week_index + 1;
    let end_week = from_week_index + max_search_weeks;

    for week_idx in start_week..=end_week {
        let home_count = home_counts.get(&week_idx).copied().unwrap_or(0);
        let away_count = away_counts.get(&week_idx).copied().unwrap_or(0);

        if home_count < max_allowed && away_count < max_allowed {
            return Some(week_idx);
        }
    }

    None
}

pub fn calculate_bye_week_date(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    bye_week_index: u32,
    allowed_weekdays: &[u32],
) -> CalendarDate {
    let week_len = if !calendar.week_days().is_empty() {
        calendar.week_days().len() as i64
    } else {
        7
    };

    let week_base_date = date_advancer::advance(
        calendar,
        season_start_date,
        bye_week_index as i64 * week_len,
    );

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
        allowed_weekdays[0]
    } else {
        base_weekday
    };

    let day_offset = (target_weekday as i64 - base_weekday as i64).rem_euclid(week_len);
    date_advancer::advance(calendar, &week_base_date, day_offset)
}
