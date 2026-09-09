use crate::lineup_runtime::fit_calculator::calculate_fit_for_position;
use arlo_domain::{Formation, Player};
use uuid::Uuid;

pub fn score_formation(
    formation: &Formation,
    roster: &[Player],
    preferred_formation_ids: &[Uuid],
) -> f64 {
    let slots = formation.slots();
    if slots.is_empty() {
        return 0.0;
    }

    let mut total_coverage = 0.0;
    for slot in slots {
        let max_fit = roster
            .iter()
            .map(|p| calculate_fit_for_position(p, slot.position()).efficiency_multiplier())
            .fold(0.0_f64, f64::max);
        total_coverage += max_fit;
    }
    let roster_coverage = total_coverage / (slots.len() as f64);

    let pref_bonus = if preferred_formation_ids.is_empty() {
        0.0
    } else if let Some(index) = preferred_formation_ids
        .iter()
        .position(|id| *id == formation.id())
    {
        1.0 - (index as f64) / (preferred_formation_ids.len() as f64)
    } else {
        0.0
    };

    roster_coverage + pref_bonus
}

pub fn select_best_formation<'a>(
    formations: &'a [Formation],
    roster: &[Player],
    preferred_formation_ids: &[Uuid],
) -> Option<&'a Formation> {
    formations.iter().max_by(|a, b| {
        let score_a = score_formation(a, roster, preferred_formation_ids);
        let score_b = score_formation(b, roster, preferred_formation_ids);
        score_a
            .partial_cmp(&score_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}