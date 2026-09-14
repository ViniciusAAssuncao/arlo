use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::league_calendar::knockout_leg_format::KnockoutLegFormat;
use crate::domain::league_calendar::stage_entry_rule::StageEntryRule;
use crate::domain::league_calendar::stage_type::StageType;
use crate::domain::validation::validate_integer_range;
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StageDefinition {
    stage_order_index: u32,
    stage_type: StageType,
    entry_rule: StageEntryRule,
    knockout_leg_format: Option<KnockoutLegFormat>,
}

impl StageDefinition {
    pub fn new(
        stage_order_index: u32,
        stage_type: StageType,
        entry_rule: StageEntryRule,
        knockout_leg_format: Option<KnockoutLegFormat>,
    ) -> DomainResult<Self> {
        match entry_rule {
            StageEntryRule::TopN { count } => {
                validate_integer_range(count as i32, 1, i32::MAX, "entry_rule.count")?;
            }
            StageEntryRule::BottomN { count } => {
                validate_integer_range(count as i32, 1, i32::MAX, "entry_rule.count")?;
            }
            StageEntryRule::AllTeams => {}
        }

        match stage_type {
            StageType::KnockoutBracket => {
                if knockout_leg_format.is_none() {
                    return Err(DomainError::InvalidInvariant {
                        field: "knockout_leg_format".to_string(),
                        violation: InvariantViolation::MissingRequiredValue,
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
            }
        }

        Ok(Self {
            stage_order_index,
            stage_type,
            entry_rule,
            knockout_leg_format,
        })
    }

    pub fn stage_order_index(&self) -> u32 {
        self.stage_order_index
    }

    pub fn stage_type(&self) -> StageType {
        self.stage_type
    }

    pub fn entry_rule(&self) -> StageEntryRule {
        self.entry_rule
    }

    pub fn knockout_leg_format(&self) -> Option<KnockoutLegFormat> {
        self.knockout_leg_format
    }
}