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
                TieBreakCriterion::PointsTotal => {
                    b.spa_metrics()
                        .ispa()
                        .partial_cmp(&a.spa_metrics().ispa())
                        .unwrap_or(Ordering::Equal)
                }
                TieBreakCriterion::HeadToHead => Ordering::Equal,
                TieBreakCriterion::Random => Ordering::Equal,
            };

            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        let sg_a = a.goal_points_for() as i64 - a.goal_points_against() as i64;
        let sg_b = b.goal_points_for() as i64 - b.goal_points_against() as i64;

        b.won()
            .cmp(&a.won())
            .then_with(|| sg_b.cmp(&sg_a))
            .then_with(|| b.goal_points_for().cmp(&a.goal_points_for()))
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
