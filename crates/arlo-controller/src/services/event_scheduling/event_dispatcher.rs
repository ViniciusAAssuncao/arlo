use crate::domain::calendar::CalendarDate;
use crate::domain::event_scheduling::TriggerKind;
use crate::error::ControllerResult;
use crate::services::event_scheduling::handlers::conflict_scan_handler::{
    handle_conflict_scan, ConflictScanReport,
};
use crate::services::event_scheduling::handlers::season_generation_handler::handle_season_generation;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::season::progression::season_progression_orchestrator::{
    progress_season, ProgressionOutcome,
};
use crate::services::season::season_generator::GeneratedSeason;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum DispatchedEventResult {
    SeasonGenerated {
        competition_id: Uuid,
        season: GeneratedSeason,
    },
    StageTransitionChecked {
        competition_id: Uuid,
        outcome: ProgressionOutcome,
    },
    ConflictScanned {
        competition_id: Uuid,
        report: ConflictScanReport,
    },
}

pub async fn dispatch_due_events(
    pool: &SqlitePool,
    trigger_store: Arc<PendingTriggerStore>,
    calendar_system_id: Uuid,
    current_date: &CalendarDate,
) -> ControllerResult<Vec<DispatchedEventResult>> {
    let due_triggers = trigger_store.pop_due(current_date).await;
    if due_triggers.is_empty() {
        return Ok(Vec::new());
    }

    let mut results = Vec::with_capacity(due_triggers.len());

    for trigger in due_triggers {
        let year = current_date.year();

        match trigger.kind() {
            TriggerKind::SeasonGenerationDue => {
                let generated = handle_season_generation(
                    pool,
                    trigger.competition_id(),
                    calendar_system_id,
                    year,
                    Arc::clone(&trigger_store),
                )
                .await?;
                results.push(DispatchedEventResult::SeasonGenerated {
                    competition_id: trigger.competition_id(),
                    season: generated,
                });
            }
            TriggerKind::StageTransitionCheckDue => {
                let outcome = progress_season(
                    pool,
                    trigger.competition_id(),
                    calendar_system_id,
                    Arc::clone(&trigger_store),
                    year,
                )
                .await?;
                results.push(DispatchedEventResult::StageTransitionChecked {
                    competition_id: trigger.competition_id(),
                    outcome,
                });
            }
            TriggerKind::ConflictScanDue => {
                let report = handle_conflict_scan(
                    pool,
                    trigger.competition_id(),
                    Arc::clone(&trigger_store),
                )
                .await?;
                results.push(DispatchedEventResult::ConflictScanned {
                    competition_id: trigger.competition_id(),
                    report,
                });
            }
        }
    }

    Ok(results)
}
