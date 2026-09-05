use crate::error::{DbError, DbResult};
use arlo_domain::{Venue, VenueKind};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct VenueRow {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub owner_team_id: Option<String>,
    pub country_id: String,
    pub capacity: Option<i32>,
    pub pitch_length_mirim: Option<f64>,
    pub pitch_width_mirim: Option<f64>,
}

impl VenueRow {
    pub fn to_domain(&self) -> DbResult<Venue> {
        let id = Uuid::parse_str(&self.id)?;
        let country_id = Uuid::parse_str(&self.country_id)?;
        let owner_team_id = match &self.owner_team_id {
            Some(tid) => Some(Uuid::parse_str(tid)?),
            None => None,
        };
        let kind = match self.kind.as_str() {
            "MatchStadium" => VenueKind::MatchStadium,
            "TrainingCenter" => VenueKind::TrainingCenter,
            _ => {
                return Err(DbError::InvalidEnum(format!(
                    "Invalid venue kind: {}",
                    self.kind
                )))
            }
        };
        Venue::new(
            id,
            &self.name,
            kind,
            owner_team_id,
            country_id,
            self.capacity,
            self.pitch_length_mirim,
            self.pitch_width_mirim,
        )
        .map_err(Into::into)
    }
}
