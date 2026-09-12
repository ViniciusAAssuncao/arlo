use crate::error::DbResult;
use crate::models::fault_severity_code::parse_fault_severity;
use arlo_domain::FaultDefinition;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct FaultDefinitionRow {
    pub id: String,
    pub code: String,
    pub description: String,
    pub severity: String,
}

impl FaultDefinitionRow {
    pub fn to_domain(&self) -> DbResult<FaultDefinition> {
        let id = Uuid::parse_str(&self.id)?;
        let severity = parse_fault_severity(&self.severity)?;
        FaultDefinition::new(id, &self.code, &self.description, severity).map_err(Into::into)
    }
}