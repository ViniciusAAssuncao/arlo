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
    pub peak_speed_meters_per_sec: f64,
    pub critical_speed_meters_per_sec: f64,
    pub high_intensity_distance_mirim: f64,
    pub supramaximal_time_seconds: f64,
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
        peak_speed_meters_per_sec: f64,
        critical_speed_meters_per_sec: f64,
        high_intensity_distance_mirim: f64,
        supramaximal_time_seconds: f64,
        exposure_duration_seconds: f64,
        age_years: f64,
    ) -> Self {
        Self {
            player_id,
            team_id,
            player_table,
            physical_state,
            injury_profile,
            peak_speed_meters_per_sec,
            critical_speed_meters_per_sec,
            high_intensity_distance_mirim,
            supramaximal_time_seconds,
            exposure_duration_seconds,
            age_years,
        }
    }
}
