use crate::attributes::PlayerAttributeTable;
use crate::error::EngineResult;
use crate::lineup_runtime::lineup::Lineup;
use crate::lineup_runtime::spatial_anchor::SpatialAnchorMap;
use crate::spatial::decision_vector::{
    calculate_player_speed, derive_velocity_towards_target, extract_attribute_value,
};
use crate::spatial::kinematics::advance_position;
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

    pub fn get_momentum(&self, player_id: &Uuid) -> Option<Velocity> {
        self.get_velocity(player_id)
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

    pub fn set_momentum(&mut self, player_id: Uuid, momentum: Velocity) {
        self.set_velocity(player_id, momentum);
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

    pub fn apply_momentum(&mut self, player_id: Uuid, target: Position, speed: Speed) {
        if let Some(&current_pos) = self.positions.get(&player_id) {
            let vel = derive_velocity_towards_target(current_pos, target, speed);
            self.velocities.insert(player_id, vel);
        }
    }

    pub fn apply_breakthrough_momentum(
        &mut self,
        player: &Player,
        target: Position,
        net_advantage: f64,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        fatigue_multiplier: f64,
    ) {
        let pid = player.id();
        if let Some(&current_pos) = self.positions.get(&pid) {
            let table = PlayerAttributeTable::from_player(player, attribute_keys);
            let base_speed = calculate_player_speed(player, attribute_keys, fatigue_multiplier);
            let balance = extract_attribute_value(&table, AttributeKey::Balance);
            let pace = extract_attribute_value(&table, AttributeKey::Pace);
            let momentum_boost =
                ((net_advantage * 0.08) + ((balance - 10.0) * 0.02) + ((pace - 10.0) * 0.02))
                    .clamp(0.0, 0.75);
            let boosted_speed = Speed::new(base_speed.value() * (1.0 + momentum_boost));
            let vel = derive_velocity_towards_target(current_pos, target, boosted_speed);
            self.velocities.insert(pid, vel);
        }
    }

    pub fn momentum_magnitude(&self, player_id: &Uuid) -> f64 {
        self.velocities
            .get(player_id)
            .map(|v| v.magnitude().value())
            .unwrap_or(0.0)
    }

    pub fn has_momentum(&self, player_id: &Uuid) -> bool {
        self.momentum_magnitude(player_id) > 1e-4
    }

    pub fn decay_momentum(&mut self, player_id: &Uuid, decay_factor: f64) {
        if let Some(vel) = self.velocities.get_mut(player_id) {
            let factor = decay_factor.clamp(0.0, 1.0);
            *vel = Velocity::from_raw(vel.raw() * factor);
        }
    }

    pub fn reset_velocity(&mut self, player_id: &Uuid) {
        self.velocities.insert(*player_id, Velocity::zero());
    }

    pub fn clear_all_velocities(&mut self) {
        for vel in self.velocities.values_mut() {
            *vel = Velocity::zero();
        }
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
            let vel = derive_velocity_towards_target(current_pos, target_pos, speed);
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
                let speed = calculate_player_speed(player, attribute_keys, 1.0);
                let vel = derive_velocity_towards_target(current_pos, target_pos, speed);
                self.velocities.insert(pid, vel);
            }
        }
    }
}