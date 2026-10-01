use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerReceivingStatsDto {
    pub targets: u32,
    pub receptions: u32,
    pub drops: u32,
    pub catch_rate: f64,
    pub drop_rate: f64,
    pub receiving_mirins: f64,
    pub run_after_catch_mirins: f64,
    pub longest_reception_mirim: f64,
    pub average_mirins_per_reception: f64,
}
