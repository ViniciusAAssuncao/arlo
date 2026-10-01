use crate::domain::calendar::{BlackoutWindow, CalendarDate, CalendarSystem};
use crate::domain::season::{Fixture, FixtureStatus, PostponementReason, PostponementRecord};
use crate::error::{ControllerError, ControllerResult};
use crate::services::season::conflict::games_per_week_conflict_detector::TeamWeekConflict;
use crate::services::season::conflict::next_valid_date_locator::find_next_valid_date_for_fixture;
use crate::services::season::conflict::team_fixture_window_loader::calculate_fixture_week_index;
use arlo_domain::{GamesPerWeekPolicy, RestGapPolicy};
use std::collections::HashSet;
use uuid::Uuid;

pub fn resolve_games_per_week_conflicts(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    conflicts: &[TeamWeekConflict],
    fixtures: &mut [Fixture],
    allowed_weekdays: &[u32],
    games_per_week_policy: &GamesPerWeekPolicy,
    rest_gap_policy: &RestGapPolicy,
    blackout_windows: &[BlackoutWindow],
    competition_stage_ids: &[Uuid],
    max_search_weeks: u32,
) -> ControllerResult<Vec<PostponementRecord>> {
    let mut conflicting_fixture_ids = HashSet::new();
    for conflict in conflicts {
        for &f_id in &conflict.conflicting_fixture_ids {
            conflicting_fixture_ids.insert(f_id);
        }
    }

    let mut sorted_fixture_ids: Vec<Uuid> = conflicting_fixture_ids.into_iter().collect();
    sorted_fixture_ids.sort_by_key(|id| {
        fixtures
            .iter()
            .find(|f| f.id() == *id)
            .map(|f| (f.scheduled_date(), f.round_index(), f.id()))
    });

    let mut postponement_records = Vec::new();

    for fixture_id in sorted_fixture_ids {
        let fixture_idx = match fixtures.iter().position(|f| f.id() == fixture_id) {
            Some(idx) => idx,
            None => continue,
        };

        let fixture = fixtures[fixture_idx];
        if fixture.status() == FixtureStatus::Completed
            || fixture.status() == FixtureStatus::Cancelled
        {
            continue;
        }

        let new_date = match find_next_valid_date_for_fixture(
            calendar,
            season_start_date,
            &fixture.scheduled_date(),
            fixture.home_team_id(),
            fixture.away_team_id(),
            fixture.id(),
            fixtures,
            allowed_weekdays,
            games_per_week_policy,
            rest_gap_policy,
            blackout_windows,
            competition_stage_ids,
            max_search_weeks,
        ) {
            Some(d) => d,
            None => {
                return Err(ControllerError::Validation(format!(
                    "Could not find available valid date for fixture {}",
                    fixture.id()
                )));
            }
        };

        let target_week = calculate_fixture_week_index(calendar, season_start_date, &new_date)
            .unwrap_or(fixture.round_index());

        let record = PostponementRecord::new(
            Uuid::new_v4(),
            fixture.id(),
            fixture.scheduled_date(),
            new_date,
            PostponementReason::GamesPerWeekConflict,
        );

        let updated_fixture = Fixture::new(
            fixture.id(),
            fixture.season_stage_id(),
            target_week,
            fixture.home_team_id(),
            fixture.away_team_id(),
            fixture.is_neutral_venue(),
            fixture.venue_id(),
            new_date,
            FixtureStatus::Postponed,
            fixture.result(),
        );

        fixtures[fixture_idx] = updated_fixture;
        postponement_records.push(record);
    }

    Ok(postponement_records)
}
