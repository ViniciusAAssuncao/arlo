use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchImpulseRunDto {
    pub run_index: u32,
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub duration_seconds: f64,
    pub peak_value: u8,
    pub integrated_intensity: f64,
    pub shifts_count: u32,
    pub average_intensity: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchImpulseShiftByKindDto {
    pub event_kind: String,
    pub shifts_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchImpulseDto {
    pub baseline: f64,
    pub current_value: u8,
    pub initial_value: u8,
    pub min_value: u8,
    pub max_value: u8,
    pub average_value: f64,
    pub shifts_count: u32,
    pub positive_shifts: u32,
    pub negative_shifts: u32,
    pub time_below_baseline_seconds: f64,
    pub critical_reached_count: u32,
    pub runs_count: u32,
    pub longest_run_duration_seconds: f64,
    pub peak_run_value: u8,
    pub total_integrated_run_intensity: f64,
    pub average_run_duration_seconds: f64,
    pub average_run_intensity: f64,
    pub shifts_by_kind: Vec<PlayerMatchImpulseShiftByKindDto>,
    pub runs: Vec<PlayerMatchImpulseRunDto>,
}