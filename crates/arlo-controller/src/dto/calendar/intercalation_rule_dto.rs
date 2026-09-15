use crate::domain::calendar::{IntercalationPlacement, IntercalationRule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum IntercalationPlacementDto {
    BeforeFirstMonth,
    AfterLastMonth,
    AppendToMonth { month_order_index: u32 },
}

impl From<&IntercalationPlacement> for IntercalationPlacementDto {
    fn from(placement: &IntercalationPlacement) -> Self {
        match placement {
            IntercalationPlacement::BeforeFirstMonth => Self::BeforeFirstMonth,
            IntercalationPlacement::AfterLastMonth => Self::AfterLastMonth,
            IntercalationPlacement::AppendToMonth { month_order_index } => {
                Self::AppendToMonth {
                    month_order_index: *month_order_index,
                }
            }
        }
    }
}

impl From<IntercalationPlacement> for IntercalationPlacementDto {
    fn from(placement: IntercalationPlacement) -> Self {
        Self::from(&placement)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntercalationRuleDto {
    pub leap_units_per_cycle: i64,
    pub cycle_length_years: i64,
    pub cycle_reference_year: i64,
    pub days_per_occurrence: u32,
    pub placement: IntercalationPlacementDto,
    pub disrupts_week_cycle: bool,
}

impl IntercalationRuleDto {
    pub fn new(
        leap_units_per_cycle: i64,
        cycle_length_years: i64,
        cycle_reference_year: i64,
        days_per_occurrence: u32,
        placement: IntercalationPlacementDto,
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
}

impl From<&IntercalationRule> for IntercalationRuleDto {
    fn from(rule: &IntercalationRule) -> Self {
        Self {
            leap_units_per_cycle: rule.leap_units_per_cycle(),
            cycle_length_years: rule.cycle_length_years(),
            cycle_reference_year: rule.cycle_reference_year(),
            days_per_occurrence: rule.days_per_occurrence(),
            placement: IntercalationPlacementDto::from(&rule.placement()),
            disrupts_week_cycle: rule.disrupts_week_cycle(),
        }
    }
}

impl From<IntercalationRule> for IntercalationRuleDto {
    fn from(rule: IntercalationRule) -> Self {
        Self::from(&rule)
    }
}