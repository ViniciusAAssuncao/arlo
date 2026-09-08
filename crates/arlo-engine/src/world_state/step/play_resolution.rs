use crate::lineup_runtime::Lineup;
use arlo_domain::SlotRole;
use arlo_tactics::{DecisionEmphasis, PlayCall, PlayCallCategory, RouteAssignment};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_role_index_for_play(
    base: &HashMap<Uuid, SlotRole>,
    lineup: &Lineup,
    overrides: &[(usize, SlotRole)],
) -> HashMap<Uuid, SlotRole> {
    let mut merged = base.clone();
    for &(slot_index, role) in overrides {
        if let Some(player) = lineup.player_at_slot_index(slot_index) {
            merged.insert(player.id(), role);
        }
    }
    merged
}

pub fn resolve_route_index_for_play(
    lineup: &Lineup,
    routes: &[RouteAssignment],
) -> HashMap<Uuid, RouteAssignment> {
    let mut map = HashMap::with_capacity(routes.len());
    for route in routes {
        if let Some(player) = lineup.player_at_slot_index(route.slot_index()) {
            map.insert(player.id(), *route);
        }
    }
    map
}

pub fn resolve_decision_emphasis_for_play(play_call: Option<&PlayCall>) -> DecisionEmphasis {
    play_call
        .map(|pc| *pc.decision_emphasis())
        .unwrap_or_default()
}

pub fn expected_category_for_phase(is_bonus_phase: bool) -> PlayCallCategory {
    if is_bonus_phase {
        PlayCallCategory::BonusPhaseConversion
    } else {
        PlayCallCategory::OpenPlay
    }
}