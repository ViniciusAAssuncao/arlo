use crate::dynamic_roster::resolve_dynamic_roster_award;
use crate::{resolve_award, AwardError, CandidateResult, ElectorateResult};
use arlo_domain::{
    AwardCandidateEvidence, AwardDefinition, AwardInstanceContext, AwardResultKind, AwardRosterSlot,
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct RosterSelection {
    pub slot: AwardRosterSlot,
    pub result: CandidateResult,
    pub candidates: Vec<CandidateResult>,
    pub electorates: Vec<ElectorateResult>,
    pub usage_score: Option<f64>,
    pub evidence_score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AwardRosterResolution {
    pub definition_id: Uuid,
    pub period_key: String,
    pub scope_id: Option<Uuid>,
    pub seed: u64,
    pub model_version: u32,
    pub selections: Vec<RosterSelection>,
}

pub fn resolve_roster_award(
    definition: &AwardDefinition,
    context: &AwardInstanceContext,
    candidates: &[AwardCandidateEvidence],
    seed: u64,
) -> Result<AwardRosterResolution, AwardError> {
    if definition.dynamic_roster.is_some() {
        return resolve_dynamic_roster_award(definition, context, candidates, seed);
    }
    if definition.result_kind != AwardResultKind::Roster || definition.roster_slots.is_empty() {
        return Err(AwardError::InvalidDefinition(
            "roster award requires configured slots".into(),
        ));
    }
    let mut slots = definition.roster_slots.clone();
    slots.sort_by_key(|slot| slot.slot_index);
    if slots
        .iter()
        .any(|slot| slot.slot_index == 0 || slot.position_code.is_empty())
        || slots
            .windows(2)
            .any(|pair| pair[0].slot_index == pair[1].slot_index)
    {
        return Err(AwardError::InvalidDefinition(
            "roster slots require distinct positive indexes and positions".into(),
        ));
    }
    let mut ranked = Vec::with_capacity(slots.len());
    for slot in slots {
        let mut slot_definition = definition.clone();
        slot_definition.result_kind = AwardResultKind::SingleWinner;
        slot_definition.eligible_positions = vec![slot.position_code.clone()];
        slot_definition.roster_slots.clear();
        if !slot.criteria.is_empty() {
            slot_definition.criteria = slot.criteria.clone();
        }
        let slot_seed = slot
            .slot_index
            .to_le_bytes()
            .iter()
            .fold(seed, |value, byte| {
                value.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
            });
        let resolution = resolve_award(&slot_definition, context, candidates, slot_seed)?;
        ranked.push((slot, resolution.candidates, resolution.electorates));
    }
    let mut selected = HashSet::new();
    let mut selections = Vec::with_capacity(ranked.len());
    for index in 0..ranked.len() {
        let (slot, results, electorates) = &ranked[index];
        let result = results
            .iter()
            .find(|result| {
                if selected.contains(&result.subject_id) {
                    return false;
                }
                selected.insert(result.subject_id);
                let feasible = remaining_slots_can_be_filled(&ranked[index + 1..], &selected);
                selected.remove(&result.subject_id);
                feasible
            })
            .cloned()
            .ok_or(AwardError::NoEligibleCandidates)?;
        selected.insert(result.subject_id);
        selections.push(RosterSelection {
            slot: slot.clone(),
            result,
            candidates: results.clone(),
            electorates: electorates.clone(),
            usage_score: None,
            evidence_score: None,
        });
    }
    Ok(AwardRosterResolution {
        definition_id: definition.id,
        period_key: context.period_key.clone(),
        scope_id: context.scope_id,
        seed,
        model_version: context.selection_model_version,
        selections,
    })
}

fn remaining_slots_can_be_filled(
    ranked: &[(AwardRosterSlot, Vec<CandidateResult>, Vec<ElectorateResult>)],
    selected: &HashSet<Uuid>,
) -> bool {
    let mut assigned = HashMap::<Uuid, usize>::new();
    for index in 0..ranked.len() {
        let mut seen = HashSet::new();
        if !assign_candidate(index, ranked, selected, &mut assigned, &mut seen) {
            return false;
        }
    }
    true
}

fn assign_candidate(
    index: usize,
    ranked: &[(AwardRosterSlot, Vec<CandidateResult>, Vec<ElectorateResult>)],
    selected: &HashSet<Uuid>,
    assigned: &mut HashMap<Uuid, usize>,
    seen: &mut HashSet<Uuid>,
) -> bool {
    for result in &ranked[index].1 {
        let id = result.subject_id;
        if selected.contains(&id) || !seen.insert(id) {
            continue;
        }
        let available = match assigned.get(&id).copied() {
            Some(previous) => assign_candidate(previous, ranked, selected, assigned, seen),
            None => true,
        };
        if available {
            assigned.insert(id, index);
            return true;
        }
    }
    false
}
