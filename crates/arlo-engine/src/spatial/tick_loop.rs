use crate::spatial::dynamic_map::DynamicSpatialMap;
use crate::spatial::kinematics::advance_position;
use crate::spatial::proximity::{calculate_distance, calculate_distance_mirim};
use crate::spatial::steering::derive_player_steered_velocity;
use arlo_domain::sport_constants::{
    MAX_OPEN_PLAY_TICKS, SPATIAL_TICK_DURATION_SECONDS, TARGET_ARRIVAL_TOLERANCE_MIRIM,
};
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Velocity};
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

    for (player, target) in movers {
        let pid = player.id();
        spatial_map.set_target(pid, *target);
        let initial_pos = spatial_map.get_position(&pid).unwrap_or_else(Position::zero);
        trajectories.insert(pid, SpatialTrajectory::new(pid, initial_pos));
    }

    let mut ticks_executed = 0;

    while ticks_executed < MAX_OPEN_PLAY_TICKS {
        let mut all_arrived = true;

        for (player, target) in movers {
            let pid = player.id();
            let current_pos = spatial_map.get_position(&pid).unwrap_or_else(Position::zero);
            let dist_mirim = calculate_distance_mirim(current_pos, *target);

            if dist_mirim > TARGET_ARRIVAL_TOLERANCE_MIRIM {
                all_arrived = false;
                let current_vel = spatial_map
                    .get_velocity(&pid)
                    .unwrap_or_else(Velocity::zero);
                let vel = derive_player_steered_velocity(
                    current_vel,
                    current_pos,
                    *target,
                    player,
                    attribute_keys,
                    1.0,
                );
                let step_dist = vel.magnitude().value() * SPATIAL_TICK_DURATION_SECONDS;
                let dist_meters = calculate_distance(current_pos, *target).value();

                let next_pos = if dist_meters <= step_dist {
                    *target
                } else {
                    advance_position(current_pos, vel, dt)
                };

                spatial_map.set_position(pid, next_pos);
                if dist_meters <= step_dist {
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
