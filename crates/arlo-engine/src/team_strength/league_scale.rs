use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LeagueStrengthScale {
    pub offense_mean: f64,
    pub offense_stddev: f64,
    pub defense_mean: f64,
    pub defense_stddev: f64,
    pub control_mean: f64,
    pub control_stddev: f64,
}

impl Default for LeagueStrengthScale {
    fn default() -> Self {
        Self {
            offense_mean: 1.0,
            offense_stddev: 0.1,
            defense_mean: 1.0,
            defense_stddev: 0.1,
            control_mean: 1.0,
            control_stddev: 0.1,
        }
    }
}
