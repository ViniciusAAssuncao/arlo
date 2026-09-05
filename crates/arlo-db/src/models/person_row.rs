use crate::error::DbResult;
use arlo_domain::Person;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PersonRow {
    pub id: String,
    pub name: String,
    pub height_m: f64,
    pub birthdate_unix_seconds: i64,
    pub nationality_id: String,
}

impl PersonRow {
    pub fn to_domain(&self) -> DbResult<Person> {
        let id = Uuid::parse_str(&self.id)?;
        let nationality_id = Uuid::parse_str(&self.nationality_id)?;
        Person::new(
            id,
            &self.name,
            self.height_m,
            self.birthdate_unix_seconds,
            nationality_id,
        )
        .map_err(Into::into)
    }
}