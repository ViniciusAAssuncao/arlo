use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::psychology::systems::event_bus::ImpulseEventBus;
use crate::psychology::systems::events::{apply_impulse_event_at, ImpulseEvent, ImpulseShift};
use crate::tactics::Lineup;
use crate::world_state::match_state::fatigue::FatigueTracker;
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::AttributeKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpulseTracker {
    home_impulse: HashMap<Uuid, ImpulseState>,
    away_impulse: HashMap<Uuid, ImpulseState>,
    impulse_bus: ImpulseEventBus,
}

impl ImpulseTracker {
    pub fn new(
        home_lineup: &Lineup,
        away_lineup: &Lineup,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Self {
        let mut home_impulse = HashMap::with_capacity(home_lineup.len());
        for p in home_lineup.players() {
            let base = calculate_player_impulse_baseline(p, attribute_keys);
            home_impulse.insert(p.id(), ImpulseState::from_baseline(base));
        }

        let mut away_impulse = HashMap::with_capacity(away_lineup.len());
        for p in away_lineup.players() {
            let base = calculate_player_impulse_baseline(p, attribute_keys);
            away_impulse.insert(p.id(), ImpulseState::from_baseline(base));
        }

        Self {
            home_impulse,
            away_impulse,
            impulse_bus: ImpulseEventBus::new(),
        }
    }

    pub fn home_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        &self.home_impulse
    }

    pub fn away_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        &self.away_impulse
    }

    pub fn impulse_bus(&self) -> &ImpulseEventBus {
        &self.impulse_bus
    }

    pub fn impulse_bus_mut(&mut self) -> &mut ImpulseEventBus {
        &mut self.impulse_bus
    }

    pub fn impulse_for(&self, player_id: &Uuid) -> ImpulseState {
        self.home_impulse
            .get(player_id)
            .or_else(|| self.away_impulse.get(player_id))
            .copied()
            .unwrap_or_default()
    }

    pub fn apply_impulse_event(
        &mut self,
        player_id: Uuid,
        event: &ImpulseEvent,
        timestamp_seconds: f64,
        teams: &TeamRegistry,
        fatigue: &FatigueTracker,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<ImpulseShift> {
        let player = teams.find_player(&player_id)?;
        let is_home = teams.is_home_player(&player_id);
        let physical_state = fatigue.fatigue_for(&player_id);
        let impulse = if is_home {
            self.home_impulse.entry(player_id).or_default()
        } else {
            self.away_impulse.entry(player_id).or_default()
        };

        let shift = apply_impulse_event_at(
            impulse,
            player,
            attribute_keys,
            &physical_state,
            event,
            timestamp_seconds,
        );

        Some(shift)
    }

    pub fn process_impulse_bus(
        &mut self,
        timestamp_seconds: f64,
        teams: &TeamRegistry,
        fatigue: &FatigueTracker,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Vec<(Uuid, ImpulseShift, ImpulseEvent)> {
        let events = self.impulse_bus.drain_events();
        let mut shifts = Vec::with_capacity(events.len());
        for dispatched in events {
            if dispatched.target_id == teams.home_team_id() {
                let home_player_ids: Vec<Uuid> =
                    teams.home_lineup().players().iter().map(|p| p.id()).collect();
                for pid in home_player_ids {
                    if let Some(shift) = self.apply_impulse_event(
                        pid,
                        &dispatched.event,
                        timestamp_seconds,
                        teams,
                        fatigue,
                        attribute_keys,
                    ) {
                        shifts.push((pid, shift, dispatched.event));
                    }
                }
            } else if dispatched.target_id == teams.away_team_id() {
                let away_player_ids: Vec<Uuid> =
                    teams.away_lineup().players().iter().map(|p| p.id()).collect();
                for pid in away_player_ids {
                    if let Some(shift) = self.apply_impulse_event(
                        pid,
                        &dispatched.event,
                        timestamp_seconds,
                        teams,
                        fatigue,
                        attribute_keys,
                    ) {
                        shifts.push((pid, shift, dispatched.event));
                    }
                }
            } else if let Some(shift) = self.apply_impulse_event(
                dispatched.target_id,
                &dispatched.event,
                timestamp_seconds,
                teams,
                fatigue,
                attribute_keys,
            ) {
                shifts.push((dispatched.target_id, shift, dispatched.event));
            }
        }
        shifts
    }
}