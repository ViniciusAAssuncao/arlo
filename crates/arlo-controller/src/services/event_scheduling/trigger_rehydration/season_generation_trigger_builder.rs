use crate::domain::calendar::{CalendarSystem, ResolvedCalendarDate};
use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::error::ControllerResult;
use crate::services::calendar::date_encoder;
use crate::services::event_scheduling::date_offset_calculator;
use arlo_domain::LeagueCalendarConfig;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn build_season_generation_triggers(
    pool: &SqlitePool,
    calendar: &CalendarSystem,
    configs: &[Arc<LeagueCalendarConfig>],
    reference_year: i64,
) -> ControllerResult<Vec<PendingTrigger>> {
    let mut triggers = Vec::new();

    for config in configs {
        let existing = arlo_persistence::repositories::season::season_instances::get_by_competition_and_year(
            pool,
            config.league_id(),
            reference_year,
        )
        .await?;

        if existing.is_some() {
            continue;
        }

        let timing = config.timing();
        let start_resolved = ResolvedCalendarDate::RegularDay {
            year: reference_year,
            month_order_index: timing.start_month_order_index(),
            day_of_month: timing.start_day_of_month(),
            week_day_index: 0,
        };

        let start_date = date_encoder::encode(calendar, &start_resolved)?;
        let season_gen_date =
            date_offset_calculator::subtract_months(calendar, &start_date, 1)?;

        triggers.push(PendingTrigger::new(
            season_gen_date,
            config.league_id(),
            TriggerKind::SeasonGenerationDue,
        ));
    }

    Ok(triggers)
}