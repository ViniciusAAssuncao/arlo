use crate::error::EngineResult;
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::kinematics::advance_position;
use crate::tactics::lineup::Lineup;
use crate::tactics::spatial_anchor::SpatialAnchorMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Speed, Velocity};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicSpatialMap {
    positions: HashMap<Uuid, Position>,
    velocities: HashMap<Uuid, Velocity>,
    targets: HashMap<Uuid, Position>,
    home_team_players: HashSet<Uuid>,
    away_team_players: HashSet<Uuid>,
}

impl DynamicSpatialMap {
    pub fn from_anchor_map(anchor_map: &SpatialAnchorMap) -> Self {
        let mut positions = HashMap::with_capacity(anchor_map.len());
        let mut velocities = HashMap::with_capacity(anchor_map.len());
        let mut home_team_players = HashSet::with_capacity(anchor_map.home_anchors().len());
        let mut away_team_players = HashSet::with_capacity(anchor_map.away_anchors().len());

        for (id, &pos) in anchor_map.home_anchors() {
            positions.insert(*id, pos);
            velocities.insert(*id, Velocity::zero());
            home_team_players.insert(*id);
        }

        for (id, &pos) in anchor_map.away_anchors() {
            positions.insert(*id, pos);
            velocities.insert(*id, Velocity::zero());
            away_team_players.insert(*id);
        }

        Self {
            positions,
            velocities,
            targets: HashMap::new(),
            home_team_players,
            away_team_players,
        }
    }

    pub fn from_pitch(
        pitch: &Pitch,
        home_lineup: &Lineup,
        away_lineup: &Lineup,
    ) -> EngineResult<Self> {
        let anchor_map = SpatialAnchorMap::from_pitch(pitch, home_lineup, away_lineup)?;
        Ok(Self::from_anchor_map(&anchor_map))
    }

    pub fn get_position(&self, player_id: &Uuid) -> Option<Position> {
        self.positions.get(player_id).copied()
    }

    pub fn get_velocity(&self, player_id: &Uuid) -> Option<Velocity> {
        self.velocities.get(player_id).copied()
    }

    pub fn get_target(&self, player_id: &Uuid) -> Option<Position> {
        self.targets.get(player_id).copied()
    }

    pub fn set_position(&mut self, player_id: Uuid, position: Position) {
        self.positions.insert(player_id, position);
    }

    pub fn set_velocity(&mut self, player_id: Uuid, velocity: Velocity) {
        self.velocities.insert(player_id, velocity);
    }

    pub fn set_target(&mut self, player_id: Uuid, target: Position) {
        self.targets.insert(player_id, target);
    }

    pub fn clear_target(&mut self, player_id: &Uuid) {
        self.targets.remove(player_id);
    }

    pub fn positions(&self) -> &HashMap<Uuid, Position> {
        &self.positions
    }

    pub fn velocities(&self) -> &HashMap<Uuid, Velocity> {
        &self.velocities
    }

    pub fn targets(&self) -> &HashMap<Uuid, Position> {
        &self.targets
    }

    pub fn is_home_player(&self, player_id: &Uuid) -> bool {
        self.home_team_players.contains(player_id)
    }

    pub fn is_away_player(&self, player_id: &Uuid) -> bool {
        self.away_team_players.contains(player_id)
    }

    pub fn tick(&mut self, dt: Duration) {
        for (id, pos) in self.positions.iter_mut() {
            if let Some(&vel) = self.velocities.get(id) {
                *pos = advance_position(*pos, vel, dt);
            }
        }
    }

    pub fn update_player_velocity_towards_target(&mut self, player_id: &Uuid, speed: Speed) {
        if let (Some(&current_pos), Some(&target_pos)) =
            (self.positions.get(player_id), self.targets.get(player_id))
        {
            let vel = crate::spatial::decision_vector::derive_velocity_towards_target(
                current_pos,
                target_pos,
                speed,
            );
            self.velocities.insert(*player_id, vel);
        }
    }

    pub fn update_all_velocities_towards_targets(
        &mut self,
        players: &[&Player],
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) {
        for player in players {
            let pid = player.id();
            if let (Some(&current_pos), Some(&target_pos)) =
                (self.positions.get(&pid), self.targets.get(&pid))
            {
                let speed = calculate_player_speed(player, attribute_keys);
                let vel = crate::spatial::decision_vector::derive_velocity_towards_target(
                    current_pos,
                    target_pos,
                    speed,
                );
                self.velocities.insert(pid, vel);
            }
        }
    }
}