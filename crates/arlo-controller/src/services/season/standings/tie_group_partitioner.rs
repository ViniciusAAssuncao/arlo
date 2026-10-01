use crate::domain::season::StandingsEntry;
use crate::services::season::standings::tie_break_resolver::{
    are_entries_tied_simple, sort_by_simple_criteria,
};
use arlo_domain::TieBreakCriterion;

pub fn partition_tied_groups(
    entries: &[StandingsEntry],
    criteria: &[TieBreakCriterion],
) -> Vec<Vec<StandingsEntry>> {
    if entries.is_empty() {
        return Vec::new();
    }

    let mut sorted = entries.to_vec();
    sort_by_simple_criteria(&mut sorted, criteria);

    let mut groups: Vec<Vec<StandingsEntry>> = Vec::new();
    let mut current_group: Vec<StandingsEntry> = Vec::new();

    for entry in sorted {
        if let Some(last) = current_group.last() {
            if are_entries_tied_simple(last, &entry, criteria) {
                current_group.push(entry);
            } else {
                groups.push(current_group);
                current_group = vec![entry];
            }
        } else {
            current_group.push(entry);
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}
