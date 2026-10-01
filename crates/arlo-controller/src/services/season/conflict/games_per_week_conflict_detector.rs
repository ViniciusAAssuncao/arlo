use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::domain::season::Fixture;
use crate::services::calendar::date_advancer;
use crate::services::season::conflict::team_fixture_window_loader::{
    calculate_fixture_week_index, filter_active_team_fixtures,
};
use arlo_domain::GamesPerWeekPolicy;
use rayon::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamWeekConflict {
    pub team_id: Uuid,
    pub week_index: u32,
    pub week_start_date: CalendarDate,
    pub week_end_date: CalendarDate,
    pub conflicting_fixture_ids: Vec<Uuid>,
    pub total_games: usize,
    pub max_allowed: usize,
}

pub fn detect_conflicts_for_teams(
    calendar: &CalendarSystem,
    season_start_date: &CalendarDate,
    fixtures: &[Fixture],
    team_ids: &[Uuid],
    policy: &GamesPerWeekPolicy,
    competition_stage_ids: &[Uuid],
) -> Vec<TeamWeekConflict> {
    if policy.max_games_per_team_per_week() == 0 {
        return Vec::new();
    }

    let max_allowed = policy.max_games_per_team_per_week() as usize;
    let scope = policy.conflict_scope();
    let week_len = if !calendar.week_days().is_empty() {
        calendar.week_days().len() as i64
    } else {
        7
    };

    team_ids
        .par_iter()
        .flat_map(|&team_id| {
            let team_fixtures =
                filter_active_team_fixtures(fixtures, team_id, scope, competition_stage_ids);

            let mut fixtures_by_week: BTreeMap<u32, Vec<&Fixture>> = BTreeMap::new();
            for f in team_fixtures {
                if let Some(week_idx) =
                    calculate_fixture_week_index(calendar, season_start_date, &f.scheduled_date())
                {
                    fixtures_by_week.entry(week_idx).or_default().push(f);
                }
            }

            let mut team_conflicts = Vec::new();
            for (week_idx, mut week_fixtures) in fixtures_by_week {
                if week_fixtures.len() > max_allowed {
                    week_fixtures.sort_by_key(|f| (f.scheduled_date(), f.round_index()));
                    let excess_fixtures = &week_fixtures[max_allowed..];
                    let conflicting_fixture_ids: Vec<Uuid> =
                        excess_fixtures.iter().map(|f| f.id()).collect();

                    let week_start_date = date_advancer::advance(
                        calendar,
                        season_start_date,
                        week_idx as i64 * week_len,
                    );
                    let week_end_date =
                        date_advancer::advance(calendar, &week_start_date, week_len - 1);

                    team_conflicts.push(TeamWeekConflict {
                        team_id,
                        week_index: week_idx,
                        week_start_date,
                        week_end_date,
                        conflicting_fixture_ids,
                        total_games: week_fixtures.len(),
                        max_allowed,
                    });
                }
            }

            team_conflicts
        })
        .collect()
}
