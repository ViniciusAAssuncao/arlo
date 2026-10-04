use crate::controllers::season::overview_calendar::resolve_overview_calendar;
use crate::domain::calendar::ResolvedCalendarDate;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::calendar::date_encoder;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub(super) async fn season_start_unix_seconds(
    pool: &SqlitePool,
    season_id: Uuid,
) -> ControllerResult<i64> {
    let row = sqlx::query(
        "SELECT si.reference_year, c.season_start_month_order_index, c.season_start_day_of_month FROM season_instances si LEFT JOIN league_calendar_configs c ON c.competition_id = si.competition_id WHERE si.id = ?",
    )
    .bind(season_id.to_string())
    .fetch_one(pool)
    .await?;
    let (year, day) = if let (Some(month), Some(day)) = (
        row.try_get::<Option<i64>, _>("season_start_month_order_index")?,
        row.try_get::<Option<i64>, _>("season_start_day_of_month")?,
    ) {
        let catalog = get_or_load_calendar_catalog(pool).await?;
        let calendar = resolve_overview_calendar(pool, &catalog).await?;
        let date = date_encoder::encode(
            calendar,
            &ResolvedCalendarDate::RegularDay {
                year: row.try_get("reference_year")?,
                month_order_index: u32::try_from(month).map_err(|_| {
                    ControllerError::InvalidData("Invalid season start month".into())
                })?,
                day_of_month: u32::try_from(day)
                    .map_err(|_| ControllerError::InvalidData("Invalid season start day".into()))?,
                week_day_index: 0,
            },
        )?;
        (date.year(), date.day_of_year())
    } else {
        let fixture = sqlx::query(
            "SELECT f.scheduled_year, f.scheduled_day_of_year FROM fixtures f JOIN season_stages ss ON ss.id = f.season_stage_id WHERE ss.season_instance_id = ? ORDER BY f.scheduled_year, f.scheduled_day_of_year LIMIT 1",
        )
        .bind(season_id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ControllerError::NotFound("Season has no scheduled fixtures".into()))?;
        (
            fixture.try_get("scheduled_year")?,
            u32::try_from(fixture.try_get::<i64, _>("scheduled_day_of_year")?)
                .map_err(|_| ControllerError::InvalidData("Invalid fixture day".into()))?,
        )
    };
    Ok((year - 1970) * 31_557_600 + i64::from(day) * 86_400)
}
