use crate::domain::calendar::{BlackoutWindow, CalendarDate, CalendarSystem};
use crate::domain::season::Fixture;
use crate::services::season::conflict::team_fixture_window_loader::{
    calculate_fixture_week_index, days_between, filter_active_team_fixtures,
};
use arlo_domain::{GamesPerWeekPolicy, RestGapPolicy};
use uuid::Uuid;

pub fn is_in_blackout(
    date: &CalendarDate,
    blackout_windows: &[BlackoutWindow],
) -> bool {
    blackout_windows.iter().any(|window| window.contains(date))
}

pub fn is_valid_candidate_date_for_team(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    candidate_date: &CalendarDate,
    team_id: Uuid,
    moving_fixture_id: Uuid,
    fixtures: &[Fixture],
    games_per_week_policy: &GamesPerWeekPolicy,
    rest_gap_policy: &RestGapPolicy,
    competition_stage_ids: &[Uuid],
) -> bool {
    let candidate_week = match calculate_fixture_week_index(calendar, season_start_date, candidate_date) {
        Some(w) => w,
        None => return false,
    };

    if games_per_week_policy.max_games_per_team_per_week() > 0 {
        let max_allowed = games_per_week_policy.max_games_per_team_per_week() as usize;
        let gpw_fixtures = filter_active_team_fixtures(
            fixtures,
            team_id,
            games_per_week_policy.conflict_scope(),
            competition_stage_ids,
        );
        let count_in_week = gpw_fixtures
            .iter()
            .filter(|f| f.id() != moving_fixture_id)
            .filter(|f| {
                calculate_fixture_week_index(calendar, season_start_date, &f.scheduled_date())
                    == Some(candidate_week)
            })
            .count();

        if count_in_week + 1 > max_allowed {
            return false;
        }
    }

    if rest_gap_policy.minimum_days_between_fixtures() > 0 {
        let min_gap = rest_gap_policy.minimum_days_between_fixtures() as i64;
        let rg_fixtures = filter_active_team_fixtures(
            fixtures,
            team_id,
            rest_gap_policy.conflict_scope(),
            competition_stage_ids,
        );
        for f in rg_fixtures {
            if f.id() == moving_fixture_id {
                continue;
            }
            let gap = days_between(calendar, &f.scheduled_date(), candidate_date).abs();
            if gap < min_gap {
                return false;
            }
        }
    }

    true
}

pub fn is_valid_candidate_date(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    candidate_date: &CalendarDate,
    home_team_id: Uuid,
    away_team_id: Uuid,
    moving_fixture_id: Uuid,
    fixtures: &[Fixture],
    games_per_week_policy: &GamesPerWeekPolicy,
    rest_gap_policy: &RestGapPolicy,
    blackout_windows: &[BlackoutWindow],
    competition_stage_ids: &[Uuid],
) -> bool {
    if is_in_blackout(candidate_date, blackout_windows) {
        return false;
    }

    if !is_valid_candidate_date_for_team(
        calendar,
        season_start_date,
        candidate_date,
        home_team_id,
        moving_fixture_id,
        fixtures,
        games_per_week_policy,
        rest_gap_policy,
        competition_stage_ids,
    ) {
        return false;
    }

    if !is_valid_candidate_date_for_team(
        calendar,
        season_start_date,
        candidate_date,
        away_team_id,
        moving_fixture_id,
        fixtures,
        games_per_week_policy,
        rest_gap_policy,
        competition_stage_ids,
    ) {
        return false;
    }

    true
}