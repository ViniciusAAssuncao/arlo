use crate::error::DbResult;
use arlo_domain::Country;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CountryRow {
    pub id: String,
    pub name: String,
    pub continent_id: String,
    pub federation_id: Option<String>,
}

impl CountryRow {
    pub fn to_domain(&self) -> DbResult<Country> {
        let id = Uuid::parse_str(&self.id)?;
        let continent_id = Uuid::parse_str(&self.continent_id)?;
        let federation_id = match &self.federation_id {
            Some(fid) => Some(Uuid::parse_str(fid)?),
            None => None,
        };
        Country::new(id, &self.name, continent_id, federation_id).map_err(Into::into)
    }
}