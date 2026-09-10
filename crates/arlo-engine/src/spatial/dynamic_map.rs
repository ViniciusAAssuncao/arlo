use crate::attributes::PlayerAttributeTable;
use crate::error::EngineResult;
use crate::lineup_runtime::lineup::Lineup;
use crate::lineup_runtime::spatial_anchor::SpatialAnchorMap;
use crate::spatial::decision_vector::{
    calculate_player_speed, derive_velocity_towards_target, extract_attribute_value,
};
use crate::spatial::kinematics::advance_position;
use crate::spatial::player_slot::{
    PlayerSlot, PlayerSlotRegistry, SLOTS_PER_TEAM, TOTAL_MATCH_SLOTS,
};
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Speed, Velocity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicSpatialMap {
    registry: PlayerSlotRegistry,
    positions: [Position; TOTAL_MATCH_SLOTS],
    velocities: [Velocity; TOTAL_MATCH_SLOTS],
    targets: [Option<Position>; TOTAL_MATCH_SLOTS],
    home_team_mask: u32,
}

impl DynamicSpatialMap {
    pub fn from_anchor_map_with_registry(
        anchor_map: &SpatialAnchorMap,
        registry: PlayerSlotRegistry,
    ) -> Self {
        let mut positions = [Position::zero(); TOTAL_MATCH_SLOTS];
        let velocities = [Velocity::zero(); TOTAL_MATCH_SLOTS];
        let targets = [None; TOTAL_MATCH_SLOTS];
        let mut home_team_mask: u32 = 0;

        for (i, &player_id) in registry.slots().iter().enumerate() {
            if let Some(pos) = anchor_map.get(&player_id) {
                positions[i] = pos;
            }
            if anchor_map.is_home_player(&player_id) || i < SLOTS_PER_TEAM {
                home_team_mask |= 1 << i;
            }
        }

        Self {
            registry,
            positions,
            velocities,
            targets,
            home_team_mask,
        }
    }

    pub fn from_anchor_map(anchor_map: &SpatialAnchorMap) -> Self {
        let mut home_keys: Vec<Uuid> = anchor_map.home_anchors().keys().copied().collect();
        home_keys.sort();
        let mut away_keys: Vec<Uuid> = anchor_map.away_anchors().keys().copied().collect();
        away_keys.sort();
        let registry = PlayerSlotRegistry::from_slices(&home_keys, &away_keys);
        Self::from_anchor_map_with_registry(anchor_map, registry)
    }

    pub fn from_pitch(
        pitch: &Pitch,
        home_lineup: &Lineup,
        away_lineup: &Lineup,
    ) -> EngineResult<Self> {
        let anchor_map = SpatialAnchorMap::from_pitch(pitch, home_lineup, away_lineup)?;
        let registry = PlayerSlotRegistry::from_lineups(home_lineup, away_lineup);
        Ok(Self::from_anchor_map_with_registry(&anchor_map, registry))
    }

    pub fn registry(&self) -> &PlayerSlotRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PlayerSlotRegistry {
        &mut self.registry
    }

    pub fn home_team_mask(&self) -> u32 {
        self.home_team_mask
    }

    pub fn get_position(&self, player_id: &Uuid) -> Option<Position> {
        self.registry.slot_for(player_id).map(|s| self.positions[s.index()])
    }

    pub fn get_velocity(&self, player_id: &Uuid) -> Option<Velocity> {
        self.registry.slot_for(player_id).map(|s| self.velocities[s.index()])
    }

    pub fn get_momentum(&self, player_id: &Uuid) -> Option<Velocity> {
        self.get_velocity(player_id)
    }

    pub fn get_target(&self, player_id: &Uuid) -> Option<Position> {
        self.registry.slot_for(player_id).and_then(|s| self.targets[s.index()])
    }

