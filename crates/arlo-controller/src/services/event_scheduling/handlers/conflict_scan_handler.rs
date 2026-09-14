use crate::error::ControllerResult;
pub use crate::services::season::conflict::ConflictScanReport;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn handle_conflict_scan(
    _pool: &SqlitePool,
    competition_id: Uuid,
) -> ControllerResult<ConflictScanReport> {
    Ok(ConflictScanReport::empty(competition_id))
}