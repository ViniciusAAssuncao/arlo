use arlo_domain::PitchZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScoringSituation {
    pub zone: PitchZone,
    pub normalized_proximity: f64,
    pub drives_in_series: u32,
    pub territory_advance_mirim: f64,
    pub finisher_rating: f64,
    pub goalguard_rating: f64,
    pub defense_closed: bool,
}

impl ScoringSituation {
    pub fn new(
        zone: PitchZone,
        normalized_proximity: f64,
        drives_in_series: u32,
        territory_advance_mirim: f64,
        finisher_rating: f64,
        goalguard_rating: f64,
        defense_closed: bool,
    ) -> Self {
        Self {
            zone,
            normalized_proximity,
            drives_in_series,
            territory_advance_mirim,
            finisher_rating,
            goalguard_rating,
            defense_closed,
        }
    }
}
