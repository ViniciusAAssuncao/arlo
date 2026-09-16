use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CalendarSystemRow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub leap_units_per_cycle: Option<i32>,
    pub cycle_length_years: Option<i32>,
    pub cycle_reference_year: Option<i32>,
    pub days_per_occurrence: Option<i32>,
    pub intercalation_placement_kind: Option<String>,
}