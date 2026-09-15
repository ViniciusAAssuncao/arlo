use crate::domain::season::{Fixture, StandingsEntry};
use crate::services::season::standings::qta_calculator::apply_qta_to_standings;
use crate::services::season::standings::spa_metrics_calculator::apply_spa_metrics_to_standings;
use crate::services::season::standings::standings_calculator::calculate_standings;
use crate::services::season::standings::tie_group_partitioner::partition_tied_groups;
use arlo_domain::{QtaWeightingPolicy, SpaScoringPolicy, TieBreakCriterion};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub fn resolve_head_to_head(
    tied_group: &[StandingsEntry],
    all_fixtures: &[Fixture],
    spa_policy: &SpaScoringPolicy,
    qta_policy: &QtaWeightingPolicy,
    criteria: &[TieBreakCriterion],
) -> Vec<StandingsEntry> {
    if tied_group.len() <= 1 {
        return tied_group.to_vec();
    }

    let original_map: HashMap<Uuid, StandingsEntry> = tied_group
        .iter()
        .map(|e| (e.team_id(), *e))
        .collect();

    let team_ids_set: HashSet<Uuid> = tied_group.iter().map(|e| e.team_id()).collect();
    let team_ids: Vec<Uuid> = tied_group.iter().map(|e| e.team_id()).collect();

    let h2h_fixtures: Vec<Fixture> = all_fixtures
        .iter()
        .filter(|f| {
            team_ids_set.contains(&f.home_team_id()) && team_ids_set.contains(&f.away_team_id())
        })
        .copied()
        .collect();

    let mut sub_entries = calculate_standings(&team_ids, &h2h_fixtures);
    apply_spa_metrics_to_standings(&mut sub_entries, spa_policy);
    apply_qta_to_standings(&mut sub_entries, qta_policy);

    let partitioned = partition_tied_groups(&sub_entries, criteria);

    if partitioned.len() == 1 && partitioned[0].len() == tied_group.len() {
        return tied_group.to_vec();
    }

    let mut resolved = Vec::with_capacity(tied_group.len());
    for group in partitioned {
        let original_entries: Vec<StandingsEntry> = group
            .iter()
            .filter_map(|e| original_map.get(&e.team_id()).copied())
            .collect();

        if original_entries.len() > 1 && original_entries.len() < tied_group.len() {
            let sub_resolved = resolve_head_to_head(
                &original_entries,
                all_fixtures,
                spa_policy,
                qta_policy,
                criteria,
            );
            resolved.extend(sub_resolved);
        } else {
            resolved.extend(original_entries);
        }
    }

    resolved
}