use crate::domain::calendar::{CalendarSystem, ResolvedCalendarDate};
use crate::domain::season::{Fixture, FixtureStatus, PostponementReason, PostponementRecord};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::date_encoder;
use crate::services::season::conflict::bye_week_locator::{
    calculate_bye_week_date, find_next_bye_week_for_fixture,
};
use crate::services::season::conflict::conflict_scan_report::ConflictScanReport;
use crate::services::season::conflict::games_per_week_conflict_detector::detect_conflicts_for_teams;
use crate::services::season::conflict::team_fixture_window_loader::calculate_fixture_week_index;
use arlo_domain::{GamesPerWeekPolicy, PostponementPolicy, PostponementStrategyKind, SeasonTiming};
use std::collections::HashSet;
use uuid::Uuid;

pub fn resolve_conflicts_and_postpone(
    calendar: &CalendarSystem,
    timing: &SeasonTiming,
    reference_year: i64,
    competition_id: Uuid,
    competition_stage_ids: &[Uuid],
    games_per_week_policy: &GamesPerWeekPolicy,
    postponement_policy: &PostponementPolicy,
    fixtures: &mut [Fixture],
    team_ids: &[Uuid],
    max_search_weeks: u32,
) -> ControllerResult<ConflictScanReport> {
    let start_resolved = ResolvedCalendarDate::RegularDay {
        year: reference_year,
        month_order_index: timing.start_month_order_index(),
        day_of_month: timing.start_day_of_month(),
        week_day_index: 0,
    };
    let season_start_date = date_encoder::encode(calendar, &start_resolved)?;

    let conflicts = detect_conflicts_for_teams(
        calendar,
        &season_start_date,
        fixtures,
        team_ids,
        games_per_week_policy,
        competition_stage_ids,
    );

    if conflicts.is_empty() {
        return Ok(ConflictScanReport::empty(competition_id));
    }

    let mut conflicting_fixture_ids = HashSet::new();
    for conflict in &conflicts {
        for &f_id in &conflict.conflicting_fixture_ids {
            conflicting_fixture_ids.insert(f_id);
        }
    }

    let conflicts_detected = conflicting_fixture_ids.len();
    let mut postponement_records = Vec::new();

    match postponement_policy.strategy() {
        PostponementStrategyKind::NextAvailableByeWeek => {
            for fixture_id in conflicting_fixture_ids {
                let fixture_idx = fixtures.iter().position(|f| f.id() == fixture_id);
                let fixture_idx = match fixture_idx {
                    Some(idx) => idx,
                    None => continue,
                };

                let fixture = fixtures[fixture_idx];
                if fixture.status() == FixtureStatus::Completed
                    || fixture.status() == FixtureStatus::Cancelled
                {
                    continue;
                }

                let current_week = calculate_fixture_week_index(
                    calendar,
                    &season_start_date,
                    &fixture.scheduled_date(),
                )
                .unwrap_or(fixture.round_index());

                let target_week = find_next_bye_week_for_fixture(
                    calendar,
                    &season_start_date,
                    fixtures,
                    fixture.home_team_id(),
                    fixture.away_team_id(),
                    current_week,
                    max_search_weeks,
                    games_per_week_policy.max_games_per_team_per_week(),
                    games_per_week_policy.conflict_scope(),
                    competition_stage_ids,
                );

                let target_week = match target_week {
                    Some(w) => w,
                    None => {
                        return Err(ControllerError::Validation(format!(
                            "Could not find available bye week for fixture {}",
                            fixture.id()
                        )));
                    }
                };

                let new_date = calculate_bye_week_date(
                    calendar,
                    &season_start_date,
                    target_week,
                    timing.allowed_weekdays(),
                );

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
        }
    }

    let postponements_applied = postponement_records.len();

    Ok(ConflictScanReport::new(
        competition_id,
        conflicts_detected,
        postponements_applied,
        postponement_records,
    ))
}
