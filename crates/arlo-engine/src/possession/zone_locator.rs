use arlo_domain::sport_constants::{
    AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM, FIRST_ZONE_DEPTH_MIRIM,
};
use arlo_domain::PitchZone;

pub fn locate_zone(
    normalized_proximity: f64,
    pitch_length_mirim: f64,
    second_zone_depth_mirim: f64,
) -> PitchZone {
    let clamped = normalized_proximity.clamp(0.0, 1.0);
    let distance_to_goal_mirim = (1.0 - clamped) * pitch_length_mirim.max(1.0);
    locate_zone_at_distance(distance_to_goal_mirim, second_zone_depth_mirim)
}

pub fn locate_zone_at_distance(
    distance_to_goal_mirim: f64,
    second_zone_depth_mirim: f64,
) -> PitchZone {
    if distance_to_goal_mirim <= FIRST_ZONE_DEPTH_MIRIM {
        PitchZone::FirstZone
    } else if distance_to_goal_mirim <= FIRST_ZONE_DEPTH_MIRIM + second_zone_depth_mirim {
        PitchZone::SecondZone
    } else {
        PitchZone::OpenField
    }
}

pub fn locate_zone_by_distance(
    distance_to_goal_mirim: f64,
    second_zone_depth_mirim: f64,
) -> PitchZone {
    locate_zone_at_distance(distance_to_goal_mirim, second_zone_depth_mirim)
}

pub fn locate_zone_at_progress(
    normalized_proximity: f64,
    pitch_length_mirim: f64,
    second_zone_depth_mirim: f64,
) -> PitchZone {
    locate_zone(normalized_proximity, pitch_length_mirim, second_zone_depth_mirim)
}

pub fn locate_zone_default(normalized_proximity: f64, pitch_length_mirim: f64) -> PitchZone {
    locate_zone(
        normalized_proximity,
        pitch_length_mirim,
        AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM,
    )
}
