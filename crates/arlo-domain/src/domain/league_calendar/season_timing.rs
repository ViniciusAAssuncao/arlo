use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::validation::{validate_integer_range, validate_no_duplicate_keys};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonTiming {
    start_month_order_index: u32,
    start_day_of_month: u32,
    duration_weeks: u32,
    allowed_weekdays: Vec<u32>,
}

impl SeasonTiming {
    pub fn new(
        start_month_order_index: u32,
        start_day_of_month: u32,
        duration_weeks: u32,
        allowed_weekdays: Vec<u32>,
    ) -> DomainResult<Self> {
        validate_integer_range(start_day_of_month as i32, 1, 31, "start_day_of_month")?;
        validate_integer_range(duration_weeks as i32, 1, 104, "duration_weeks")?;

        if allowed_weekdays.is_empty() {
            return Err(DomainError::InvalidInvariant {
                field: "allowed_weekdays".to_string(),
                violation: InvariantViolation::Empty,
            });
        }

        for &weekday in &allowed_weekdays {
            validate_integer_range(weekday as i32, 0, 31, "allowed_weekdays")?;
        }

        validate_no_duplicate_keys(&allowed_weekdays, |w| *w, "allowed_weekdays", "weekday")?;

        Ok(Self {
            start_month_order_index,
            start_day_of_month,
            duration_weeks,
            allowed_weekdays,
        })
    }

    pub fn start_month_order_index(&self) -> u32 {
        self.start_month_order_index
    }

    pub fn start_day_of_month(&self) -> u32 {
        self.start_day_of_month
    }

    pub fn duration_weeks(&self) -> u32 {
        self.duration_weeks
    }

    pub fn allowed_weekdays(&self) -> &[u32] {
        &self.allowed_weekdays
    }
}
