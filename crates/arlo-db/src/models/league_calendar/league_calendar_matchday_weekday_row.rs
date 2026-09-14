use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LeagueCalendarMatchdayWeekdayRow {
    pub id: String,
    pub league_calendar_config_id: String,
    pub weekday_order_index: i32,
}
