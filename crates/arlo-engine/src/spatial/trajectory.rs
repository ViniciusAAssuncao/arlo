use crate::physical::models::metabolic_power::calculate_metabolic_work_rate;
use crate::spatial::live_collisions::LiveCollision;
use arlo_domain::PitchZone;
use arlo_math::units::{Duration, Position, Velocity, MIRIM_TO_METERS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialTrajectory {
    player_id: Uuid,
    positions: Vec<Position>,
    distance_meters: f64,
    distance_mirim: f64,
    high_intensity_distance_mirim: f64,
    low_intensity_distance_mirim: f64,
    supramaximal_time_seconds: f64,
    metabolic_energy_joules: f64,
    peak_speed_meters_per_sec: f64,
    zone_distances: HashMap<PitchZone, f64>,
}

impl SpatialTrajectory {
    pub fn new(player_id: Uuid, initial_position: Position) -> Self {
        Self {
            player_id,
            positions: vec![initial_position],
            distance_meters: 0.0,
            distance_mirim: 0.0,
            high_intensity_distance_mirim: 0.0,
            low_intensity_distance_mirim: 0.0,
            supramaximal_time_seconds: 0.0,
            metabolic_energy_joules: 0.0,
            peak_speed_meters_per_sec: 0.0,
            zone_distances: HashMap::new(),
        }
    }

    pub fn record_step(
        &mut self,
        new_pos: Position,
        velocity: Velocity,
        critical_speed_m_s: f64,
        mass_kg: f64,
        zone: PitchZone,
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
            self.high_intensity_distance_mirim += step_distance_mirim;
        } else {
            self.low_intensity_distance_mirim += step_distance_mirim;
        }

        *self.zone_distances.entry(zone).or_insert(0.0) += step_distance_mirim;

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

    pub fn high_intensity_distance_mirim(&self) -> f64 {
        self.high_intensity_distance_mirim
    }

    pub fn low_intensity_distance_mirim(&self) -> f64 {
        self.low_intensity_distance_mirim
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

    pub fn zone_distances(&self) -> &HashMap<PitchZone, f64> {
        &self.zone_distances
    }

    pub fn distance_in_zone(&self, zone: PitchZone) -> f64 {
        self.zone_distances.get(&zone).copied().unwrap_or(0.0)
    }

    pub fn primary_zone(&self) -> PitchZone {
        self.zone_distances
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(&zone, _)| zone)
            .unwrap_or(PitchZone::Central)
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
        self.positions.windows(2).map(|w| (w[0], w[1])).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TickSimulationResult {
    ticks_executed: u32,
    elapsed_seconds: f64,
    trajectories: HashMap<Uuid, SpatialTrajectory>,
    interrupted_collision: Option<LiveCollision>,
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
            interrupted_collision: None,
        }
    }

    pub fn with_collision(
        ticks_executed: u32,
        elapsed_seconds: f64,
        trajectories: HashMap<Uuid, SpatialTrajectory>,
        interrupted_collision: Option<LiveCollision>,
    ) -> Self {
        Self {
            ticks_executed,
            elapsed_seconds,
            trajectories,
            interrupted_collision,
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

    pub fn interrupted_collision(&self) -> Option<&LiveCollision> {
        self.interrupted_collision.as_ref()
    }
}