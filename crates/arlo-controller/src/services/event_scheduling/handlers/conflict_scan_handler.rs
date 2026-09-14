use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictScanReport {
    pub competition_id: Uuid,
    pub conflicts_detected: usize,
}

pub async fn handle_conflict_scan(
    _pool: &SqlitePool,
    competition_id: Uuid,
) -> ControllerResult<ConflictScanReport> {
    Ok(ConflictScanReport {
        competition_id,
        conflicts_detected: 0,
    })
}