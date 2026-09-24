use crate::domain::calendar::CalendarSystem;
use crate::domain::season::{Fixture, FixtureStatus};
use crate::services::season::conflict::team_fixture_window_loader::{
    days_between, filter_active_team_fixtures,
};
use arlo_domain::RestGapPolicy;
use rayon::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestGapConflict {
    pub team_id: Uuid,
    pub previous_fixture_id: Uuid,
    pub conflicting_fixture_id: Uuid,
    pub gap_days: i64,
    pub minimum_days: u32,
}

pub fn detect_rest_gap_conflicts_for_teams(
    calendar: &CalendarSystem,
    fixtures: &[Fixture],
    team_ids: &[Uuid],
    policy: &RestGapPolicy,
    competition_stage_ids: &[Uuid],
) -> Vec<RestGapConflict> {
    if policy.minimum_days_between_fixtures() == 0 {
        return Vec::new();
    }

    let min_days = policy.minimum_days_between_fixtures() as i64;
    let scope = policy.conflict_scope();

    team_ids
        .par_iter()
        .flat_map(|&team_id| {
            let mut team_fixtures =
                filter_active_team_fixtures(fixtures, team_id, scope, competition_stage_ids);

            team_fixtures.sort_by_key(|f| (f.scheduled_date(), f.round_index(), f.id()));

            let mut conflicts = Vec::new();
            for window in team_fixtures.windows(2) {
                let f1 = window[0];
                let f2 = window[1];

                let gap = days_between(calendar, &f1.scheduled_date(), &f2.scheduled_date());
                if gap < min_days {
                    if f2.status() != FixtureStatus::Completed
                        && f2.status() != FixtureStatus::Cancelled
                    {
                        conflicts.push(RestGapConflict {
                            team_id,
                            previous_fixture_id: f1.id(),
                            conflicting_fixture_id: f2.id(),
                            gap_days: gap,
                            minimum_days: policy.minimum_days_between_fixtures(),
                        });
                    }
                }
            }

            conflicts
        })
        .collect()
}
