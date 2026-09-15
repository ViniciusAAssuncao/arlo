pub mod context;
pub mod error;
pub mod match_session;
pub mod session;

pub use context::Context;
pub use error::RuntimeError;
pub use match_session::MatchSession;
pub use session::GameSimulationSession;

pub fn run(rt: &tokio::runtime::Runtime) -> Result<(), RuntimeError> {
    rt.block_on(async {
        let pool = arlo_db::save::resolve_current_save_pool().await?;
        arlo_controller::persistence::run_migrations(&pool).await?;

        let trigger_store = std::sync::Arc::new(
            arlo_controller::services::event_scheduling::PendingTriggerStore::new(),
        );
        let catalog =
            arlo_controller::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog(
                &pool,
            )
            .await?;

        let save_state_row = sqlx::query_as::<
            _,
            arlo_persistence::models::calendar::SaveCalendarStateRow,
        >(
            "SELECT save_uuid, calendar_system_id, current_year, current_day_of_year, assigned_at_unix_seconds FROM save_calendar_states LIMIT 1",
        )
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten();

        let (calendar, reference_year) = if let Some(row) = save_state_row {
            let cal_id = uuid::Uuid::parse_str(&row.calendar_system_id).ok();
            let cal = cal_id
                .and_then(|id| catalog.get(&id))
                .or_else(|| catalog.all().next());
            (cal, row.current_year)
        } else {
            let cal = catalog.all().next();
            let year = cal
                .map(|c| c.intercalation_rule().cycle_reference_year())
                .unwrap_or(0);
            (cal, year)
        };

        if let Some(cal) = calendar {
            trigger_store.rebuild(&pool, cal, reference_year).await?;
        }

        let _context = Context::new(pool, tokio::runtime::Handle::current(), trigger_store);
        Ok(())
    })
}