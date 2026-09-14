use crate::error::ControllerResult;
use crate::services::day_simulation::day_advancement_runner::run_day_advancement;
use crate::services::day_simulation::simulation_loop_handle::SimulationLoopHandle;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SimulationLoopConfig {
    pub tick_interval: Duration,
    pub max_ticks: Option<u64>,
}

impl SimulationLoopConfig {
    pub fn new(tick_interval: Duration) -> Self {
        Self {
            tick_interval,
            max_ticks: None,
        }
    }

    pub fn with_max_ticks(mut self, max_ticks: u64) -> Self {
        self.max_ticks = Some(max_ticks);
        self
    }
}

impl Default for SimulationLoopConfig {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_millis(50),
            max_ticks: None,
        }
    }
}

pub async fn run_simulation_loop(
    pool: &SqlitePool,
    save_uuid: Uuid,
    trigger_store: &PendingTriggerStore,
    config: SimulationLoopConfig,
    mut stop_rx: watch::Receiver<bool>
) -> ControllerResult<()> {
    let mut tick_count: u64 = 0;

    while !*stop_rx.borrow() {
        if let Some(max) = config.max_ticks {
            if tick_count >= max {
                break;
            }
        }

        run_day_advancement(pool, save_uuid, trigger_store).await?;
        tick_count += 1;

        if let Some(max) = config.max_ticks {
            if tick_count >= max {
                break;
            }
        }

        if config.tick_interval > Duration::ZERO {
            tokio::select! {
                _ = tokio::time::sleep(config.tick_interval) => {},
                changed = stop_rx.changed() => {
                    if changed.is_ok() && *stop_rx.borrow() {
                        break;
                    }
                }
            }
        } else if *stop_rx.borrow() {
            break;
        }
    }

    Ok(())
}

pub fn start_simulation_loop(
    pool: SqlitePool,
    save_uuid: Uuid,
    trigger_store: Arc<PendingTriggerStore>,
    config: SimulationLoopConfig
) -> SimulationLoopHandle {
    let (stop_tx, stop_rx) = watch::channel(false);

    let join_handle = tokio::spawn(async move {
        run_simulation_loop(&pool, save_uuid, &trigger_store, config, stop_rx).await
    });

    SimulationLoopHandle::new(stop_tx, join_handle)
}
