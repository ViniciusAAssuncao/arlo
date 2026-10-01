use crate::domain::calendar::calendar_month::CalendarMonthDefinition;
use crate::domain::calendar::calendar_week_day::CalendarWeekDayDefinition;
use crate::domain::calendar::intercalation_placement::IntercalationPlacement;
use crate::domain::calendar::intercalation_rule::IntercalationRule;
use crate::error::{ControllerError, ControllerResult};

pub fn validate_calendar_system(
    months: &[CalendarMonthDefinition],
    week_days: &[CalendarWeekDayDefinition],
    intercalation_rule: &IntercalationRule,
) -> ControllerResult<()> {
    validate_months(months)?;
    validate_week_days(week_days)?;
    validate_intercalation_rule(intercalation_rule, months)?;
    Ok(())
}

pub fn validate_months(months: &[CalendarMonthDefinition]) -> ControllerResult<()> {
    if months.is_empty() {
        return Err(ControllerError::Validation(
            "Month list must not be empty".to_string(),
        ));
    }

    for (expected_index, month) in months.iter().enumerate() {
        if month.order_index() != expected_index as u32 {
            return Err(ControllerError::Validation(format!(
                "Month order_index must be sequential starting from 0, expected {} but got {}",
                expected_index,
                month.order_index()
            )));
        }

        if month.day_count() == 0 {
            return Err(ControllerError::Validation(format!(
                "Month '{}' must have day_count greater than 0",
                month.name()
            )));
        }
    }

    Ok(())
}

pub fn validate_week_days(week_days: &[CalendarWeekDayDefinition]) -> ControllerResult<()> {
    if week_days.is_empty() {
        return Err(ControllerError::Validation(
            "Week day list must not be empty".to_string(),
        ));
    }

    for (expected_index, week_day) in week_days.iter().enumerate() {
        if week_day.order_index() != expected_index as u32 {
            return Err(ControllerError::Validation(format!(
                "Week day order_index must be sequential starting from 0, expected {} but got {}",
                expected_index,
                week_day.order_index()
            )));
        }
    }

    Ok(())
}

pub fn validate_intercalation_rule(
    rule: &IntercalationRule,
    months: &[CalendarMonthDefinition],
) -> ControllerResult<()> {
    if rule.cycle_length_years() <= 0 {
        return Err(ControllerError::Validation(
            "cycle_length_years must be greater than 0".to_string(),
        ));
    }

    if rule.leap_units_per_cycle() < 0 {
        return Err(ControllerError::Validation(
            "leap_units_per_cycle must be greater than or equal to 0".to_string(),
        ));
    }

    if let IntercalationPlacement::AppendToMonth { month_order_index } = rule.placement() {
        let exists = months.iter().any(|m| m.order_index() == month_order_index);
        if !exists {
            return Err(ControllerError::Validation(format!(
                "Referenced month_order_index {} in intercalation placement does not exist in months",
                month_order_index
            )));
        }
    }

    Ok(())
}
