use crate::physical::models::metabolic_power::{
    calculate_desired_cruise_speed, calculate_player_body_mass, calculate_player_critical_speed,
};
use crate::physical::systems::degradation::physical_attribute_modifier;
use crate::physical::systems::pacing::calculate_player_pacing_state_with_effort_and_impulse;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::dynamic_map::DynamicSpatialMap;
use crate::spatial::kinematics::advance_position;
use crate::spatial::movement_context::MovementContext;
use crate::spatial::proximity::{calculate_distance, calculate_distance_mirim};
use crate::spatial::steering::{
    calculate_dynamic_boid_steering_velocity_with_context, derive_player_physical_radius,
    SpatialNeighbor,
};
use crate::spatial::trajectory::{SpatialTrajectory, TickSimulationResult};
use crate::team_identity::tempo::effort_multiplier_from_value;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{
    MAX_OPEN_PLAY_TICKS, SPATIAL_TICK_DURATION_SECONDS, TARGET_ARRIVAL_TOLERANCE_MIRIM,
};
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Speed, Velocity};
use std::collections::HashMap;
use uuid::Uuid;

pub fn run_spatial_tick_loop(
    spatial_map: &mut DynamicSpatialMap,
    movers: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
) -> TickSimulationResult {
    run_spatial_tick_loop_with_context(
        spatial_map,
        movers,
        attribute_keys,
        MovementContext::LivePlay,
        pitch,
        &|_| PhysicalState::initial(),
        0.0,
        0.5,
        true,
    )
}

pub fn run_spatial_tick_loop_with_fatigue<F>(
    spatial_map: &mut DynamicSpatialMap,
    movers: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    fatigue_for: &F,
) -> TickSimulationResult
where
    F: Fn(&Uuid) -> PhysicalState,
{
    run_spatial_tick_loop_with_context(
        spatial_map,
        movers,
        attribute_keys,
        MovementContext::LivePlay,
        pitch,
        fatigue_for,
        0.0,
        0.5,
        true,
    )
}

