use crate::domain::calendar::intercalation_placement::IntercalationPlacement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntercalationRule {
    leap_units_per_cycle: i64,
    cycle_length_years: i64,
    cycle_reference_year: i64,
    days_per_occurrence: u32,
    placement: IntercalationPlacement,
    disrupts_week_cycle: bool,
}

impl IntercalationRule {
    pub fn new(
        leap_units_per_cycle: i64,
        cycle_length_years: i64,
        cycle_reference_year: i64,
        days_per_occurrence: u32,
        placement: IntercalationPlacement,
        disrupts_week_cycle: bool,
    ) -> Self {
        Self {
            leap_units_per_cycle,
            cycle_length_years,
            cycle_reference_year,
            days_per_occurrence,
            placement,
            disrupts_week_cycle,
        }
    }

    pub fn leap_units_per_cycle(&self) -> i64 {
        self.leap_units_per_cycle
    }

    pub fn cycle_length_years(&self) -> i64 {
        self.cycle_length_years
    }

    pub fn cycle_reference_year(&self) -> i64 {
        self.cycle_reference_year
    }

    pub fn days_per_occurrence(&self) -> u32 {
        self.days_per_occurrence
    }

    pub fn placement(&self) -> IntercalationPlacement {
        self.placement
    }

    pub fn disrupts_week_cycle(&self) -> bool {
        self.disrupts_week_cycle
    }
}
