use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerPhysicalSnapshot {
    pub player_id: Uuid,
    pub end_energy_level: f64,
    pub peak_anaerobic_depletion: f64,
    pub total_distance_covered: f64,
    pub intra_match_recovery_amount: f64,
}

impl PlayerPhysicalSnapshot {
    pub fn new(
        player_id: Uuid,
        end_energy_level: f64,
        peak_anaerobic_depletion: f64,
        total_distance_covered: f64,
        intra_match_recovery_amount: f64,
    ) -> Self {
        Self {
            player_id,
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            intra_match_recovery_amount,
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
            intra_match_recovery_amount: 0.0,
        }
    }
}