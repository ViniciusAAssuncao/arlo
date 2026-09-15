pub mod conflict_scan_trigger_rehydrator;
pub mod season_generation_trigger_builder;
pub mod stage_transition_trigger_rehydrator;

pub use conflict_scan_trigger_rehydrator::*;
pub use season_generation_trigger_builder::*;
pub use stage_transition_trigger_rehydrator::*;

use crate::domain::calendar::CalendarSystem;
use crate::domain::event_scheduling::PendingTrigger;
use crate::error::ControllerResult;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use sqlx::SqlitePool;
use std::collections::BTreeMap;

pub async fn rehydrate_all_triggers(
    pool: &SqlitePool,
    calendar: &CalendarSystem,
    reference_year: i64,
) -> ControllerResult<BTreeMap<(i64, u32), Vec<PendingTrigger>>> {
    let leagues = arlo_db::repositories::league::list_all(pool)
        .await
        .map_err(|e| crate::error::ControllerError::InvalidData(e.to_string()))?;

    let mut configs = Vec::with_capacity(leagues.len());
    for league in leagues {
        if let Some(config) = get_or_load_league_calendar_config(pool, league.id()).await? {
            configs.push(config);
        }
    }

    let season_gen_triggers =
        build_season_generation_triggers(pool, calendar, &configs, reference_year).await?;
    let stage_trans_triggers =
        rehydrate_stage_transition_triggers(pool, calendar, &configs).await?;
    let conflict_scan_triggers =
        rehydrate_conflict_scan_triggers(pool, &configs).await?;

    let mut index: BTreeMap<(i64, u32), Vec<PendingTrigger>> = BTreeMap::new();

    for trigger in season_gen_triggers
        .into_iter()
        .chain(stage_trans_triggers)
        .chain(conflict_scan_triggers)
    {
        let date = trigger.trigger_date();
        index
            .entry((date.year(), date.day_of_year()))
            .or_default()
            .push(trigger);
    }

    Ok(index)
}