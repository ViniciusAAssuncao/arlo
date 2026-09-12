use arlo_stats::PlayerPhysicalStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerPhysicalRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
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

impl MatchPlayerPhysicalRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
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
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
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

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerPhysicalStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.end_energy_level,
            stats.peak_anaerobic_depletion,
            stats.total_distance_covered,
            stats.high_intensity_distance,
            stats.low_intensity_distance,
            stats.metabolic_energy_joules,
            stats.peak_speed_meters_per_sec,
            stats.intra_match_recovery_amount,
            stats.distance_first_zone,
            stats.distance_second_zone,
            stats.distance_corridors,
            stats.distance_central,
        )
    }
}
