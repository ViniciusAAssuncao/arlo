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
use arlo_math::units::{Duration, Position, Speed, Velocity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialTrajectory {
    player_id: Uuid,
    positions: Vec<Position>,
}

impl SpatialTrajectory {
    pub fn new(player_id: Uuid, initial_position: Position) -> Self {
        Self {
            player_id,
            positions: vec![initial_position],
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn positions(&self) -> &[Position] {
        &self.positions
    }

    pub fn push(&mut self, position: Position) {
        self.positions.push(position);
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

    pub fn total_distance_mirim(&self) -> f64 {
        self.segments()
            .iter()
            .map(|(a, b)| calculate_distance_mirim(*a, *b))
            .sum()
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

    struct MoverProps {
        speed: Speed,
        agility: f64,
        acceleration: f64,
        balance: f64,
        physical_radius: f64,
    }

    let mut mover_props = HashMap::with_capacity(movers.len());

    for (player, target) in movers {
        let pid = player.id();
        spatial_map.set_target(pid, *target);
        let initial_pos = spatial_map.get_position(&pid).unwrap_or_else(Position::zero);
        trajectories.insert(pid, SpatialTrajectory::new(pid, initial_pos));

        let speed = crate::spatial::decision_vector::calculate_player_speed(player, attribute_keys, 1.0);
        let agility = crate::spatial::decision_vector::extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
        let acceleration = crate::spatial::decision_vector::extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
        let balance = crate::spatial::decision_vector::extract_attribute_value(player, attribute_keys, AttributeKey::Balance);
        let physical_radius = derive_player_physical_radius(player, attribute_keys);

        mover_props.insert(pid, MoverProps {
            speed,
            agility,
            acceleration,
            balance,
            physical_radius,
        });
    }

    let mut physical_radii = HashMap::with_capacity(spatial_map.positions().len());
    for (&id, _) in spatial_map.positions() {
        let radius = if let Some(props) = mover_props.get(&id) {
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
                let props = &mover_props[&pid];

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
                    1.0,
                    props.physical_radius,
                    dt,
                );

                let step_dist = vel.magnitude().value() * SPATIAL_TICK_DURATION_SECONDS;
                let dist_meters = calculate_distance(current_pos, *target).value();

                let next_pos = if dist_meters <= step_dist || dist_mirim <= TARGET_ARRIVAL_TOLERANCE_MIRIM {
                    *target
                } else {
                    advance_position(current_pos, vel, dt)
                };

                spatial_map.set_position(pid, next_pos);
                if dist_meters <= step_dist || dist_mirim <= TARGET_ARRIVAL_TOLERANCE_MIRIM {
                    spatial_map.set_velocity(pid, Velocity::zero());
                } else {
                    spatial_map.set_velocity(pid, vel);
                }

                if let Some(traj) = trajectories.get_mut(&pid) {
                    traj.push(next_pos);
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