use crate::roster::{AwardRosterResolution, RosterSelection};
use crate::{resolve_award, AwardError, CandidateResult};
use arlo_domain::{
    AwardCandidateEvidence, AwardDefinition, AwardDynamicRosterPolicy, AwardInstanceContext,
    AwardResultKind, AwardRosterSlot,
};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

struct PositionPool {
    code: String,
    usage_seconds: f64,
    qualified: Vec<Uuid>,
    scores: BTreeMap<Uuid, (f64, f64)>,
    quality: f64,
    weight: f64,
    allocated: u32,
    remainder: f64,
}

pub(super) fn resolve_dynamic_roster_award(
    definition: &AwardDefinition,
    context: &AwardInstanceContext,
    candidates: &[AwardCandidateEvidence],
    seed: u64,
) -> Result<AwardRosterResolution, AwardError> {
    let policy = definition
        .dynamic_roster
        .as_ref()
        .ok_or_else(|| AwardError::InvalidDefinition("dynamic roster policy is missing".into()))?;
    validate_policy(definition, policy)?;
    let mut pools = discover_positions(definition, context, candidates, seed, policy)?;
    loop {
        let preferred = preferred_positions(candidates, &pools, policy);
        let previous_len = pools.len();
        pools.retain(|pool| {
            pool.qualified
                .iter()
                .filter(|id| {
                    preferred
                        .get(id)
                        .is_some_and(|position| position == &pool.code)
                })
                .count()
                >= policy.minimum_position_candidates as usize
        });
        if pools.len() == previous_len {
            break;
        }
    }
    let preferred = preferred_positions(candidates, &pools, policy);
    for pool in &mut pools {
        pool.qualified.retain(|id| {
            preferred
                .get(id)
                .is_some_and(|position| position == &pool.code)
        });
        pool.quality = pool
            .qualified
            .iter()
            .filter_map(|id| pool.scores.get(id).map(|score| score.0))
            .take(policy.minimum_position_candidates as usize)
            .sum::<f64>()
            / f64::from(policy.minimum_position_candidates);
    }
    pools.retain(|pool| pool.qualified.len() >= policy.minimum_position_candidates as usize);
    if pools.is_empty() {
        return Err(AwardError::NoEligibleCandidates);
    }
    let total_usage: f64 = pools.iter().map(|pool| pool.usage_seconds).sum();
    let max_depth = pools
        .iter()
        .map(|pool| pool.qualified.len())
        .max()
        .unwrap_or(1) as f64;
    for pool in &mut pools {
        let depth = (1.0 + pool.qualified.len() as f64).ln() / (1.0 + max_depth).ln();
        pool.weight = (pool.usage_seconds / total_usage) * pool.quality * depth;
    }
    allocate_slots(&mut pools, policy.slot_count)?;
    let mut selections = Vec::with_capacity(policy.slot_count as usize);
    let mut chosen = HashSet::new();
    for pool in pools.iter().filter(|pool| pool.allocated > 0) {
        let eligible: Vec<_> = candidates
            .iter()
            .filter(|candidate| pool.qualified.contains(&candidate.subject_id))
            .cloned()
            .collect();
        for _ in 0..pool.allocated {
            let slot_index = selections.len() as u32 + 1;
            let slot = AwardRosterSlot {
                slot_index,
                position_code: pool.code.clone(),
                selection_group: None,
                slot_role: None,
                criteria: position_criteria(definition, &pool.code).to_vec(),
            };
            let mut slot_definition = definition.clone();
            slot_definition.result_kind = AwardResultKind::SingleWinner;
            slot_definition.dynamic_roster = None;
            slot_definition.roster_slots.clear();
            slot_definition.eligible_positions = vec![pool.code.clone()];
            slot_definition.nomination_limit = None;
            if !slot.criteria.is_empty() {
                slot_definition.criteria = slot.criteria.clone();
            }
            let slot_seed = slot_index.to_le_bytes().iter().fold(seed, |value, byte| {
                value.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
            });
            let resolution = resolve_award(&slot_definition, context, &eligible, slot_seed)?;
            let result = resolution
                .candidates
                .iter()
                .find(|result| !chosen.contains(&result.subject_id))
                .cloned()
                .ok_or(AwardError::NoEligibleCandidates)?;
            chosen.insert(result.subject_id);
            selections.push(RosterSelection {
                slot,
                result,
                candidates: resolution.candidates,
                electorates: resolution.electorates,
                usage_score: Some(pool.usage_seconds / total_usage),
                evidence_score: Some(pool.quality),
            });
        }
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

fn validate_policy(
    definition: &AwardDefinition,
    policy: &AwardDynamicRosterPolicy,
) -> Result<(), AwardError> {
    if definition.result_kind != AwardResultKind::Roster
        || !definition.roster_slots.is_empty()
        || policy.slot_count == 0
        || policy.minimum_position_candidates == 0
        || !policy.minimum_position_seconds.is_finite()
        || policy.minimum_position_seconds < 0.0
        || !policy.minimum_utility.is_finite()
        || policy.minimum_utility < 0.0
    {
        return Err(AwardError::InvalidDefinition(
            "invalid dynamic roster policy".into(),
        ));
    }
    Ok(())
}

fn discover_positions(
    definition: &AwardDefinition,
    context: &AwardInstanceContext,
    candidates: &[AwardCandidateEvidence],
    seed: u64,
    policy: &AwardDynamicRosterPolicy,
) -> Result<Vec<PositionPool>, AwardError> {
    let mut usage = BTreeMap::<String, f64>::new();
    for candidate in candidates {
        if candidate.matches_played < definition.minimum_matches {
            continue;
        }
        for position in &candidate.position_usage {
            if position.seconds_played >= policy.minimum_position_seconds
                && (definition.eligible_positions.is_empty()
                    || definition
                        .eligible_positions
                        .contains(&position.position_code))
            {
                *usage.entry(position.position_code.clone()).or_default() +=
                    position.seconds_played;
            }
        }
    }
    let mut pools = Vec::new();
    for (code, usage_seconds) in usage {
        let position_candidates: Vec<_> = candidates
            .iter()
            .filter(|candidate| {
                candidate.position_usage.iter().any(|item| {
                    item.position_code == code
                        && item.seconds_played >= policy.minimum_position_seconds
                })
            })
            .cloned()
            .collect();
        let mut scoped = definition.clone();
        scoped.result_kind = AwardResultKind::SingleWinner;
        scoped.dynamic_roster = None;
        scoped.roster_slots.clear();
        scoped.eligible_positions = vec![code.clone()];
        scoped.nomination_limit = None;
        if !position_criteria(definition, &code).is_empty() {
            scoped.criteria = position_criteria(definition, &code).to_vec();
        }
        let resolution = match resolve_award(&scoped, context, &position_candidates, seed) {
            Ok(value) => value,
            Err(AwardError::NoEligibleCandidates) => continue,
            Err(error) => return Err(error),
        };
        let mut qualified: Vec<CandidateResult> = resolution
            .candidates
            .into_iter()
            .filter(|item| item.utility >= policy.minimum_utility)
            .collect();
        if qualified.len() < policy.minimum_position_candidates as usize {
            continue;
        }
        qualified.sort_by(|left, right| {
            right
                .utility
                .total_cmp(&left.utility)
                .then_with(|| left.subject_id.cmp(&right.subject_id))
        });
        let count = policy.minimum_position_candidates as usize;
        let quality = qualified
            .iter()
            .take(count)
            .map(|item| item.utility)
            .sum::<f64>()
            / count as f64;
        let scores = qualified
            .iter()
            .map(|item| (item.subject_id, (item.utility, item.selection_score)))
            .collect();
        pools.push(PositionPool {
            code,
            usage_seconds,
            qualified: qualified.into_iter().map(|item| item.subject_id).collect(),
            scores,
            quality,
            weight: 0.0,
            allocated: 0,
            remainder: 0.0,
        });
    }
    Ok(pools)
}

fn position_criteria<'a>(
    definition: &'a AwardDefinition,
    code: &str,
) -> &'a [arlo_domain::AwardCriterion] {
    definition
        .dynamic_position_profiles
        .iter()
        .find(|profile| profile.position_code == code)
        .map(|profile| profile.criteria.as_slice())
        .unwrap_or(&[])
}

