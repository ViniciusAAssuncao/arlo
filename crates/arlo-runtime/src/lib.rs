pub mod context;
pub mod error;
pub mod match_session;
pub mod tests;
pub mod session;

pub use context::Context;
pub use error::RuntimeError;
pub use match_session::MatchSession;
pub use tests::*;
pub use session::GameSimulationSession;

pub fn run(rt: &tokio::runtime::Runtime) -> Result<(), RuntimeError> {
    rt.block_on(async {
        let pool = arlo_db::save::resolve_current_save_pool().await?;
        let _context = Context::new(pool, tokio::runtime::Handle::current());
        validation::run_full_validation_and_simulation();
        Ok(())
    })
}