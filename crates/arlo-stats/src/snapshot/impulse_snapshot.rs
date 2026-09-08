use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseRunSnapshot {
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub duration_seconds: f64,
    pub peak_value: u8,
    pub integrated_intensity: f64,
    pub shifts_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TeamImpulseRunSnapshot {
    pub team_id: Uuid,
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub duration_seconds: f64,
    pub peak_average_value: f64,
    pub integrated_intensity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerImpulseSnapshot {
    pub player_id: Uuid,
    pub baseline: f64,
    pub current_value: u8,
    pub min_value: u8,
    pub max_value: u8,
    pub average_value: f64,
    pub total_shifts: u32,
    pub positive_shifts: u32,
    pub negative_shifts: u32,
    pub time_below_baseline_seconds: f64,
    pub critical_reached_count: u32,
    pub runs_count: u32,
    pub longest_run_duration_seconds: f64,
    pub peak_run_value: u8,
    pub total_integrated_run_intensity: f64,
    pub runs: Vec<ImpulseRunSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamImpulseSnapshot {
    pub team_id: Uuid,
    pub average_baseline: f64,
    pub current_average_value: f64,
    pub min_average_value: f64,
    pub max_average_value: f64,
    pub average_value: f64,
    pub time_below_baseline_seconds: f64,
    pub runs_count: u32,
    pub longest_run_duration_seconds: f64,
    pub peak_run_average_value: f64,
    pub total_integrated_run_intensity: f64,
    pub runs: Vec<TeamImpulseRunSnapshot>,
}