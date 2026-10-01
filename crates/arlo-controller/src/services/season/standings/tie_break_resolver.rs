use crate::domain::season::StandingsEntry;
use arlo_domain::TieBreakCriterion;
use std::cmp::Ordering;

pub const DEFAULT_SIMPLE_CRITERIA: [TieBreakCriterion; 4] = [
    TieBreakCriterion::IspaTotal,
    TieBreakCriterion::QtaScore,
    TieBreakCriterion::GoalDifference,
    TieBreakCriterion::GoalPointsTotal,
];

pub fn compare_simple_criterion(
    a: &StandingsEntry,
    b: &StandingsEntry,
    criterion: TieBreakCriterion,
) -> Option<Ordering> {
    match criterion {
        TieBreakCriterion::IspaTotal => {
            let ord = b
                .spa_metrics()
                .ispa()
                .partial_cmp(&a.spa_metrics().ispa())
                .unwrap_or(Ordering::Equal);
            Some(ord)
        }
        TieBreakCriterion::QtaScore => {
            let ord = b.qta().partial_cmp(&a.qta()).unwrap_or(Ordering::Equal);
            Some(ord)
        }
        TieBreakCriterion::GoalDifference => {
            let sg_a = a.goal_points_for() as i64 - a.goal_points_against() as i64;
            let sg_b = b.goal_points_for() as i64 - b.goal_points_against() as i64;
            Some(sg_b.cmp(&sg_a))
        }
        TieBreakCriterion::GoalPointsTotal => Some(b.goal_points_for().cmp(&a.goal_points_for())),
        TieBreakCriterion::HeadToHead | TieBreakCriterion::Random => None,
    }
}

pub fn compare_simple_criteria(
    a: &StandingsEntry,
    b: &StandingsEntry,
    criteria: &[TieBreakCriterion],
) -> Ordering {
    let active_criteria: &[TieBreakCriterion] = if criteria.is_empty() {
        &DEFAULT_SIMPLE_CRITERIA
    } else {
        criteria
    };

    for &criterion in active_criteria {
        if let Some(ord) = compare_simple_criterion(a, b, criterion) {
            if ord != Ordering::Equal {
                return ord;
            }
        }
    }

    Ordering::Equal
}

pub fn sort_by_simple_criteria(entries: &mut [StandingsEntry], criteria: &[TieBreakCriterion]) {
    entries.sort_by(|a, b| compare_simple_criteria(a, b, criteria));
}

pub fn are_entries_tied_simple(
    a: &StandingsEntry,
    b: &StandingsEntry,
    criteria: &[TieBreakCriterion],
) -> bool {
    compare_simple_criteria(a, b, criteria) == Ordering::Equal
}
