use crate::attributes::PlayerAttributeTable;
use crate::physical::FatigueState;
use crate::spatial::pitch_control::build_team_voronoi_sites_from_tables;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::Player;
use arlo_math::geometry::{compute_point_team_control, VoronoiSite};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_expected_free_path(
    attackers: &[VoronoiSite],
    defenders: &[VoronoiSite],
    start_pos: VectorPosition,
    direction: VectorPosition,
    max_distance_meters: f64,
    step_size_meters: f64,
) -> f64 {
    let dir_raw = direction.raw();
    let dir_mag = (dir_raw.0 * dir_raw.0 + dir_raw.1 * dir_raw.1).sqrt();
    if dir_mag <= 1e-9 || max_distance_meters <= 0.0 {
        return 0.0;
    }

    let norm_dx = dir_raw.0 / dir_mag;
    let norm_dy = dir_raw.1 / dir_mag;
    let step = if step_size_meters > 0.0 {
        step_size_meters
    } else {
        0.25 * MIRIM_TO_METERS
    };

    let control_0 =
        compute_point_team_control(attackers, defenders, start_pos.raw().0, start_pos.raw().1);
    if control_0 < 0.5 {
        return 0.0;
    }

    let mut prev_dist = 0.0;
    let mut prev_control = control_0;
    let mut current_dist = step;

    while current_dist <= max_distance_meters {
        let qx = start_pos.raw().0 + norm_dx * current_dist;
        let qy = start_pos.raw().1 + norm_dy * current_dist;
        let control = compute_point_team_control(attackers, defenders, qx, qy);

        if control < 0.5 {
            let denominator = control - prev_control;
            let t = if denominator.abs() > 1e-9 {
                ((0.5 - prev_control) / denominator).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let free_path_m = prev_dist + t * (current_dist - prev_dist);
            return free_path_m.clamp(0.0, max_distance_meters);
        }

        prev_dist = current_dist;
        prev_control = control;
        current_dist += step;
    }

    if prev_dist < max_distance_meters {
        let qx = start_pos.raw().0 + norm_dx * max_distance_meters;
        let qy = start_pos.raw().1 + norm_dy * max_distance_meters;
        let control = compute_point_team_control(attackers, defenders, qx, qy);
        if control < 0.5 {
            let denominator = control - prev_control;
            let t = if denominator.abs() > 1e-9 {
                ((0.5 - prev_control) / denominator).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let free_path_m = prev_dist + t * (max_distance_meters - prev_dist);
            return free_path_m.clamp(0.0, max_distance_meters);
        }
    }

    max_distance_meters
}

pub fn calculate_expected_free_path_mirim(
    attackers: &[VoronoiSite],
    defenders: &[VoronoiSite],
    start_pos: VectorPosition,
    direction: VectorPosition,
    max_distance_mirim: f64,
) -> f64 {
    let max_dist_m = max_distance_mirim * MIRIM_TO_METERS;
    let step_size_m = 0.25 * MIRIM_TO_METERS;
    let free_m = calculate_expected_free_path(
        attackers,
        defenders,
        start_pos,
        direction,
        max_dist_m,
        step_size_m,
    );
    free_m / MIRIM_TO_METERS
}

pub fn calculate_expected_free_path_to_goal(
    attackers: &[VoronoiSite],
    defenders: &[VoronoiSite],
    start_pos: VectorPosition,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64 {
    let pitch_len_m = pitch.length().value();
    let center_y_m = pitch.width().value() * 0.5;
    let (goal_x_m, dist_to_goal_m) = if attacking_positive_x {
        (pitch_len_m, (pitch_len_m - start_pos.raw().0).max(0.0))
    } else {
        (0.0, start_pos.raw().0.max(0.0))
    };
    let direction = VectorPosition::from_components(
        goal_x_m - start_pos.raw().0,
        center_y_m - start_pos.raw().1,
        0.0,
    );
    let max_distance_mirim = dist_to_goal_m / MIRIM_TO_METERS;
    calculate_expected_free_path_mirim(
        attackers,
        defenders,
        start_pos,
        direction,
        max_distance_mirim,
    )
}

pub fn calculate_player_expected_free_path_from_tables<F>(
    player_pos: VectorPosition,
    attackers: &[&Player],
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fatigue_for: &F,
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> f64
where
    F: Fn(&Uuid) -> FatigueState,
{
    let att_sites = build_team_voronoi_sites_from_tables(
        attackers,
        attribute_tables,
        spatial_map,
        fatigue_for,
        0,
    );
    let def_sites = build_team_voronoi_sites_from_tables(
        defenders,
        attribute_tables,
        spatial_map,
        fatigue_for,
        1,
    );
    calculate_expected_free_path_to_goal(
        &att_sites,
        &def_sites,
        player_pos,
        pitch,
        attacking_positive_x,
    )
}

pub fn estimate_free_path_from_pitch_control(
    pitch_control_ahead: f64,
    distance_to_boundary_mirim: f64,
) -> f64 {
    let pc = pitch_control_ahead.clamp(0.0, 1.0);
    if pc < 0.5 {
        (pc / 0.5) * 2.0
    } else {
        2.0 + ((pc - 0.5) / 0.5) * (distance_to_boundary_mirim.min(15.0) - 2.0).max(0.0)
    }
}
