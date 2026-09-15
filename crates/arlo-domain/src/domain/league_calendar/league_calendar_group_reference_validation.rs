use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::league_calendar::competition_group::CompetitionGroup;
use crate::domain::league_calendar::schedule_block::ScheduleBlock;
use crate::domain::league_calendar::stage_definition::StageDefinition;
use crate::domain::league_calendar::team_pool_ref::TeamPoolRef;
use crate::domain::validation::{validate_integer_range, validate_no_duplicate_keys};
use crate::error::{DomainError, DomainResult};
use std::collections::HashSet;
use uuid::Uuid;

pub fn validate_no_duplicate_team_across_groups(groups: &[CompetitionGroup]) -> DomainResult<()> {
    let mut seen = HashSet::new();
    for group in groups {
        for &team_id in group.team_ids() {
            if !seen.insert(team_id) {
                return Err(DomainError::InvalidInvariant {
                    field: "groups".to_string(),
                    violation: InvariantViolation::DuplicateKey {
                        key_name: "team_id".to_string(),
                    },
                });
            }
        }
    }
    Ok(())
}

pub fn validate_group_order_indices_sequential(groups: &[CompetitionGroup]) -> DomainResult<()> {
    if groups.is_empty() {
        return Ok(());
    }

    validate_no_duplicate_keys(
        groups,
        |g| g.order_index(),
        "groups",
        "order_index",
    )?;

    let mut sorted_indices: Vec<u32> = groups.iter().map(|g| g.order_index()).collect();
    sorted_indices.sort_unstable();

    for (expected_index, &actual_index) in sorted_indices.iter().enumerate() {
        if actual_index != expected_index as u32 {
            return Err(DomainError::InvalidInvariant {
                field: "groups".to_string(),
                violation: InvariantViolation::OutOfIntegerRange {
                    min: 0,
                    max: (groups.len() - 1) as i32,
                },
            });
        }
    }

    Ok(())
}

pub fn validate_schedule_block_group_references(
    stages: &[StageDefinition],
    groups: &[CompetitionGroup],
) -> DomainResult<()> {
    let group_ids: HashSet<Uuid> = groups.iter().map(|g| g.id()).collect();

    for stage in stages {
        if let Some(blocks) = stage.schedule_blocks() {
            for block in blocks {
                match block {
                    ScheduleBlock::GroupRoundRobin { group_id, .. } => {
                        if !group_ids.contains(group_id) {
                            return Err(DomainError::InvalidInvariant {
                                field: "schedule_blocks.group_id".to_string(),
                                violation: InvariantViolation::UnexpectedValue,
                            });
                        }
                    }
                    ScheduleBlock::CrossGroupPairing {
                        group_a_id,
                        group_b_id,
                        ..
                    } => {
                        if group_a_id == group_b_id {
                            return Err(DomainError::InvalidInvariant {
                                field: "schedule_blocks.cross_group_pairing".to_string(),
                                violation: InvariantViolation::SelfReference,
                            });
                        }
                        if !group_ids.contains(group_a_id) {
                            return Err(DomainError::InvalidInvariant {
                                field: "schedule_blocks.group_a_id".to_string(),
                                violation: InvariantViolation::UnexpectedValue,
                            });
                        }
                        if !group_ids.contains(group_b_id) {
                            return Err(DomainError::InvalidInvariant {
                                field: "schedule_blocks.group_b_id".to_string(),
                                violation: InvariantViolation::UnexpectedValue,
                            });
                        }
                    }
                    ScheduleBlock::RandomPoolRounds { pool, rounds_count } => {
                        validate_integer_range(
                            *rounds_count as i32,
                            1,
                            i32::MAX,
                            "schedule_blocks.rounds_count",
                        )?;
                        if let TeamPoolRef::SpecificGroups(pool_group_ids) = pool {
                            if pool_group_ids.is_empty() {
                                return Err(DomainError::InvalidInvariant {
                                    field: "schedule_blocks.pool".to_string(),
                                    violation: InvariantViolation::Empty,
                                });
                            }
                            for gid in pool_group_ids {
                                if !group_ids.contains(gid) {
                                    return Err(DomainError::InvalidInvariant {
                                        field: "schedule_blocks.pool.group_id".to_string(),
                                        violation: InvariantViolation::UnexpectedValue,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn validate_entry_rule_group_references(
    stages: &[StageDefinition],
    groups: &[CompetitionGroup],
) -> DomainResult<()> {
    for stage in stages {
        if stage.entry_rule().requires_groups() && groups.is_empty() {
            return Err(DomainError::InvalidInvariant {
                field: "groups".to_string(),
                violation: InvariantViolation::Empty,
            });
        }
    }
    Ok(())
}
