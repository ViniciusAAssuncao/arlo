use crate::error::DbResult;
use crate::models::body_region_code::parse_body_region;
use crate::models::injury_mechanism_code::parse_injury_mechanism;
use arlo_domain::InjuryDefinition;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct InjuryDefinitionRow {
    pub id: String,
    pub code: String,
    pub description: String,
    pub mechanism: String,
    pub body_region: String,
    pub relative_frequency: f64,
}

impl InjuryDefinitionRow {
    pub fn to_domain(&self) -> DbResult<InjuryDefinition> {
        let id = Uuid::parse_str(&self.id)?;
        let mechanism = parse_injury_mechanism(&self.mechanism)?;
        let body_region = parse_body_region(&self.body_region)?;
        InjuryDefinition::new(
            id,
            &self.code,
            &self.description,
            mechanism,
            body_region,
            self.relative_frequency,
        )
        .map_err(Into::into)
    }
}