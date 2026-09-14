use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::league_calendar::knockout_leg_format::KnockoutLegFormat;
use crate::domain::league_calendar::qualification_pool_rule_validation::validate_qualification_pool_rule;
use crate::domain::league_calendar::schedule_block::ScheduleBlock;
use crate::domain::league_calendar::stage_entry_rule::StageEntryRule;
use crate::domain::league_calendar::stage_type::StageType;
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageDefinition {
    stage_order_index: u32,
    stage_type: StageType,
    entry_rule: StageEntryRule,
    knockout_leg_format: Option<KnockoutLegFormat>,
    schedule_blocks: Option<Vec<ScheduleBlock>>,
}

impl StageDefinition {
    pub fn new(
        stage_order_index: u32,
        stage_type: StageType,
        entry_rule: StageEntryRule,
        knockout_leg_format: Option<KnockoutLegFormat>,
        schedule_blocks: Option<Vec<ScheduleBlock>>,
    ) -> DomainResult<Self> {
        for pool in entry_rule.pools() {
            validate_qualification_pool_rule(pool)?;
        }

        match stage_type {
            StageType::KnockoutBracket => {
                if knockout_leg_format.is_none() {
                    return Err(DomainError::InvalidInvariant {
                        field: "knockout_leg_format".to_string(),
                        violation: InvariantViolation::MissingRequiredValue,
                    });
                }
                if schedule_blocks.is_some() {
                    return Err(DomainError::InvalidInvariant {
                        field: "schedule_blocks".to_string(),
                        violation: InvariantViolation::UnexpectedValue,
                    });
                }
            }
            StageType::RoundRobinTable => {
                if knockout_leg_format.is_some() {
                    return Err(DomainError::InvalidInvariant {
                        field: "knockout_leg_format".to_string(),
                        violation: InvariantViolation::UnexpectedValue,
                    });
                }
                if schedule_blocks.is_some() {
                    return Err(DomainError::InvalidInvariant {
                        field: "schedule_blocks".to_string(),
                        violation: InvariantViolation::UnexpectedValue,
                    });
                }
            }
            StageType::GroupedCompetitionTable => {
                if knockout_leg_format.is_some() {
                    return Err(DomainError::InvalidInvariant {
                        field: "knockout_leg_format".to_string(),
                        violation: InvariantViolation::UnexpectedValue,
                    });
                }
                match &schedule_blocks {
                    None => {
                        return Err(DomainError::InvalidInvariant {
                            field: "schedule_blocks".to_string(),
                            violation: InvariantViolation::MissingRequiredValue,
                        });
                    }
                    Some(blocks) => {
                        if blocks.is_empty() {
                            return Err(DomainError::InvalidInvariant {
                                field: "schedule_blocks".to_string(),
                                violation: InvariantViolation::Empty,
                            });
                        }
                    }
                }
            }
        }

        Ok(Self {
            stage_order_index,
            stage_type,
            entry_rule,
            knockout_leg_format,
            schedule_blocks,
        })
    }

    pub fn stage_order_index(&self) -> u32 {
        self.stage_order_index
    }

    pub fn stage_type(&self) -> StageType {
        self.stage_type
    }

    pub fn entry_rule(&self) -> &StageEntryRule {
        &self.entry_rule
    }

    pub fn knockout_leg_format(&self) -> Option<KnockoutLegFormat> {
        self.knockout_leg_format
    }

    pub fn schedule_blocks(&self) -> Option<&[ScheduleBlock]> {
        self.schedule_blocks.as_deref()
    }
}
