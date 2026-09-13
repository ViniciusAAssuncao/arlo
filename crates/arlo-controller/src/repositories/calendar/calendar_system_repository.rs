use crate::domain::calendar::CalendarSystem;
use crate::error::ControllerResult;
use crate::repositories::calendar::models::{
    CalendarMonthRow, CalendarSystemRow, CalendarWeekDayRow,
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> ControllerResult<Option<CalendarSystem>> {
    let row = sqlx::query_as::<_, CalendarSystemRow>(
        "SELECT id, name, description, leap_units_per_cycle, cycle_length_years, cycle_reference_year, days_per_occurrence, intercalation_placement_kind, intercalation_placement_month_order_index, intercalation_disrupts_week_cycle, created_at_unix_seconds FROM calendar_systems WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let month_rows = sqlx::query_as::<_, CalendarMonthRow>(
        "SELECT id, calendar_system_id, order_index, name, day_count FROM calendar_months WHERE calendar_system_id = ? ORDER BY order_index ASC",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    let mut months = Vec::with_capacity(month_rows.len());
    for mr in month_rows {
        months.push(mr.to_domain()?);
    }

    let week_day_rows = sqlx::query_as::<_, CalendarWeekDayRow>(
        "SELECT id, calendar_system_id, order_index, name FROM calendar_week_days WHERE calendar_system_id = ? ORDER BY order_index ASC",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    let mut week_days = Vec::with_capacity(week_day_rows.len());
    for wdr in week_day_rows {
        week_days.push(wdr.to_domain()?);
    }

    Ok(Some(row.to_domain(months, week_days)?))
}

pub async fn list_all(pool: &SqlitePool) -> ControllerResult<Vec<CalendarSystem>> {
    let system_rows = sqlx::query_as::<_, CalendarSystemRow>(
        "SELECT id, name, description, leap_units_per_cycle, cycle_length_years, cycle_reference_year, days_per_occurrence, intercalation_placement_kind, intercalation_placement_month_order_index, intercalation_disrupts_week_cycle, created_at_unix_seconds FROM calendar_systems",
    )
    .fetch_all(pool)
    .await?;

    let month_rows = sqlx::query_as::<_, CalendarMonthRow>(
        "SELECT id, calendar_system_id, order_index, name, day_count FROM calendar_months ORDER BY calendar_system_id, order_index ASC",
    )
    .fetch_all(pool)
    .await?;

    let mut months_by_system: HashMap<String, Vec<CalendarMonthRow>> = HashMap::new();
    for mr in month_rows {
        months_by_system
            .entry(mr.calendar_system_id.clone())
            .or_default()
            .push(mr);
    }

    let week_day_rows = sqlx::query_as::<_, CalendarWeekDayRow>(
        "SELECT id, calendar_system_id, order_index, name FROM calendar_week_days ORDER BY calendar_system_id, order_index ASC",
    )
    .fetch_all(pool)
    .await?;

    let mut week_days_by_system: HashMap<String, Vec<CalendarWeekDayRow>> = HashMap::new();
    for wdr in week_day_rows {
        week_days_by_system
            .entry(wdr.calendar_system_id.clone())
            .or_default()
            .push(wdr);
    }

    let mut systems = Vec::with_capacity(system_rows.len());
    for sr in system_rows {
        let month_rows_for_system = months_by_system.remove(&sr.id).unwrap_or_default();
        let mut months = Vec::with_capacity(month_rows_for_system.len());
        for mr in month_rows_for_system {
            months.push(mr.to_domain()?);
        }

        let week_day_rows_for_system = week_days_by_system.remove(&sr.id).unwrap_or_default();
        let mut week_days = Vec::with_capacity(week_day_rows_for_system.len());
        for wdr in week_day_rows_for_system {
            week_days.push(wdr.to_domain()?);
        }

        systems.push(sr.to_domain(months, week_days)?);
    }

    Ok(systems)
}