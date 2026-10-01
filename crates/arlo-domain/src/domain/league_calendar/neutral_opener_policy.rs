use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::league_calendar::neutral_opener_selection_strategy::NeutralOpenerSelectionStrategy;
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NeutralOpenerPolicy {
    enabled: bool,
    selection_strategy: Option<NeutralOpenerSelectionStrategy>,
}

impl NeutralOpenerPolicy {
    pub fn new(
        enabled: bool,
        selection_strategy: Option<NeutralOpenerSelectionStrategy>,
    ) -> DomainResult<Self> {
        if enabled && selection_strategy.is_none() {
            return Err(DomainError::InvalidInvariant {
                field: "selection_strategy".to_string(),
                violation: InvariantViolation::MissingRequiredValue,
            });
        }

        if !enabled && selection_strategy.is_some() {
            return Err(DomainError::InvalidInvariant {
                field: "selection_strategy".to_string(),
                violation: InvariantViolation::UnexpectedValue,
            });
        }

        Ok(Self {
            enabled,
            selection_strategy,
        })
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            selection_strategy: None,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn selection_strategy(&self) -> Option<NeutralOpenerSelectionStrategy> {
        self.selection_strategy
    }
}
