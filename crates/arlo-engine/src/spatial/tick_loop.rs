use crate::physical::models::metabolic_power::{
    calculate_metabolic_work_rate, calculate_player_body_mass,
    calculate_player_critical_speed,
};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::PhysicalState;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::dynamic_map::DynamicSpatialMap;
use crate::spatial::kinematics::advance_position;
use crate::spatial::proximity::{calculate_distance, calculate_distance_mirim};
use crate::spatial::steering::{
    calculate_dynamic_boid_steering_velocity_with_id, derive_player_physical_radius,
    SpatialNeighbor,
};
use arlo_domain::sport_constants::{
    MAX_OPEN_PLAY_TICKS, SPATIAL_TICK_DURATION_SECONDS, TARGET_ARRIVAL_TOLERANCE_MIRIM,
};
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Speed, Velocity, MIRIM_TO_METERS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialTrajectory {
    player_id: Uuid,
    positions: Vec<Position>,
    distance_meters: f64,
    distance_mirim: f64,
    supramaximal_time_seconds: f64,
    metabolic_energy_joules: f64,
    peak_speed_meters_per_sec: f64,
}

impl SpatialTrajectory {
    pub fn new(player_id: Uuid, initial_position: Position) -> Self {
        Self {
            player_id,
            positions: vec![initial_position],
            distance_meters: 0.0,
            distance_mirim: 0.0,
            supramaximal_time_seconds: 0.0,
            metabolic_energy_joules: 0.0,
            peak_speed_meters_per_sec: 0.0,
        }
    }

    pub fn record_step(
        &mut self,
        new_pos: Position,
        velocity: Velocity,
        critical_speed_m_s: f64,
        mass_kg: f64,
        dt: Duration,
    ) {
        let speed = velocity.magnitude().value();
        let step_distance_m = speed * dt.value();
        let step_distance_mirim = step_distance_m / MIRIM_TO_METERS;

        self.distance_meters += step_distance_m;
        self.distance_mirim += step_distance_mirim;

        if speed > self.peak_speed_meters_per_sec {
            self.peak_speed_meters_per_sec = speed;
        }

        if speed > critical_speed_m_s {
            self.supramaximal_time_seconds += dt.value();
        }

        let metabolic_rate = calculate_metabolic_work_rate(speed, critical_speed_m_s, mass_kg, 1.0);
        self.metabolic_energy_joules += metabolic_rate * dt.value();

        self.positions.push(new_pos);
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn positions(&self) -> &[Position] {
        &self.positions
    }

    pub fn distance_meters(&self) -> f64 {
        self.distance_meters
    }

    pub fn distance_mirim(&self) -> f64 {
        self.distance_mirim
    }

    pub fn total_distance_mirim(&self) -> f64 {
        self.distance_mirim
    }

    pub fn supramaximal_time_seconds(&self) -> f64 {
        self.supramaximal_time_seconds
    }

    pub fn metabolic_energy_joules(&self) -> f64 {
        self.metabolic_energy_joules
    }

    pub fn peak_speed_meters_per_sec(&self) -> f64 {
        self.peak_speed_meters_per_sec
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    pub fn start_position(&self) -> Option<Position> {
        self.positions.first().copied()
    }

    pub fn end_position(&self) -> Option<Position> {
        self.positions.last().copied()
    }

    pub fn segments(&self) -> Vec<(Position, Position)> {
        if self.positions.len() < 2 {
            return Vec::new();
        }
        self.positions
            .windows(2)
            .map(|w| (w[0], w[1]))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TickSimulationResult {
    ticks_executed: u32,
    elapsed_seconds: f64,
    trajectories: HashMap<Uuid, SpatialTrajectory>,
}

impl TickSimulationResult {
    pub fn new(
        ticks_executed: u32,
        elapsed_seconds: f64,
        trajectories: HashMap<Uuid, SpatialTrajectory>,
    ) -> Self {
        Self {
            ticks_executed,
            elapsed_seconds,
            trajectories,
        }
    }

    pub fn ticks_executed(&self) -> u32 {
        self.ticks_executed
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed_seconds
    }

    pub fn trajectories(&self) -> &HashMap<Uuid, SpatialTrajectory> {
        &self.trajectories
    }

    pub fn get_trajectory(&self, player_id: &Uuid) -> Option<&SpatialTrajectory> {
        self.trajectories.get(player_id)
    }
}

pub fn run_spatial_tick_loop(
    spatial_map: &mut DynamicSpatialMap,
    movers: &[(&Player, Position)],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> TickSimulationResult {
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
        physical_radius: f64,
    }

    let mut mover_kinematics = HashMap::with_capacity(movers.len());

    for (player, target) in movers {
        let pid = player.id();
        spatial_map.set_target(pid, *target);
        let initial_pos = spatial_map.get_position(&pid).unwrap_or_else(Position::zero);
        trajectories.insert(pid, SpatialTrajectory::new(pid, initial_pos));

        let state = PhysicalState::initial();
        let speed = calculate_effective_player_speed(player, attribute_keys, &state);
        let critical_speed_m_s = calculate_player_critical_speed(player, attribute_keys, 0).value();
        let agility = extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
        let acceleration = extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
        let balance = extract_attribute_value(player, attribute_keys, AttributeKey::Balance);
        let strength = extract_attribute_value(player, attribute_keys, AttributeKey::Strength);
        let mass_kg = calculate_player_body_mass(player, attribute_keys);
        let physical_radius = derive_player_physical_radius(player, attribute_keys);

        mover_kinematics.insert(pid, MoverKinematics {
            speed,
            critical_speed_m_s,
            agility,
            acceleration,
            balance,
            strength,
            mass_kg,
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

    let mut ticks_executed = 0;
    let mut neighbor_snapshot = Vec::with_capacity(spatial_map.positions().len());

    while ticks_executed < MAX_OPEN_PLAY_TICKS {
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

                let vel = calculate_dynamic_boid_steering_velocity_with_id(
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
                    1.0,
                    props.physical_radius,
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
                    traj.record_step(next_pos, vel, props.critical_speed_m_s, props.mass_kg, dt);
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