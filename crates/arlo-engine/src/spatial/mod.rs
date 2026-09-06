pub mod ball_kinematics;
pub mod decision_vector;
pub mod dynamic_map;
pub mod influence;
pub mod interception;
pub mod kinematics;
pub mod pitch_control;
pub mod positioning_drift;
pub mod proximity;
pub mod steering;
pub mod tick_loop;

pub use ball_kinematics::{
    ball_flight_duration, calculate_cross_speed, calculate_pass_speed, calculate_shot_speed,
};
pub use decision_vector::{
    calculate_player_speed, derive_player_velocity_towards_target,
    derive_velocity_towards_target, extract_attribute_value,
};
pub use dynamic_map::DynamicSpatialMap;
pub use influence::{
    calculate_point_resistance, calculate_spatial_resistance,
    calculate_spatial_resistance_between, defender_projected_mean, defender_variance,
    find_next_artro_position,
};
pub use interception::{
    calculate_defender_tti, filter_kinematic_active_duelists,
    identify_kinematic_lead_defender, identify_kinematic_lead_defender_with_drift,
};
pub use kinematics::{advance_position, calculate_displacement};
pub use pitch_control::{
    build_player_voronoi_site, build_team_voronoi_sites,
    calculate_artro_advance_pitch_control, calculate_kinematic_pitch_control,
    calculate_point_pitch_control_players,
};
pub use positioning_drift::{
    anchor_drift_radius_mirim, apply_positioning_drift, get_drifted_defender_position,
    nearest_drifted_opponent,
};
pub use proximity::{
    active_duelists, calculate_distance, calculate_distance_mirim,
    calculate_time_to_direct_intercept, calculate_time_to_moving_intercept,
    filter_active_duelists, filter_active_duelists_by_id,
    filter_active_duelists_by_id_swept, filter_active_duelists_swept,
    is_in_contest_range, is_within_collision_radius, is_within_proximity_mirim,
};
pub use steering::{
    calculate_boid_steering_velocity, calculate_dynamic_boid_steering_velocity,
    calculate_dynamic_separation_force, calculate_seek_force, calculate_separation_force,
    calculate_steered_velocity, derive_arrival_slowing_radius, derive_braking_deceleration,
    derive_dynamic_separation_radius, derive_player_arrival_radius,
    derive_player_boid_steered_velocity, derive_player_dynamic_boid_steered_velocity,
    derive_player_physical_radius, derive_player_steered_velocity, max_turn_radians_per_tick,
    SpatialNeighbor,
};
pub use tick_loop::{
    run_spatial_tick_loop, SpatialTrajectory, TickSimulationResult,
};