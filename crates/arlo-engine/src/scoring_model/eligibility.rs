use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
    GOAL_POINT_REQUIRED_DRIVES,
};

pub fn can_attempt_goal_point(drives_in_series: u32) -> bool {
    drives_in_series >= GOAL_POINT_REQUIRED_DRIVES
}

pub fn can_attempt_field_point(drives_in_series: u32, territory_advance_mirim: f64) -> bool {
    (territory_advance_mirim >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM && drives_in_series >= 1)
        || drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
}

pub fn can_attempt_field_goal(drives_in_series: u32, _territory_advance_mirim: f64) -> bool {
    drives_in_series >= GOAL_POINT_REQUIRED_DRIVES
}
