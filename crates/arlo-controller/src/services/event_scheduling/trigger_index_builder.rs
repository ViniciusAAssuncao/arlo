use crate::domain::calendar::{CalendarSystem, ResolvedCalendarDate};
use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::error::ControllerResult;
use crate::services::calendar::date_encoder;
use crate::services::event_scheduling::date_offset_calculator;
use arlo_domain::LeagueCalendarConfig;
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn build_trigger_index(
    calendar: &CalendarSystem,
    configs: &[Arc<LeagueCalendarConfig>],
    reference_year: i64,
) -> ControllerResult<BTreeMap<(i64, u32), Vec<PendingTrigger>>> {
    let mut index: BTreeMap<(i64, u32), Vec<PendingTrigger>> = BTreeMap::new();

    for config in configs {
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

        let season_gen_trigger = PendingTrigger::new(
            season_gen_date,
            config.league_id(),
            TriggerKind::SeasonGenerationDue,
        );

        index
            .entry((season_gen_date.year(), season_gen_date.day_of_year()))
            .or_default()
            .push(season_gen_trigger);
    }

    Ok(index)
}