pub mod context;
pub mod error;
pub mod match_session;
pub mod session;
pub mod tests;

pub use context::Context;
pub use error::RuntimeError;
pub use match_session::MatchSession;
pub use session::GameSimulationSession;
pub use tests::*;

pub fn run(rt: &tokio::runtime::Runtime) -> Result<(), RuntimeError> {
    rt.block_on(async {
        let pool = arlo_db::save::resolve_current_save_pool().await?;
        let _context = Context::new(pool, tokio::runtime::Handle::current());
        Ok(())
    })
}