use super::plan::PreparedPlan;
use super::standings::{compare, records_for, PlayedFixture, Record};
use arlo_domain::TieBreakCriterion;
use rand::Rng;
use std::cmp::Ordering;

pub(super) fn resolve_ties<R: Rng>(
    plan: &PreparedPlan,
    ordered: Vec<usize>,
    records: &[Record],
    fixtures: &[PlayedFixture],
    rng: &mut R,
) -> Vec<usize> {
    let h2h = plan
        .plan
        .tie_break_criteria
        .contains(&TieBreakCriterion::HeadToHead);
    let random = plan
        .plan
        .tie_break_criteria
        .contains(&TieBreakCriterion::Random);
    let mut resolved = Vec::with_capacity(ordered.len());
    for group in partitions(plan, ordered, records) {
        let groups = if h2h && group.len() > 1 {
            mini_league_groups(plan, group, fixtures)
        } else {
            vec![group]
        };
        for mut group in groups {
            if random && group.len() > 1 {
                for i in (1..group.len()).rev() {
                    group.swap(i, rng.gen_range(0..=i));
                }
            }
            resolved.extend(group);
        }
    }
    resolved
}

fn mini_league_groups(
    plan: &PreparedPlan,
    group: Vec<usize>,
    fixtures: &[PlayedFixture],
) -> Vec<Vec<usize>> {
    let mut present = vec![false; plan.ids.len()];
    for &team in &group {
        present[team] = true;
    }
    let mutual: Vec<_> = fixtures
        .iter()
        .copied()
        .filter(|fixture| present[fixture.home] && present[fixture.away])
        .collect();
    let records = records_for(plan.ids.len(), &mutual);
    let mut ordered = group.clone();
    ordered.sort_by(|&a, &b| compare(plan, &records[a], &records[b]));
    let partitions = partitions(plan, ordered, &records);
    if partitions.len() == 1 {
        return vec![group];
    }
    let mut result = Vec::new();
    for subset in partitions {
        if subset.len() > 1 {
            result.extend(mini_league_groups(plan, subset, fixtures));
        } else {
            result.push(subset);
        }
    }
    result
}

fn partitions(plan: &PreparedPlan, ordered: Vec<usize>, records: &[Record]) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for team in ordered {
        if let Some(last) = groups.last_mut() {
            if compare(plan, &records[last[0]], &records[team]) == Ordering::Equal {
                last.push(team);
                continue;
            }
        }
        groups.push(vec![team]);
    }
    groups
}
