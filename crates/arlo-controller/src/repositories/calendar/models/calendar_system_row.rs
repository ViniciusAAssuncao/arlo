use crate::domain::calendar::{
    CalendarMonthDefinition, CalendarSystem, CalendarWeekDayDefinition, IntercalationRule,
};
use crate::error::ControllerResult;
use crate::repositories::calendar::models::intercalation_placement_code::parse_intercalation_placement;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CalendarSystemRow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub leap_units_per_cycle: i64,
    pub cycle_length_years: i64,
    pub cycle_reference_year: i64,
    pub days_per_occurrence: i32,
    pub intercalation_placement_kind: String,
    pub intercalation_placement_month_order_index: Option<i32>,
    pub intercalation_disrupts_week_cycle: bool,
    pub created_at_unix_seconds: i64,
}

impl CalendarSystemRow {
    pub fn to_domain(
        &self,
        months: Vec<CalendarMonthDefinition>,
        week_days: Vec<CalendarWeekDayDefinition>,
    ) -> ControllerResult<CalendarSystem> {
        let id = Uuid::parse_str(&self.id)?;
        let placement = parse_intercalation_placement(
            &self.intercalation_placement_kind,
            self.intercalation_placement_month_order_index,
        )?;
        let intercalation_rule = IntercalationRule::new(
            self.leap_units_per_cycle,
            self.cycle_length_years,
            self.cycle_reference_year,
            self.days_per_occurrence as u32,
            placement,
            self.intercalation_disrupts_week_cycle,
        );

        CalendarSystem::new(
            id,
            &self.name,
            self.description.clone(),
            months,
            week_days,
            intercalation_rule,
        )
    }
}
