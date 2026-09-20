use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnnualBlackoutWindow {
    start_month_order_index: u32,
    start_day_of_month: u32,
    end_month_order_index: u32,
    end_day_of_month: u32,
    end_year_offset: u32,
}

impl AnnualBlackoutWindow {
    pub fn new(
        start_month_order_index: u32,
        start_day_of_month: u32,
        end_month_order_index: u32,
        end_day_of_month: u32,
        end_year_offset: u32,
    ) -> DomainResult<Self> {
        validate_integer_range(start_day_of_month as i32, 1, 31, "start_day_of_month")?;
        validate_integer_range(end_day_of_month as i32, 1, 31, "end_day_of_month")?;
        validate_integer_range(end_year_offset as i32, 0, 5, "end_year_offset")?;

        Ok(Self {
            start_month_order_index,
            start_day_of_month,
            end_month_order_index,
            end_day_of_month,
            end_year_offset,
        })
    }

    pub fn start_month_order_index(&self) -> u32 {
        self.start_month_order_index
    }

    pub fn start_day_of_month(&self) -> u32 {
        self.start_day_of_month
    }

    pub fn end_month_order_index(&self) -> u32 {
        self.end_month_order_index
    }

    pub fn end_day_of_month(&self) -> u32 {
        self.end_day_of_month
    }

    pub fn end_year_offset(&self) -> u32 {
        self.end_year_offset
    }
}