fn preferred_positions(
    candidates: &[AwardCandidateEvidence],
    pools: &[PositionPool],
    policy: &AwardDynamicRosterPolicy,
) -> BTreeMap<Uuid, String> {
    let mut preferred = BTreeMap::new();
    for candidate in candidates {
        let best = candidate
            .position_usage
            .iter()
            .filter(|usage| {
                usage.seconds_played >= policy.minimum_position_seconds
                    && pools.iter().any(|pool| {
                        pool.code == usage.position_code
                            && pool.qualified.contains(&candidate.subject_id)
                    })
            })
            .max_by(|left, right| {
                left.seconds_played
                    .total_cmp(&right.seconds_played)
                    .then_with(|| left.proficiency.cmp(&right.proficiency))
                    .then_with(|| {
                        let left_score = pools
                            .iter()
                            .find(|pool| pool.code == left.position_code)
                            .and_then(|pool| pool.scores.get(&candidate.subject_id))
                            .map(|score| score.1)
                            .unwrap_or(0.0);
                        let right_score = pools
                            .iter()
                            .find(|pool| pool.code == right.position_code)
                            .and_then(|pool| pool.scores.get(&candidate.subject_id))
                            .map(|score| score.1)
                            .unwrap_or(0.0);
                        left_score.total_cmp(&right_score)
                    })
                    .then_with(|| right.position_code.cmp(&left.position_code))
            });
        if let Some(usage) = best {
            preferred.insert(candidate.subject_id, usage.position_code.clone());
        }
    }
    preferred
}

fn allocate_slots(pools: &mut [PositionPool], slot_count: u32) -> Result<(), AwardError> {
    if pools.iter().map(|pool| pool.qualified.len()).sum::<usize>() < slot_count as usize {
        return Err(AwardError::NoEligibleCandidates);
    }
    let total_weight: f64 = pools.iter().map(|pool| pool.weight).sum();
    if total_weight <= 0.0 || !total_weight.is_finite() {
        return Err(AwardError::NoEligibleCandidates);
    }
    let mut assigned = 0;
    for pool in pools.iter_mut() {
        let quota = pool.weight / total_weight * f64::from(slot_count);
        pool.allocated = (quota.floor() as u32).min(pool.qualified.len() as u32);
        pool.remainder = quota.fract();
        assigned += pool.allocated;
    }
    while assigned < slot_count {
        let next = pools
            .iter()
            .enumerate()
            .filter(|(_, pool)| pool.allocated < pool.qualified.len() as u32)
            .max_by(|(_, left), (_, right)| {
                left.remainder
                    .total_cmp(&right.remainder)
                    .then_with(|| right.code.cmp(&left.code))
            })
            .map(|(index, _)| index)
            .ok_or(AwardError::NoEligibleCandidates)?;
        pools[next].allocated += 1;
        pools[next].remainder = 0.0;
        assigned += 1;
    }
    Ok(())
}
