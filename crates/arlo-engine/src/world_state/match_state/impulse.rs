use crate::lineup_runtime::Lineup;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::{
    calculate_player_contextual_baseline,
    find_active_captain,
};
use crate::psychology::systems::dynamics::update_player_impulse_contextual;
use crate::psychology::systems::event_bus::ImpulseEventBus;
use crate::psychology::systems::events::{
    apply_impulse_event_contextual_at,
    ImpulseEvent,
    ImpulseShift,
};
use crate::world_state::match_state::fatigue::FatigueTracker;
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::AttributeKey;
use serde::{ Deserialize, Serialize };
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
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) -> Self {
        let home_players = home_lineup.players();
        let away_players = away_lineup.players();

        let home_captain = find_active_captain(&home_players, attribute_keys);
        let away_captain = find_active_captain(&away_players, attribute_keys);

        let mut home_impulse = HashMap::with_capacity(home_lineup.len());
        for p in &home_players {
            let base = calculate_player_contextual_baseline(p, attribute_keys, home_captain, true);
            home_impulse.insert(p.id(), ImpulseState::from_baseline(base));
        }

        let mut away_impulse = HashMap::with_capacity(away_lineup.len());
        for p in &away_players {
            let base = calculate_player_contextual_baseline(p, attribute_keys, away_captain, false);
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

    pub fn home_impulse_mut(&mut self) -> &mut HashMap<Uuid, ImpulseState> {
        &mut self.home_impulse
    }

    pub fn away_impulse_mut(&mut self) -> &mut HashMap<Uuid, ImpulseState> {
        &mut self.away_impulse
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

    pub fn set_player_impulse(&mut self, player_id: Uuid, is_home: bool, state: ImpulseState) {
        if is_home {
            self.home_impulse.insert(player_id, state);
        } else {
            self.away_impulse.insert(player_id, state);
        }
    }

    pub fn substitute_player(
        &mut self,
        _outgoing: Uuid,
        incoming: Uuid,
        is_home: bool,
        teams: &TeamRegistry,
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) {
        let captain = if is_home {
            teams.home_captain(attribute_keys)
        } else {
            teams.away_captain(attribute_keys)
        };

        let map = if is_home { &mut self.home_impulse } else { &mut self.away_impulse };

        if !map.contains_key(&incoming) {
            if let Some(player) = teams.find_player(&incoming) {
                let base = calculate_player_contextual_baseline(
                    player,
                    attribute_keys,
                    captain,
                    is_home
                );
                map.insert(incoming, ImpulseState::from_baseline(base));
            }
        }
    }

    pub fn set_all_to_initial(&mut self) {
        for state in self.home_impulse.values_mut() {
            *state = ImpulseState::initial();
        }
        for state in self.away_impulse.values_mut() {
            *state = ImpulseState::initial();
        }
    }

    pub fn reset_all_to_baseline(
        &mut self,
        teams: &TeamRegistry,
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) {
        let home_captain = teams.home_captain(attribute_keys);
        let away_captain = teams.away_captain(attribute_keys);

        for p in teams.home_lineup().players() {
            let base = calculate_player_contextual_baseline(p, attribute_keys, home_captain, true);
            self.home_impulse.insert(p.id(), ImpulseState::from_baseline(base));
        }
        for p in teams.away_lineup().players() {
            let base = calculate_player_contextual_baseline(p, attribute_keys, away_captain, false);
            self.away_impulse.insert(p.id(), ImpulseState::from_baseline(base));
        }
    }

    pub fn advance_impulse_dynamics(
        &mut self,
        dt_seconds: f64,
        teams: &TeamRegistry,
        fatigue: &FatigueTracker,
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) {
        if dt_seconds <= 0.0 {
            return;
        }

        let home_captain = teams.home_captain(attribute_keys);
        let away_captain = teams.away_captain(attribute_keys);

        for player in teams.home_lineup().players() {
            let pid = player.id();
            let phys = fatigue.fatigue_for(&pid);
            if let Some(state) = self.home_impulse.get_mut(&pid) {
                update_player_impulse_contextual(
                    state,
                    player,
                    attribute_keys,
                    &phys,
                    dt_seconds,
                    home_captain,
                    true
                );
            }
        }
        for player in teams.away_lineup().players() {
            let pid = player.id();
            let phys = fatigue.fatigue_for(&pid);
            if let Some(state) = self.away_impulse.get_mut(&pid) {
                update_player_impulse_contextual(
                    state,
                    player,
                    attribute_keys,
                    &phys,
                    dt_seconds,
                    away_captain,
                    false
                );
            }
        }
    }

    pub fn apply_impulse_event(
        &mut self,
        player_id: Uuid,
        event: &ImpulseEvent,
        timestamp_seconds: f64,
        teams: &TeamRegistry,
        fatigue: &FatigueTracker,
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) -> Option<ImpulseShift> {
        let player = teams.find_player(&player_id)?;
        let is_home = teams.is_home_player(&player_id);
        let captain = if is_home {
            teams.home_captain(attribute_keys)
        } else {
            teams.away_captain(attribute_keys)
        };
        let physical_state = fatigue.fatigue_for(&player_id);
        let impulse = if is_home {
            self.home_impulse.entry(player_id).or_default()
        } else {
            self.away_impulse.entry(player_id).or_default()
        };

        let shift = apply_impulse_event_contextual_at(
            impulse,
            player,
            attribute_keys,
            &physical_state,
            event,
            timestamp_seconds,
            captain,
            is_home
        );

        Some(shift)
    }

    pub fn process_impulse_bus(
        &mut self,
        timestamp_seconds: f64,
        teams: &TeamRegistry,
        fatigue: &FatigueTracker,
        attribute_keys: &HashMap<Uuid, AttributeKey>
    ) -> Vec<(Uuid, ImpulseShift, ImpulseEvent)> {
        let events = self.impulse_bus.drain_events();
        let mut shifts = Vec::with_capacity(events.len());
        for dispatched in events {
            if dispatched.target_id == teams.home_team_id() {
                let home_player_ids: Vec<Uuid> = teams
                    .home_lineup()
                    .players()
                    .iter()
                    .map(|p| p.id())
                    .collect();
                for pid in home_player_ids {
                    if
                        let Some(shift) = self.apply_impulse_event(
                            pid,
                            &dispatched.event,
                            timestamp_seconds,
                            teams,
                            fatigue,
                            attribute_keys
                        )
                    {
                        shifts.push((pid, shift, dispatched.event));
                    }
                }
            } else if dispatched.target_id == teams.away_team_id() {
                let away_player_ids: Vec<Uuid> = teams
                    .away_lineup()
                    .players()
                    .iter()
                    .map(|p| p.id())
                    .collect();
                for pid in away_player_ids {
                    if
                        let Some(shift) = self.apply_impulse_event(
                            pid,
                            &dispatched.event,
                            timestamp_seconds,
                            teams,
                            fatigue,
                            attribute_keys
                        )
                    {
                        shifts.push((pid, shift, dispatched.event));
                    }
                }
            } else if
                let Some(shift) = self.apply_impulse_event(
                    dispatched.target_id,
                    &dispatched.event,
                    timestamp_seconds,
                    teams,
                    fatigue,
                    attribute_keys
                )
            {
                shifts.push((dispatched.target_id, shift, dispatched.event));
            }
        }
        shifts
    }
}
