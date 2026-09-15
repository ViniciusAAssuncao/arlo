use crate::domain::calendar::{CalendarSystem, ResolvedCalendarDate};
use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::domain::season::PromotionRelegationOutcome;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::calendar::date_encoder;
use crate::services::event_scheduling::date_offset_calculator;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::season::persistence::finalize_season_instance;
use crate::services::season::progression::champion_resolver::resolve_season_champion;
use crate::services::season::progression::league_movement_applier::resolve_and_apply_promotion_relegation;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn finalize_season(
    pool: &SqlitePool,
    competition_id: Uuid,
    season_instance_id: Uuid,
    knockout_champion: Option<Uuid>,
    calendar: &CalendarSystem,
    trigger_store: Arc<PendingTriggerStore>,
) -> ControllerResult<PromotionRelegationOutcome> {
    let season_row = arlo_persistence::repositories::season::season_instances::get_by_id(
        pool,
        season_instance_id,
    )
    .await?
    .ok_or_else(|| {
        ControllerError::NotFound(format!(
            "Season instance {} not found",
            season_instance_id
        ))
    })?;

    let reference_year = season_row.reference_year;

    let outcome =
        resolve_and_apply_promotion_relegation(pool, competition_id, season_instance_id).await?;
    finalize_season_instance(pool, season_instance_id).await?;
    let title =
        resolve_season_champion(pool, competition_id, season_instance_id, knockout_champion).await?;
    arlo_db::repositories::title::insert(pool, &title)
        .await
        .map_err(|e| crate::error::ControllerError::InvalidData(e.to_string()))?;

    let config = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let next_reference_year = reference_year + 1;
    let timing = config.timing();
    let start_resolved = ResolvedCalendarDate::RegularDay {
        year: next_reference_year,
        month_order_index: timing.start_month_order_index(),
        day_of_month: timing.start_day_of_month(),
        week_day_index: 0,
    };

    let start_date = date_encoder::encode(calendar, &start_resolved)?;
    let season_gen_date = date_offset_calculator::subtract_months(calendar, &start_date, 1)?;

    trigger_store
        .insert(PendingTrigger::new(
            season_gen_date,
            competition_id,
            TriggerKind::SeasonGenerationDue,
        ))
        .await;

    Ok(outcome)
}