use crate::domain::calendar::{BlackoutWindow, CalendarDate};
use crate::domain::season::{Fixture, FixtureStatus};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackoutConflict {
    pub fixture_id: Uuid,
    pub scheduled_date: CalendarDate,
    pub blackout_window: BlackoutWindow,
}

pub fn detect_blackout_conflicts(
    fixtures: &[Fixture],
    blackout_windows: &[BlackoutWindow],
    competition_stage_ids: &[Uuid],
) -> Vec<BlackoutConflict> {
    if blackout_windows.is_empty() {
        return Vec::new();
    }

    let mut conflicts = Vec::new();

    for fixture in fixtures {
        if fixture.status() == FixtureStatus::Completed
            || fixture.status() == FixtureStatus::Cancelled
        {
            continue;
        }

        if !competition_stage_ids.is_empty()
            && !competition_stage_ids.contains(&fixture.season_stage_id())
        {
            continue;
        }

        for window in blackout_windows {
            if window.contains(&fixture.scheduled_date()) {
                conflicts.push(BlackoutConflict {
                    fixture_id: fixture.id(),
                    scheduled_date: fixture.scheduled_date(),
                    blackout_window: *window,
                });
                break;
            }
        }
    }

    conflicts
}