use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::spatial::LiveCollision;
use arlo_domain::PlayerInjuryProfile;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ContactInjuryContext<'a> {
    pub collision: &'a LiveCollision,
    pub carrier_id: Uuid,
    pub carrier_team_id: Uuid,
    pub carrier_table: &'a PlayerAttributeTable,
    pub carrier_physical_state: PhysicalState,
    pub carrier_injury_profile: PlayerInjuryProfile,
    pub carrier_age_years: f64,
    pub defender_id: Uuid,
    pub defender_team_id: Uuid,
    pub defender_table: &'a PlayerAttributeTable,
    pub defender_physical_state: PhysicalState,
    pub defender_injury_profile: PlayerInjuryProfile,
    pub defender_age_years: f64,
}

impl<'a> ContactInjuryContext<'a> {
    pub fn new(
        collision: &'a LiveCollision,
        carrier_id: Uuid,
        carrier_team_id: Uuid,
        carrier_table: &'a PlayerAttributeTable,
        carrier_physical_state: PhysicalState,
        carrier_injury_profile: PlayerInjuryProfile,
        carrier_age_years: f64,
        defender_id: Uuid,
        defender_team_id: Uuid,
        defender_table: &'a PlayerAttributeTable,
        defender_physical_state: PhysicalState,
        defender_injury_profile: PlayerInjuryProfile,
        defender_age_years: f64,
    ) -> Self {
        Self {
            collision,
            carrier_id,
            carrier_team_id,
            carrier_table,
            carrier_physical_state,
            carrier_injury_profile,
            carrier_age_years,
            defender_id,
            defender_team_id,
            defender_table,
            defender_physical_state,
            defender_injury_profile,
            defender_age_years,
        }
    }
}
