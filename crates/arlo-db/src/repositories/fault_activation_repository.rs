use crate::error::{DbError, DbResult};
use arlo_domain::{FaultActivation, FaultOffenderRole};
use sqlx::{Row, SqlitePool};

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<FaultActivation>> {
    let rows = sqlx::query(
        "SELECT fault_code, context, offender_role, weight FROM fault_activation_contexts ORDER BY context, fault_code, offender_role",
    )
    .fetch_all(pool)
    .await?;
    rows.into_iter()
        .map(|row| {
            let fault_code: String = row.try_get("fault_code")?;
            let context: String = row.try_get("context")?;
            let role: String = row.try_get("offender_role")?;
            let weight: f64 = row.try_get("weight")?;
            let offender_role = FaultOffenderRole::from_catalog_value(&role)
                .ok_or_else(|| DbError::InvalidEnum(role.clone()))?;
            FaultActivation::new(fault_code, context, offender_role, weight)
                .ok_or_else(|| DbError::InvalidData("invalid fault activation context".into()))
        })
        .collect()
}
