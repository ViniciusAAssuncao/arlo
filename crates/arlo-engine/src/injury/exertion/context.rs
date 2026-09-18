use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use arlo_domain::PlayerInjuryProfile;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ExertionInjuryContext<'a> {
    pub player_id: Uuid,
    pub team_id: Uuid,
    pub player_table: &'a PlayerAttributeTable,
    pub physical_state: PhysicalState,
    pub injury_profile: PlayerInjuryProfile,
    pub intensity_strain: f64,
    pub exposure_duration_seconds: f64,
    pub age_years: f64,
}

impl<'a> ExertionInjuryContext<'a> {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        player_table: &'a PlayerAttributeTable,
        physical_state: PhysicalState,
        injury_profile: PlayerInjuryProfile,
        intensity_strain: f64,
        exposure_duration_seconds: f64,
        age_years: f64,
    ) -> Self {
        Self {
            player_id,
            team_id,
            player_table,
            physical_state,
            injury_profile,
            intensity_strain,
            exposure_duration_seconds,
            age_years,
        }
    }
}