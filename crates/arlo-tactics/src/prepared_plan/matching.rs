use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub(super) fn fill(
    slot: usize,
    anchors: &[Uuid],
    assigned: &mut HashMap<usize, Uuid>,
    locked: &HashSet<usize>,
    visited: &mut HashSet<usize>,
    quality: &impl Fn(Uuid, usize) -> f64,
) -> bool {
    if locked.contains(&slot) || !visited.insert(slot) {
        return false;
    }
    let mut candidates: Vec<_> = anchors
        .iter()
        .map(|id| (*id, quality(*id, slot)))
        .filter(|(_, value)| value.is_finite() && *value >= 0.0)
        .collect();
    candidates.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    for (anchor, _) in candidates {
        let occupied = assigned
            .iter()
            .find_map(|(index, id)| (*id == anchor).then_some(*index));
        if occupied.is_none_or(|old| fill(old, anchors, assigned, locked, visited, quality)) {
            assigned.insert(slot, anchor);
            return true;
        }
    }
    false
}
