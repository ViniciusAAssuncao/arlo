use crate::error::DbResult;
use crate::models::punishment_kind_code::parse_punishment_kind;
use arlo_domain::FaultPunishmentOption;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct FaultPunishmentOptionRow {
    pub id: String,
    pub fault_definition_id: String,
    pub kind: String,
    pub magnitude_min: Option<i32>,
    pub magnitude_max: Option<i32>,
}

impl FaultPunishmentOptionRow {
    pub fn to_domain(&self) -> DbResult<FaultPunishmentOption> {
        let id = Uuid::parse_str(&self.id)?;
        let fault_definition_id = Uuid::parse_str(&self.fault_definition_id)?;
        let kind = parse_punishment_kind(&self.kind)?;
        FaultPunishmentOption::new(
            id,
            fault_definition_id,
            kind,
            self.magnitude_min,
            self.magnitude_max,
        )
        .map_err(Into::into)
    }
}