    pub fn set_position(&mut self, player_id: Uuid, position: Position) {
        if let Some(s) = self.registry.slot_for(&player_id) {
            self.positions[s.index()] = position;
        }
    }

    pub fn set_velocity(&mut self, player_id: Uuid, velocity: Velocity) {
        if let Some(s) = self.registry.slot_for(&player_id) {
            self.velocities[s.index()] = velocity;
        }
    }

    pub fn set_momentum(&mut self, player_id: Uuid, momentum: Velocity) {
        self.set_velocity(player_id, momentum);
    }

    pub fn set_target(&mut self, player_id: Uuid, target: Position) {
        if let Some(s) = self.registry.slot_for(&player_id) {
            self.targets[s.index()] = Some(target);
        }
    }

    pub fn clear_target(&mut self, player_id: &Uuid) {
        if let Some(s) = self.registry.slot_for(player_id) {
            self.targets[s.index()] = None;
        }
    }

    pub fn clear_all_targets(&mut self) {
        self.targets = [None; TOTAL_MATCH_SLOTS];
    }

    pub fn is_home_player(&self, player_id: &Uuid) -> bool {
        self.registry
            .slot_for(player_id)
            .map(|s| (self.home_team_mask & (1 << s.index())) != 0)
            .unwrap_or(false)
    }

    pub fn is_away_player(&self, player_id: &Uuid) -> bool {
        self.registry
            .slot_for(player_id)
            .map(|s| (self.home_team_mask & (1 << s.index())) == 0)
            .unwrap_or(false)
    }

    pub fn substitute_player(&mut self, outgoing: Uuid, incoming: Uuid) {
        self.registry.substitute(outgoing, incoming);
    }

    pub fn apply_momentum(&mut self, player_id: Uuid, target: Position, speed: Speed) {
        if let Some(s) = self.registry.slot_for(&player_id) {
            let current_pos = self.positions[s.index()];
            let vel = derive_velocity_towards_target(current_pos, target, speed);
            self.velocities[s.index()] = vel;
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
        if let Some(s) = self.registry.slot_for(&pid) {
            let current_pos = self.positions[s.index()];
            let table = PlayerAttributeTable::from_player(player, attribute_keys);
            let base_speed = calculate_player_speed(player, attribute_keys, fatigue_multiplier);
            let balance = extract_attribute_value(&table, AttributeKey::Balance);
            let pace = extract_attribute_value(&table, AttributeKey::Pace);
            let momentum_boost =
                ((net_advantage * 0.08) + ((balance - 10.0) * 0.02) + ((pace - 10.0) * 0.02))
                    .clamp(0.0, 0.75);
            let boosted_speed = Speed::new(base_speed.value() * (1.0 + momentum_boost));
            let vel = derive_velocity_towards_target(current_pos, target, boosted_speed);
            self.velocities[s.index()] = vel;
        }
    }

    pub fn momentum_magnitude(&self, player_id: &Uuid) -> f64 {
        self.get_velocity(player_id)
            .map(|v| v.magnitude().value())
            .unwrap_or(0.0)
    }

    pub fn has_momentum(&self, player_id: &Uuid) -> bool {
        self.momentum_magnitude(player_id) > 1e-4
    }

    pub fn decay_momentum(&mut self, player_id: &Uuid, decay_factor: f64) {
        if let Some(s) = self.registry.slot_for(player_id) {
            let factor = decay_factor.clamp(0.0, 1.0);
            let vel = &mut self.velocities[s.index()];
            *vel = Velocity::from_raw(vel.raw() * factor);
        }
    }

    pub fn reset_velocity(&mut self, player_id: &Uuid) {
        if let Some(s) = self.registry.slot_for(player_id) {
            self.velocities[s.index()] = Velocity::zero();
        }
    }

    pub fn clear_all_velocities(&mut self) {
        self.velocities = [Velocity::zero(); TOTAL_MATCH_SLOTS];
    }

    pub fn tick(&mut self, dt: Duration) {
        for i in 0..TOTAL_MATCH_SLOTS {
            self.positions[i] = advance_position(self.positions[i], self.velocities[i], dt);
        }
    }

    pub fn update_player_velocity_towards_target(&mut self, player_id: &Uuid, speed: Speed) {
        if let Some(s) = self.registry.slot_for(player_id) {
            let idx = s.index();
            if let Some(target_pos) = self.targets[idx] {
                let current_pos = self.positions[idx];
                let vel = derive_velocity_towards_target(current_pos, target_pos, speed);
                self.velocities[idx] = vel;
            }
        }
    }

    pub fn update_all_velocities_towards_targets(
        &mut self,
        players: &[&Player],
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) {
        for player in players {
            let pid = player.id();
            if let Some(s) = self.registry.slot_for(&pid) {
                let idx = s.index();
                if let Some(target_pos) = self.targets[idx] {
                    let current_pos = self.positions[idx];
                    let speed = calculate_player_speed(player, attribute_keys, 1.0);
                    let vel = derive_velocity_towards_target(current_pos, target_pos, speed);
                    self.velocities[idx] = vel;
                }
            }
        }
    }

    pub fn player_ids(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.registry.slots().iter().copied()
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = (Uuid, Position)> + '_ {
        self.registry
            .slots()
            .iter()
            .copied()
            .enumerate()
            .map(move |(i, id)| (id, self.positions[i]))
    }

    pub fn iter_velocities(&self) -> impl Iterator<Item = (Uuid, Velocity)> + '_ {
        self.registry
            .slots()
            .iter()
            .copied()
            .enumerate()
            .map(move |(i, id)| (id, self.velocities[i]))
    }

