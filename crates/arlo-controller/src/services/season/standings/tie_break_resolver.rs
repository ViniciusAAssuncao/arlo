use crate::domain::season::{StandingsEntry, TieBreakCriterion};
use std::cmp::Ordering;

pub fn resolve_standings(
    entries: &mut [StandingsEntry],
    criteria: &[TieBreakCriterion],
) {
    let active_criteria = if criteria.is_empty() {
        &[TieBreakCriterion::PointsTotal][..]
    } else {
        criteria
    };

    entries.sort_by(|a, b| {
        for criterion in active_criteria {
            let ordering = match criterion {
                TieBreakCriterion::PointsTotal => b.points().cmp(&a.points()),
                TieBreakCriterion::HeadToHead => Ordering::Equal,
                TieBreakCriterion::Random => Ordering::Equal,
            };

            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        b.won()
            .cmp(&a.won())
            .then_with(|| a.lost().cmp(&b.lost()))
            .then_with(|| a.team_id().cmp(&b.team_id()))
    });
}

pub fn sort_standings(
    mut entries: Vec<StandingsEntry>,
    criteria: &[TieBreakCriterion],
) -> Vec<StandingsEntry> {
    resolve_standings(&mut entries, criteria);
    entries
}
