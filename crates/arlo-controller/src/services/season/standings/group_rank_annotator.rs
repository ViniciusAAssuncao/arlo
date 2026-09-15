use crate::domain::season::{GroupRankedStandingsEntry, StandingsEntry};
use arlo_domain::{build_team_to_group_index, CompetitionGroup};
use std::collections::HashMap;
use uuid::Uuid;

pub fn annotate_group_ranks(
    sorted_entries: &[StandingsEntry],
    groups: &[CompetitionGroup],
) -> Vec<GroupRankedStandingsEntry> {
    let team_to_group = build_team_to_group_index(groups);
    let mut group_counters: HashMap<Uuid, u32> = HashMap::with_capacity(groups.len());
    let mut result = Vec::with_capacity(sorted_entries.len());

    for entry in sorted_entries {
        if let Some(&group_id) = team_to_group.get(&entry.team_id()) {
            let rank = group_counters.entry(group_id).or_insert(0);
            result.push(GroupRankedStandingsEntry::new(*entry, group_id, *rank));
            *rank += 1;
        }
    }

    result
}