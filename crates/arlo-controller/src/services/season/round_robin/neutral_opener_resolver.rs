use crate::services::season::round_robin::circle_method_generator::RoundRobinMatch;
use arlo_domain::{NeutralOpenerPolicy, NeutralOpenerSelectionStrategy};
use rand::seq::SliceRandom;

pub fn resolve_neutral_opener(matches: &mut [RoundRobinMatch], policy: &NeutralOpenerPolicy) {
    if !policy.enabled() {
        return;
    }

    match policy.selection_strategy() {
        Some(NeutralOpenerSelectionStrategy::RandomSingleFixture) => {
            let opener_indices: Vec<usize> = matches
                .iter()
                .enumerate()
                .filter(|(_, m)| m.round_index == 0)
                .map(|(idx, _)| idx)
                .collect();

            let target_indices = if !opener_indices.is_empty() {
                opener_indices
            } else {
                (0..matches.len()).collect()
            };

            let mut rng = rand::thread_rng();
            if let Some(&chosen_idx) = target_indices.choose(&mut rng) {
                matches[chosen_idx].is_neutral_venue = true;
            }
        }
        None => {}
    }
}