pub fn run_spatial_tick_loop_with_context<F>(
    spatial_map: &mut DynamicSpatialMap,
    movers: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    movement_context: MovementContext,
    pitch: &Pitch,
    fatigue_for: &F,
    offense_tempo_value: f64,
    defense_pressing_value: f64,
    is_home_offense: bool,
) -> TickSimulationResult
where
    F: Fn(&Uuid) -> PhysicalState,
{
    if movers.is_empty() {
        return TickSimulationResult::new(0, 0.0, HashMap::new());
    }

    let dt = Duration::new(SPATIAL_TICK_DURATION_SECONDS);
    let mut trajectories: HashMap<Uuid, SpatialTrajectory> = HashMap::with_capacity(movers.len());

    struct MoverKinematics {
        speed: Speed,
        critical_speed_m_s: f64,
        agility: f64,
        acceleration: f64,
        balance: f64,
        strength: f64,
        mass_kg: f64,
        fatigue_multiplier: f64,
        physical_radius: f64,
    }

    let mut mover_kinematics = HashMap::with_capacity(movers.len());

    for (player, target) in movers {
        let pid = player.id();
        spatial_map.set_target(pid, *target);
        let initial_pos = spatial_map.get_position(&pid).unwrap_or_else(Position::zero);
        trajectories.insert(pid, SpatialTrajectory::new(pid, initial_pos));

        let state = fatigue_for(&pid);
        let critical_speed_m_s = calculate_player_critical_speed(player, attribute_keys, 0).value();
        let work_rate = extract_attribute_value(player, attribute_keys, AttributeKey::WorkRate);
        let positioning = extract_attribute_value(player, attribute_keys, AttributeKey::Positioning);
        let agility = extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
        let acceleration = extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
        let balance = extract_attribute_value(player, attribute_keys, AttributeKey::Balance);
        let strength = extract_attribute_value(player, attribute_keys, AttributeKey::Strength);
        let mass_kg = calculate_player_body_mass(player, attribute_keys);
        let physical_radius = derive_player_physical_radius(player, attribute_keys);

        let is_player_home = spatial_map.is_home_player(&pid);
        let is_player_offense = is_player_home == is_home_offense;
        let effort_mult = if is_player_offense {
            effort_multiplier_from_value(offense_tempo_value)
        } else {
            effort_multiplier_from_value(defense_pressing_value)
        };

        let dist_mirim = calculate_distance_mirim(initial_pos, *target);
        let is_near_ball = dist_mirim <= TARGET_ARRIVAL_TOLERANCE_MIRIM * 4.0;
        let baseline = calculate_player_impulse_baseline(player, attribute_keys);
        let impulse = ImpulseState::from_baseline(baseline);
        let pressure = GameStatePressure::default();

        let pacing_state = calculate_player_pacing_state_with_effort_and_impulse(
            player,
            attribute_keys,
            is_near_ball,
            &pressure,
            &state,
            &impulse,
            effort_mult,
            0,
        );

        let speed = match movement_context {
            MovementContext::LivePlay => pacing_state.target_cruise_speed(),
            MovementContext::DeadBall => {
                let base_cruise = calculate_desired_cruise_speed(critical_speed_m_s, work_rate, positioning);
                let phys_mod = physical_attribute_modifier(&state);
                let cruise_val = (base_cruise * phys_mod).clamp(0.5, critical_speed_m_s);
                Speed::new(cruise_val)
            }
        };

        mover_kinematics.insert(pid, MoverKinematics {
            speed,
            critical_speed_m_s,
            agility,
            acceleration,
            balance,
            strength,
            mass_kg,
            fatigue_multiplier: state.energy(),
            physical_radius,
        });
    }

    let mut physical_radii = HashMap::with_capacity(spatial_map.positions().len());
    for (&id, _) in spatial_map.positions() {
        let radius = if let Some(props) = mover_kinematics.get(&id) {
            props.physical_radius
        } else {
            0.55
        };
        physical_radii.insert(id, radius);
    }

    let max_ticks = match movement_context {
        MovementContext::LivePlay => MAX_OPEN_PLAY_TICKS,
        MovementContext::DeadBall => 600,
    };

    let mut ticks_executed = 0;
    let mut neighbor_snapshot = Vec::with_capacity(spatial_map.positions().len());

    while ticks_executed < max_ticks {
        let mut all_arrived = true;

        neighbor_snapshot.clear();
        for (&id, &pos) in spatial_map.positions() {
            let vel = spatial_map
                .get_velocity(&id)
                .unwrap_or_else(Velocity::zero);
            let is_home = spatial_map.is_home_player(&id);
            let physical_radius = *physical_radii.get(&id).unwrap_or(&0.55);
            neighbor_snapshot.push(SpatialNeighbor::new(id, pos, vel, is_home, physical_radius));
        }

        for (player, target) in movers {
            let pid = player.id();
            let current_pos = spatial_map.get_position(&pid).unwrap_or_else(Position::zero);
            let dist_mirim = calculate_distance_mirim(current_pos, *target);

            if dist_mirim > TARGET_ARRIVAL_TOLERANCE_MIRIM {
                all_arrived = false;
                let current_vel = spatial_map
                    .get_velocity(&pid)
                    .unwrap_or_else(Velocity::zero);

                let is_player_home = spatial_map.is_home_player(&pid);
                let props = &mover_kinematics[&pid];

                let vel = calculate_dynamic_boid_steering_velocity_with_context(
                    current_vel,
                    current_pos,
                    *target,
                    pid,
                    is_player_home,
                    &neighbor_snapshot,
                    props.speed,
                    props.agility,
                    props.acceleration,
                    props.balance,
                    props.strength,
                    props.mass_kg,
                    props.fatigue_multiplier,
                    props.physical_radius,
                    movement_context,
                    dt,
                );

                let step_dist = vel.magnitude().value() * SPATIAL_TICK_DURATION_SECONDS;
                let dist_meters = calculate_distance(current_pos, *target).value();

                let (next_pos, step_vel) = if dist_meters <= step_dist || dist_mirim <= TARGET_ARRIVAL_TOLERANCE_MIRIM {
                    (*target, Velocity::zero())
                } else {
                    (advance_position(current_pos, vel, dt), vel)
                };

                spatial_map.set_position(pid, next_pos);
                spatial_map.set_velocity(pid, step_vel);

                if let Some(traj) = trajectories.get_mut(&pid) {
                    let zone = pitch.zone_at_position(next_pos);
                    traj.record_step(next_pos, vel, props.critical_speed_m_s, props.mass_kg, zone, dt);
                }
            } else {
                spatial_map.set_position(pid, *target);
                spatial_map.set_velocity(pid, Velocity::zero());
            }
        }

        if all_arrived {
            break;
        }

        ticks_executed += 1;
    }

    for (player, _) in movers {
        spatial_map.clear_target(&player.id());
    }

    let elapsed_seconds = (ticks_executed as f64) * SPATIAL_TICK_DURATION_SECONDS;

    TickSimulationResult::new(ticks_executed, elapsed_seconds, trajectories)
}
