use crate::domain::calendar::CalendarDate;
use crate::domain::event_scheduling::TriggerKind;
use crate::error::{ControllerError, ControllerResult};
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
use tokio::task::JoinSet;
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

    let mut join_set = JoinSet::new();

    for trigger in due_triggers {
        let pool = pool.clone();
        let store = Arc::clone(&trigger_store);
        let year = current_date.year();

        match trigger.kind() {
            TriggerKind::SeasonGenerationDue => {
                join_set.spawn(async move {
                    let generated = handle_season_generation(
                        &pool,
                        trigger.competition_id(),
                        calendar_system_id,
                        year,
                        store,
                    )
                    .await?;
                    Ok::<DispatchedEventResult, ControllerError>(
                        DispatchedEventResult::SeasonGenerated {
                            competition_id: trigger.competition_id(),
                            season: generated,
                        },
                    )
                });
            }
            TriggerKind::StageTransitionCheckDue => {
                join_set.spawn(async move {
                    let outcome = progress_season(
                        &pool,
                        trigger.competition_id(),
                        calendar_system_id,
                        store,
                        year,
                    )
                    .await?;
                    Ok::<DispatchedEventResult, ControllerError>(
                        DispatchedEventResult::StageTransitionChecked {
                            competition_id: trigger.competition_id(),
                            outcome,
                        },
                    )
                });
            }
            TriggerKind::ConflictScanDue => {
                join_set.spawn(async move {
                    let report = handle_conflict_scan(
                        &pool,
                        trigger.competition_id(),
                        store,
                    )
                    .await?;
                    Ok::<DispatchedEventResult, ControllerError>(
                        DispatchedEventResult::ConflictScanned {
                            competition_id: trigger.competition_id(),
                            report,
                        },
                    )
                });
            }
        }
    }

    let mut results = Vec::new();
    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(Ok(event_result)) => results.push(event_result),
            Ok(Err(_)) => {}
            Err(join_err) => {
                return Err(ControllerError::InvalidData(format!(
                    "Task join error during event dispatch: {}",
                    join_err
                )))
            }
        }
    }

    Ok(results)
}