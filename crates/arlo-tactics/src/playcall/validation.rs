use crate::error::{TacticsError, TacticsResult};
use crate::lineup::TacticalLineup;
use crate::playcall::category::PlayCallCategory;
use crate::playcall::misdirection::MisdirectionLink;
use crate::playcall::route::RouteAssignment;
use arlo_domain::{Position, SlotRole};
use std::collections::HashSet;

pub fn validate_play_call(
    routes: &[RouteAssignment],
    role_overrides: &[(usize, SlotRole)],
    misdirection: Option<&MisdirectionLink>,
    category: PlayCallCategory,
    lineup: &TacticalLineup,
) -> TacticsResult<()> {
    for route in routes {
        if lineup.assignment_for_slot(route.slot_index()).is_none() {
            return Err(TacticsError::InvalidPlayCall(format!(
                "Route references slot index {} which does not exist in lineup",
                route.slot_index()
            )));
        }
    }

    for (slot_idx, _) in role_overrides {
        if lineup.assignment_for_slot(*slot_idx).is_none() {
            return Err(TacticsError::InvalidPlayCall(format!(
                "Role override references slot index {} which does not exist in lineup",
                slot_idx
            )));
        }
    }

    if let Some(m) = misdirection {
        if lineup.assignment_for_slot(m.decoy_slot_index()).is_none() {
            return Err(TacticsError::InvalidPlayCall(format!(
                "Misdirection decoy references slot index {} which does not exist in lineup",
                m.decoy_slot_index()
            )));
        }
        if lineup
            .assignment_for_slot(m.true_carrier_slot_index())
            .is_none()
        {
            return Err(TacticsError::InvalidPlayCall(format!(
                "Misdirection true carrier references slot index {} which does not exist in lineup",
                m.true_carrier_slot_index()
            )));
        }
    }

    let mut seen_route_slots = HashSet::new();
    for route in routes {
        if !seen_route_slots.insert(route.slot_index()) {
            return Err(TacticsError::InvalidPlayCall(format!(
                "Duplicate route assignment for slot index {}",
                route.slot_index()
            )));
        }
    }

    if let Some(m) = misdirection {
        if m.decoy_slot_index() == m.true_carrier_slot_index() {
            return Err(TacticsError::InvalidPlayCall(
                "Misdirection decoy and true carrier cannot be the same slot".to_string(),
            ));
        }

        let decoy_role = role_overrides
            .iter()
            .rev()
            .find(|(idx, _)| *idx == m.decoy_slot_index())
            .map(|(_, role)| *role)
            .unwrap_or_else(|| {
                lineup
                    .assignment_for_slot(m.decoy_slot_index())
                    .unwrap()
                    .slot_role()
            });

        if decoy_role != SlotRole::FalseArtrine {
            return Err(TacticsError::InvalidPlayCall(format!(
                "Misdirection decoy slot {} must resolve to FalseArtrine role, found {:?}",
                m.decoy_slot_index(),
                decoy_role
            )));
        }

        let true_carrier_pos = lineup
            .assignment_for_slot(m.true_carrier_slot_index())
            .unwrap()
            .position();

        if true_carrier_pos != Position::Artrine {
            return Err(TacticsError::InvalidPlayCall(format!(
                "Misdirection true carrier slot {} must have Artrine position, found {:?}",
                m.true_carrier_slot_index(),
                true_carrier_pos
            )));
        }
    }

    if category == PlayCallCategory::BonusPhaseConversion {
        let has_kicker = lineup.assignments().iter().any(|assignment| {
            let resolved_role = role_overrides
                .iter()
                .rev()
                .find(|(idx, _)| *idx == assignment.formation_slot_index())
                .map(|(_, role)| *role)
                .unwrap_or_else(|| assignment.slot_role());
            resolved_role == SlotRole::Kicker
        });

        if !has_kicker {
            return Err(TacticsError::InvalidPlayCall(
                "BonusPhaseConversion play calls require at least one slot with Kicker role"
                    .to_string(),
            ));
        }
    }

    Ok(())
}