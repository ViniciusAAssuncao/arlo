use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct PlayerConditionRow {
    pub player_id: String,
    pub energy_level: f64,
    pub anaerobic_reserve: f64,
    pub impulse_current_value: i32,
    pub impulse_baseline: f64,
    pub conditioning_score: f64,
    pub last_updated_year: i64,
    pub last_updated_day_of_year: i32,
    pub last_match_year: Option<i64>,
    pub last_match_day_of_year: Option<i32>,
}

impl PlayerConditionRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        player_id: Uuid,
        energy_level: f64,
        anaerobic_reserve: f64,
        impulse_current_value: u8,
        impulse_baseline: f64,
        conditioning_score: f64,
        last_updated_year: i64,
        last_updated_day_of_year: u32,
        last_match_year: Option<i64>,
        last_match_day_of_year: Option<u32>,
    ) -> Self {
        Self {
            player_id: player_id.to_string(),
            energy_level,
            anaerobic_reserve,
            impulse_current_value: impulse_current_value as i32,
            impulse_baseline,
            conditioning_score,
            last_updated_year,
            last_updated_day_of_year: last_updated_day_of_year as i32,
            last_match_year,
            last_match_day_of_year: last_match_day_of_year.map(|d| d as i32),
        }
    }
}