    pub fn iter_targets(&self) -> impl Iterator<Item = (Uuid, Position)> + '_ {
        self.registry
            .slots()
            .iter()
            .copied()
            .enumerate()
            .filter_map(move |(i, id)| self.targets[i].map(|target| (id, target)))
    }

    #[inline]
    pub fn position_at_slot(&self, slot: PlayerSlot) -> Position {
        self.positions[slot.index()]
    }

    #[inline]
    pub fn velocity_at_slot(&self, slot: PlayerSlot) -> Velocity {
        self.velocities[slot.index()]
    }

    #[inline]
    pub fn target_at_slot(&self, slot: PlayerSlot) -> Option<Position> {
        self.targets[slot.index()]
    }

    #[inline]
    pub fn set_position_at_slot(&mut self, slot: PlayerSlot, position: Position) {
        self.positions[slot.index()] = position;
    }

    #[inline]
    pub fn set_velocity_at_slot(&mut self, slot: PlayerSlot, velocity: Velocity) {
        self.velocities[slot.index()] = velocity;
    }

    #[inline]
    pub fn set_target_at_slot(&mut self, slot: PlayerSlot, target: Option<Position>) {
        self.targets[slot.index()] = target;
    }

    #[inline]
    pub fn positions_raw(&self) -> &[Position; TOTAL_MATCH_SLOTS] {
        &self.positions
    }

    #[inline]
    pub fn velocities_raw(&self) -> &[Velocity; TOTAL_MATCH_SLOTS] {
        &self.velocities
    }

    #[inline]
    pub fn targets_raw(&self) -> &[Option<Position>; TOTAL_MATCH_SLOTS] {
        &self.targets
    }

    #[inline]
    pub fn len(&self) -> usize {
        TOTAL_MATCH_SLOTS
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl Default for DynamicSpatialMap {
    fn default() -> Self {
        Self {
            registry: PlayerSlotRegistry::default(),
            positions: [Position::zero(); TOTAL_MATCH_SLOTS],
            velocities: [Velocity::zero(); TOTAL_MATCH_SLOTS],
            targets: [None; TOTAL_MATCH_SLOTS],
            home_team_mask: 0x0000_3FFF,
        }
    }
}