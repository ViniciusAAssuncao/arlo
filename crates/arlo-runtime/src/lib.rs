pub mod context;
pub mod error;
pub mod session;

pub use context::Context;
pub use error::RuntimeError;

pub fn run(rt: &tokio::runtime::Runtime) -> Result<(), RuntimeError> {
    rt.block_on(async {
        let db_path = std::path::Path::new("arlo.db");
        let _pool = arlo_db::connection::provision_database(db_path).await?;
        Ok(())
    })
}