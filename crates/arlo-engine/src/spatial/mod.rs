pub mod ball_kinematics;
pub mod decision_vector;
pub mod dynamic_map;
pub mod kinematics;
pub mod proximity;
pub mod tick_loop;

pub use ball_kinematics::{
    ball_flight_duration, calculate_cross_speed, calculate_pass_speed, calculate_shot_speed,
};
pub use decision_vector::{
    calculate_player_speed, derive_player_velocity_towards_target,
    derive_velocity_towards_target, extract_attribute_value, ACCELERATION_SPEED_SCALE,
    BASE_SPRINT_SPEED_METERS_PER_SEC, PACE_SPEED_SCALE,
};
pub use dynamic_map::DynamicSpatialMap;
pub use kinematics::{advance_position, calculate_displacement};
pub use proximity::{
    calculate_distance, calculate_distance_mirim, calculate_time_to_direct_intercept,
    calculate_time_to_moving_intercept, is_in_contest_range, is_within_proximity_mirim,
};
pub use tick_loop::{
    run_spatial_tick_loop, SpatialTrajectory, TickSimulationResult,
};
