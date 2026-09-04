use sqlx::SqlitePool;
use tokio::runtime::Handle;

pub struct Context {
    pub pool: SqlitePool,
    pub handle: Handle,
}

impl Context {
    pub fn new(pool: SqlitePool, handle: Handle) -> Self {
        Self { pool, handle }
    }
}