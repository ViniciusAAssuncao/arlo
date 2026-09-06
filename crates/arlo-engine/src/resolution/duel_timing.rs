use crate::spatial::proximity::calculate_distance;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::sport_constants::MINIMUM_ENGAGEMENT_SECONDS;
use arlo_domain::Player;
use arlo_math::units::{Duration, Length, Position, Speed};

pub fn time_to_close(distance: Length, speed_a: Speed, speed_b: Speed) -> Option<Duration> {
    let total_speed = speed_a.value() + speed_b.value();
    if total_speed <= 0.0 {
        None
    } else {
        Some(Duration::new(distance.value() / total_speed))
    }
}

pub fn derive_duel_duration(
    attacker_pos: Position,
    attacker_speed: Speed,
    defender_pos: Position,
    defender_speed: Speed,
) -> Duration {
    let distance = calculate_distance(attacker_pos, defender_pos);
    let closing_time =
        time_to_close(distance, attacker_speed, defender_speed).unwrap_or(Duration::new(0.0));
    Duration::new(MINIMUM_ENGAGEMENT_SECONDS + closing_time.value())
}

pub fn nearest_opponent<'a>(
    reference_pos: Position,
    candidates: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
) -> Option<(&'a Player, Position)> {
    candidates
        .iter()
        .filter_map(|&p| spatial_map.get_position(&p.id()).map(|pos| (p, pos)))
        .min_by(|(_, pos_a), (_, pos_b)| {
            let dist_a = calculate_distance(reference_pos, *pos_a).value();
            let dist_b = calculate_distance(reference_pos, *pos_b).value();
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}
