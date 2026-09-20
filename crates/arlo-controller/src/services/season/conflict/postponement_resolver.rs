use crate::domain::calendar::{BlackoutWindow, CalendarSystem, ResolvedCalendarDate};
use crate::domain::season::{Fixture, FixtureStatus, PostponementReason, PostponementRecord};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::date_encoder;
use crate::services::season::conflict::blackout_conflict_detector::detect_blackout_conflicts;
use crate::services::season::conflict::conflict_scan_report::ConflictScanReport;
use crate::services::season::conflict::games_per_week_conflict_detector::detect_conflicts_for_teams;
use crate::services::season::conflict::next_valid_date_locator::find_next_valid_date_for_fixture;
use crate::services::season::conflict::rest_gap_conflict_detector::detect_rest_gap_conflicts_for_teams;
use crate::services::season::conflict::team_fixture_window_loader::calculate_fixture_week_index;
use arlo_domain::{GamesPerWeekPolicy, PostponementPolicy, PostponementStrategyKind, RestGapPolicy, SeasonTiming};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_conflicts_and_postpone(
    calendar: &CalendarSystem,
    timing: &SeasonTiming,
    reference_year: i64,
    competition_id: Uuid,
    competition_stage_ids: &[Uuid],
    games_per_week_policy: &GamesPerWeekPolicy,
    rest_gap_policy: &RestGapPolicy,
    blackout_windows: &[BlackoutWindow],
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

    let gpw_conflicts = detect_conflicts_for_teams(
        calendar,
        &season_start_date,
        fixtures,
        team_ids,
        games_per_week_policy,
        competition_stage_ids,
    );

    let rest_gap_conflicts = detect_rest_gap_conflicts_for_teams(
        calendar,
        fixtures,
        team_ids,
        rest_gap_policy,
        competition_stage_ids,
    );

    let blackout_conflicts = detect_blackout_conflicts(
        fixtures,
        blackout_windows,
        competition_stage_ids,
    );

    if gpw_conflicts.is_empty() && rest_gap_conflicts.is_empty() && blackout_conflicts.is_empty() {
        return Ok(ConflictScanReport::empty(competition_id));
    }

    let mut reasons_by_fixture: HashMap<Uuid, PostponementReason> = HashMap::new();

    for c in &blackout_conflicts {
        reasons_by_fixture
            .entry(c.fixture_id)
            .or_insert(PostponementReason::CollectiveAgreementBlackout);
    }

    for c in &rest_gap_conflicts {
        reasons_by_fixture
            .entry(c.conflicting_fixture_id)
            .or_insert(PostponementReason::InsufficientRestGap);
    }

    for c in &gpw_conflicts {
        for &f_id in &c.conflicting_fixture_ids {
            reasons_by_fixture
                .entry(f_id)
                .or_insert(PostponementReason::GamesPerWeekConflict);
        }
    }

    let conflicts_detected = reasons_by_fixture.len();
    let mut postponement_records = Vec::new();

    match postponement_policy.strategy() {
        PostponementStrategyKind::NextAvailableByeWeek => {
            let mut sorted_fixture_ids: Vec<Uuid> = reasons_by_fixture.keys().copied().collect();
            sorted_fixture_ids.sort_by_key(|id| {
                fixtures
                    .iter()
                    .find(|f| f.id() == *id)
                    .map(|f| (f.scheduled_date(), f.round_index(), f.id()))
            });

            for fixture_id in sorted_fixture_ids {
                let reason = match reasons_by_fixture.get(&fixture_id) {
                    Some(&r) => r,
                    None => continue,
                };

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
                    &season_start_date,
                    &fixture.scheduled_date(),
                    fixture.home_team_id(),
                    fixture.away_team_id(),
                    fixture.id(),
                    fixtures,
                    timing.allowed_weekdays(),
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

                let target_week = calculate_fixture_week_index(
                    calendar,
                    &season_start_date,
                    &new_date,
                )
                .unwrap_or(fixture.round_index());

                let record = PostponementRecord::new(
                    Uuid::new_v4(),
                    fixture.id(),
                    fixture.scheduled_date(),
                    new_date,
                    reason,
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