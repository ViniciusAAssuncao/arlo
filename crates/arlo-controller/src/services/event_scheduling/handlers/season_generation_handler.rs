use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::event_scheduling::stage_completion_date_calculator::calculate_stage_completion_date;
use crate::services::season::season_generator::{self, GeneratedSeason};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn handle_season_generation(
    pool: &SqlitePool,
    competition_id: Uuid,
    calendar_system_id: Uuid,
    reference_year: i64,
    trigger_store: Arc<PendingTriggerStore>,
) -> ControllerResult<GeneratedSeason> {
    let generated = season_generator::generate_season_for_league(
        pool,
        competition_id,
        calendar_system_id,
        reference_year,
    )
    .await?;

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!(
            "Calendar system {} not found",
            calendar_system_id
        ))
    })?;

    if let Some(completion_date) = calculate_stage_completion_date(calendar, &generated.fixtures) {
        trigger_store
            .insert(PendingTrigger::new(
                completion_date,
                competition_id,
                TriggerKind::StageTransitionCheckDue,
            ))
            .await;
    }

    if let Some(first_date) = generated.fixtures.iter().map(|f| f.scheduled_date()).min() {
        trigger_store
            .insert(PendingTrigger::new(
                first_date,
                competition_id,
                TriggerKind::ConflictScanDue,
            ))
            .await;
    }

    Ok(generated)
}
