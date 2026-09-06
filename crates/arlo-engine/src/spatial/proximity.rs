use arlo_domain::sport_constants::{MINIMUM_ENGAGEMENT_SECONDS, PROXIMITY_CONTEST_RADIUS_MIRIM};
use arlo_domain::Player;
use arlo_math::units::{Duration, Length, Position, Speed, Velocity, MIRIM_TO_METERS};
use uuid::Uuid;

pub fn calculate_distance(pos_a: Position, pos_b: Position) -> Length {
    let delta = pos_a.raw() - pos_b.raw();
    Length::new(delta.magnitude())
}

pub fn calculate_distance_mirim(pos_a: Position, pos_b: Position) -> f64 {
    calculate_distance(pos_a, pos_b).value() / MIRIM_TO_METERS
}

pub fn is_within_proximity_mirim(pos_a: Position, pos_b: Position, radius_mirim: f64) -> bool {
    let dist_mirim = calculate_distance_mirim(pos_a, pos_b);
    dist_mirim <= radius_mirim
}

pub fn is_in_contest_range(pos_a: Position, pos_b: Position) -> bool {
    is_within_proximity_mirim(pos_a, pos_b, PROXIMITY_CONTEST_RADIUS_MIRIM)
}

pub fn calculate_time_to_direct_intercept(
    pursuer_pos: Position,
    pursuer_speed: Speed,
    target_pos: Position,
) -> Option<Duration> {
    if pursuer_speed.value() <= 0.0 {
        return None;
    }
    let dist = calculate_distance(pursuer_pos, target_pos);
    Some(Duration::new(dist.value() / pursuer_speed.value()))
}

pub fn calculate_time_to_moving_intercept(
    pursuer_pos: Position,
    pursuer_speed: Speed,
    target_pos: Position,
    target_vel: Velocity,
) -> Option<Duration> {
    let s_p = pursuer_speed.value();
    if s_p <= 0.0 {
        return None;
    }

    let d = target_pos.raw() - pursuer_pos.raw();
    let v = target_vel.raw();

    let a = v.dot(&v) - (s_p * s_p);
    let b = 2.0 * d.dot(&v);
    let c = d.dot(&d);

    if a.abs() < 1e-9 {
        if b.abs() < 1e-9 {
            if c.abs() < 1e-9 {
                return Some(Duration::new(0.0));
            }
            return None;
        }
        let t = -c / b;
        if t >= 0.0 {
            return Some(Duration::new(t));
        }
        return None;
    }

    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }

    let sqrt_disc = disc.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);

    let mut valid_times = Vec::with_capacity(2);
    if t1 >= 0.0 {
        valid_times.push(t1);
    }
    if t2 >= 0.0 {
        valid_times.push(t2);
    }

    valid_times.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));

    valid_times.first().copied().map(Duration::new)
}

pub fn is_within_collision_radius(
    pos_a: Position,
    speed_a: Speed,
    pos_b: Position,
    speed_b: Speed,
) -> bool {
    let dist = calculate_distance(pos_a, pos_b).value();
    let collision_radius = (speed_a.value() + speed_b.value()) * MINIMUM_ENGAGEMENT_SECONDS;
    dist < collision_radius
}

pub fn filter_active_duelists(
    epicenter: Position,
    primary_speed: Speed,
    candidates: &[(&Player, Position, Speed)],
) -> Vec<Uuid> {
    candidates
        .iter()
        .filter(|(_, pos, speed)| {
            is_within_collision_radius(epicenter, primary_speed, *pos, *speed)
        })
        .map(|(player, _, _)| player.id())
        .collect()
}

pub fn filter_active_duelists_by_id(
    epicenter: Position,
    primary_speed: Speed,
    candidates: &[(Uuid, Position, Speed)],
) -> Vec<Uuid> {
    candidates
        .iter()
        .filter(|(_, pos, speed)| {
            is_within_collision_radius(epicenter, primary_speed, *pos, *speed)
        })
        .map(|(id, _, _)| *id)
        .collect()
}

pub fn active_duelists(
    epicenter: Position,
    primary_speed: Speed,
    candidates: &[(&Player, Position, Speed)],
) -> Vec<Uuid> {
    filter_active_duelists(epicenter, primary_speed, candidates)
}