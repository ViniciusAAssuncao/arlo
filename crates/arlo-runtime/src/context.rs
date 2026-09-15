use arlo_controller::services::event_scheduling::PendingTriggerStore;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::runtime::Handle;

pub struct Context {
    pub pool: SqlitePool,
    pub handle: Handle,
    pub trigger_store: Arc<PendingTriggerStore>,
}

impl Context {
    pub fn new(pool: SqlitePool, handle: Handle, trigger_store: Arc<PendingTriggerStore>) -> Self {
        Self {
            pool,
            handle,
            trigger_store,
        }
    }
}