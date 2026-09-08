use crate::error::DbResult;
use crate::models::manager_preferred_formation_row::ManagerPreferredFormationRow;
use crate::models::manager_profile_codes::{
    parse_artrine_dependency, parse_defensive_approach, parse_offensive_approach,
    parse_rotation_policy,
};
use arlo_domain::ManagerTacticalProfile;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ManagerTacticalProfileRow {
    pub id: String,
    pub manager_id: String,
    pub offensive_approach: String,
    pub defensive_approach: String,
    pub rotation_policy: String,
    pub artrine_dependency: String,
    pub flexibility_tendency: f64,
}

impl ManagerTacticalProfileRow {
    pub fn to_domain(
        &self,
        formation_rows: &[ManagerPreferredFormationRow],
    ) -> DbResult<ManagerTacticalProfile> {
        let id = Uuid::parse_str(&self.id)?;
        let manager_id = Uuid::parse_str(&self.manager_id)?;
        let offensive_approach = parse_offensive_approach(&self.offensive_approach)?;
        let defensive_approach = parse_defensive_approach(&self.defensive_approach)?;
        let rotation_policy = parse_rotation_policy(&self.rotation_policy)?;
        let artrine_dependency = parse_artrine_dependency(&self.artrine_dependency)?;

        let mut preferred_formation_ids = Vec::with_capacity(formation_rows.len());
        for row in formation_rows {
            preferred_formation_ids.push(row.formation_id()?);
        }

        ManagerTacticalProfile::new(
            id,
            manager_id,
            offensive_approach,
            defensive_approach,
            rotation_policy,
            artrine_dependency,
            self.flexibility_tendency,
            preferred_formation_ids,
        )
        .map_err(Into::into)
    }
}