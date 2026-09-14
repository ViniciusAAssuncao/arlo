use crate::error::ControllerResult;
use crate::services::day_simulation::day_advancement_runner::{
    self, DayAdvancementResult,
};
use crate::services::day_simulation::simulation_loop_driver::{
    self, SimulationLoopConfig,
};
use crate::services::day_simulation::simulation_loop_handle::SimulationLoopHandle;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub fn start_simulation_loop(
    pool: &SqlitePool,
    save_uuid: Uuid,
    trigger_store: Arc<PendingTriggerStore>,
    config: SimulationLoopConfig,
) -> SimulationLoopHandle {
    simulation_loop_driver::start_simulation_loop(
        pool.clone(),
        save_uuid,
        trigger_store,
        config,
    )
}

pub async fn stop_simulation_loop(handle: SimulationLoopHandle) -> ControllerResult<()> {
    handle.stop_and_join().await
}

pub async fn advance_single_day(
    pool: &SqlitePool,
    save_uuid: Uuid,
    trigger_store: &PendingTriggerStore,
) -> ControllerResult<DayAdvancementResult> {
    day_advancement_runner::run_day_advancement(pool, save_uuid, trigger_store).await
}
