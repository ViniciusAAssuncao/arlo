use super::TacticalLayout;
use crate::{SlotAssignment, TacticalLineup};
use arlo_domain::{Formation, Position};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub fn adapt_layout(
    initial: &TacticalLineup,
    current: &TacticalLayout,
    target: &TacticalLayout,
    active: impl Fn(Uuid) -> bool,
    quality: impl Fn(Uuid, usize) -> f64,
) -> Option<TacticalLayout> {
    let templates = target.lineup.assignments();
    if initial.assignments().len() != 14 || templates.len() != 14 {
        return None;
    }
    let mut assigned = HashMap::new();
    let mut used = HashSet::new();
    let mut locked = HashSet::new();
    for template in templates.iter().filter(|slot| mandatory(slot.position())) {
        let mut anchors = initial
            .assignments()
            .iter()
            .filter(|slot| slot.position() == template.position());
        let anchor = anchors.next()?.player_id();
        if anchors.next().is_some() || !used.insert(anchor) {
            return None;
        }
        assigned.insert(template.formation_slot_index(), anchor);
        locked.insert(template.formation_slot_index());
    }
    let mut previous: Vec<_> = current.lineup.assignments().iter().collect();
    previous.sort_by_key(|slot| (active(slot.player_id()), slot.formation_slot_index()));
    for old in previous {
        if used.contains(&old.player_id()) {
            continue;
        }
        let old_slot = current.formation.slots().get(old.formation_slot_index())?;
        let target_slot = templates
            .iter()
            .filter(|slot| !assigned.contains_key(&slot.formation_slot_index()))
            .filter(|slot| same_role(&target.formation, slot, old_slot, old))
            .min_by_key(|slot| slot.formation_slot_index());
        if let Some(slot) = target_slot {
            assigned.insert(slot.formation_slot_index(), old.player_id());
            used.insert(old.player_id());
            if !active(old.player_id()) {
                locked.insert(slot.formation_slot_index());
            }
        } else if !active(old.player_id()) {
            let slot = templates
                .iter()
                .filter(|slot| !assigned.contains_key(&slot.formation_slot_index()))
                .min_by_key(|slot| {
                    (
                        slot.position() != old.position(),
                        slot.formation_slot_index(),
                    )
                })?;
            assigned.insert(slot.formation_slot_index(), old.player_id());
            used.insert(old.player_id());
            locked.insert(slot.formation_slot_index());
        }
    }
    let mut remaining: Vec<_> = templates
        .iter()
        .filter(|slot| !assigned.contains_key(&slot.formation_slot_index()))
        .collect();
    remaining.sort_by_key(|slot| {
        (
            initial
                .assignments()
                .iter()
                .filter(|anchor| {
                    !used.contains(&anchor.player_id())
                        && quality(anchor.player_id(), slot.formation_slot_index()) >= 0.0
                })
                .count(),
            slot.formation_slot_index(),
        )
    });
    let anchors: Vec<_> = initial
        .assignments()
        .iter()
        .map(SlotAssignment::player_id)
        .collect();
    for slot in remaining {
        let anchor = initial
            .assignments()
            .iter()
            .filter(|anchor| !used.contains(&anchor.player_id()))
            .map(|anchor| {
                (
                    anchor.player_id(),
                    quality(anchor.player_id(), slot.formation_slot_index()),
                )
            })
            .filter(|(_, quality)| quality.is_finite() && *quality >= 0.0)
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .map(|value| value.0);
        if let Some(anchor) = anchor {
            assigned.insert(slot.formation_slot_index(), anchor);
        } else if !super::matching::fill(
            slot.formation_slot_index(),
            &anchors,
            &mut assigned,
            &locked,
            &mut HashSet::new(),
            &quality,
        ) {
            return None;
        }
        used = assigned.values().copied().collect();
    }
    let mut assignments: Vec<_> = templates
        .iter()
        .map(|slot| {
            SlotAssignment::new(
                slot.formation_slot_index(),
                slot.position(),
                assigned[&slot.formation_slot_index()],
                slot.slot_role(),
                *slot.player_instructions(),
            )
        })
        .collect();
    assignments.sort_by_key(SlotAssignment::formation_slot_index);
    Some(TacticalLayout {
        formation: target.formation.clone(),
        lineup: TacticalLineup::new(
            target.lineup.id(),
            initial.team_id(),
            target.formation.id(),
            target.lineup.name(),
            assignments,
        ),
    })
}

fn mandatory(position: Position) -> bool {
    matches!(
        position,
        Position::Artrine | Position::Passer | Position::Goalguard
    )
}

fn same_role(
    formation: &Formation,
    new: &SlotAssignment,
    old_slot: &arlo_domain::FormationSlot,
    old: &SlotAssignment,
) -> bool {
    formation
        .slots()
        .get(new.formation_slot_index())
        .is_some_and(|slot| {
            slot.offensive_position() == old_slot.offensive_position()
                && slot.defensive_position() == old_slot.defensive_position()
                && new.slot_role() == old.slot_role()
        })
}
