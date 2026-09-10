use crate::psychology::state::ImpulseState;
use crate::psychology::systems::event_bus::ImpulseEventBus;
use crate::psychology::systems::events::{ImpulseEvent, ImpulseShift};
use crate::world_state::match_state::state::MatchState;
use std::collections::HashMap;
use uuid::Uuid;

impl MatchState {
    pub fn home_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        self.impulse.home_impulse()
    }

    pub fn away_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        self.impulse.away_impulse()
    }

    pub fn home_impulse_mut(&mut self) -> &mut HashMap<Uuid, ImpulseState> {
        self.impulse.home_impulse_mut()
    }

    pub fn away_impulse_mut(&mut self) -> &mut HashMap<Uuid, ImpulseState> {
        self.impulse.away_impulse_mut()
    }

    pub fn set_player_impulse(&mut self, player_id: Uuid, state: ImpulseState) {
        let is_home = self.teams.is_home_player(&player_id);
        self.impulse.set_player_impulse(player_id, is_home, state);
    }

    pub fn substitute_impulse_player(&mut self, outgoing: Uuid, incoming: Uuid, is_home: bool) {
        self.impulse.substitute_player(
            outgoing,
            incoming,
            is_home,
            &self.teams,
            &self.attribute_keys,
        );
    }

    pub fn set_all_impulse_to_initial(&mut self) {
        self.impulse.set_all_to_initial();
    }

    pub fn reset_all_impulse_to_baseline(&mut self) {
        self.impulse
            .reset_all_to_baseline(&self.teams, &self.attribute_keys);
    }

    pub fn advance_impulse_dynamics(&mut self, dt_seconds: f64) {
        self.impulse.advance_impulse_dynamics(
            dt_seconds,
            &self.teams,
            &self.fatigue,
            &self.attribute_keys,
        );
    }

    pub fn impulse_bus(&self) -> &ImpulseEventBus {
        self.impulse.impulse_bus()
    }

    pub fn impulse_bus_mut(&mut self) -> &mut ImpulseEventBus {
        self.impulse.impulse_bus_mut()
    }

    pub fn impulse_for(&self, player_id: &Uuid) -> ImpulseState {
        self.impulse.impulse_for(player_id)
    }

    pub fn apply_impulse_event(
        &mut self,
        player_id: Uuid,
        event: &ImpulseEvent,
        timestamp_seconds: f64,
    ) -> Option<ImpulseShift> {
        self.impulse.apply_impulse_event(
            player_id,
            event,
            timestamp_seconds,
            &self.teams,
            &self.fatigue,
            &self.attribute_keys,
        )
    }

    pub fn process_impulse_bus(
        &mut self,
        timestamp_seconds: f64,
    ) -> Vec<(Uuid, ImpulseShift, ImpulseEvent)> {
        self.impulse.process_impulse_bus(
            timestamp_seconds,
            &self.teams,
            &self.fatigue,
            &self.attribute_keys,
        )
    }
}