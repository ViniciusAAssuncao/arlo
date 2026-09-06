use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerPhysicalSnapshot {
    pub player_id: Uuid,
    pub end_energy_level: f64,
    pub peak_anaerobic_depletion: f64,
    pub total_distance_covered: f64,
    pub high_intensity_distance: f64,
    pub low_intensity_distance: f64,
    pub metabolic_energy_joules: f64,
    pub peak_speed_meters_per_sec: f64,
    pub intra_match_recovery_amount: f64,
    pub distance_first_zone: f64,
    pub distance_second_zone: f64,
    pub distance_corridors: f64,
    pub distance_central: f64,
}

impl PlayerPhysicalSnapshot {
    pub fn new(
        player_id: Uuid,
        end_energy_level: f64,
        peak_anaerobic_depletion: f64,
        total_distance_covered: f64,
        high_intensity_distance: f64,
        low_intensity_distance: f64,
        metabolic_energy_joules: f64,
        peak_speed_meters_per_sec: f64,
        intra_match_recovery_amount: f64,
        distance_first_zone: f64,
        distance_second_zone: f64,
        distance_corridors: f64,
        distance_central: f64,
    ) -> Self {
        Self {
            player_id,
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            high_intensity_distance,
            low_intensity_distance,
            metabolic_energy_joules,
            peak_speed_meters_per_sec,
            intra_match_recovery_amount,
            distance_first_zone,
            distance_second_zone,
            distance_corridors,
            distance_central,
        }
    }
}

impl Default for PlayerPhysicalSnapshot {
    fn default() -> Self {
        Self {
            player_id: Uuid::nil(),
            end_energy_level: 1.0,
            peak_anaerobic_depletion: 0.0,
            total_distance_covered: 0.0,
            high_intensity_distance: 0.0,
            low_intensity_distance: 0.0,
            metabolic_energy_joules: 0.0,
            peak_speed_meters_per_sec: 0.0,
            intra_match_recovery_amount: 0.0,
            distance_first_zone: 0.0,
            distance_second_zone: 0.0,
            distance_corridors: 0.0,
            distance_central: 0.0,
        }
    }
}