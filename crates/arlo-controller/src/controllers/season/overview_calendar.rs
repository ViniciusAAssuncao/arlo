use crate::domain::calendar::{CalendarCatalog, CalendarSystem};
use crate::error::{ControllerError, ControllerResult};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_overview_calendar<'a>(
    pool: &SqlitePool,
    catalog: &'a CalendarCatalog,
) -> ControllerResult<&'a CalendarSystem> {
    let metadata = arlo_db::repositories::save_metadata::get(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| {
            ControllerError::NotFound("Metadados da gravação não encontrados".to_string())
        })?;

    let save_calendar_row =
        arlo_persistence::repositories::calendar::save_calendar_state::get_by_save_uuid(
            pool,
            metadata.save_uuid(),
        )
        .await?;

    let calendar_system_id = if let Some(row) = save_calendar_row {
        Uuid::parse_str(&row.calendar_system_id)?
    } else {
        let systems = arlo_db::repositories::calendar_system::list_all(pool)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        let first = systems.into_iter().next().ok_or_else(|| {
            ControllerError::NotFound("Nenhum sistema de calendário cadastrado".to_string())
        })?;
        Uuid::parse_str(&first.id)?
    };

    catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })
}
