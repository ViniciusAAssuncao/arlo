use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CalendarMonthRow {
    pub id: String,
    pub calendar_system_id: String,
    pub order_index: i32,
    pub name: String,
    pub day_count: i32,
}
