use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScoringRegimePolicy {
    pub goal_point_required_drives: u32,
    pub field_point_required_drives: u32,
    pub field_point_min_territory_advance_mirim: f64,
}

impl Default for ScoringRegimePolicy {
    fn default() -> Self {
        Self {
            goal_point_required_drives: arlo_domain::sport_constants::GOAL_POINT_REQUIRED_DRIVES,
            field_point_required_drives: arlo_domain::sport_constants::FIELD_POINT_REQUIRED_DRIVES,
            field_point_min_territory_advance_mirim:
                arlo_domain::sport_constants::FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM,
        }
    }
}
