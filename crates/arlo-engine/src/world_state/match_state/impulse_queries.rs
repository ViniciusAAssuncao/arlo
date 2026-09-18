use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_captaincy_influence;
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
        let captain_id = if is_home {
            self.teams.home_captain_id()
        } else {
            self.teams.away_captain_id()
        };
        let captain_influence = captain_id
            .and_then(|id| self.teams.player_attribute_table(&id))
            .map(calculate_captaincy_influence)
            .unwrap_or(0.0);
        let is_captain = Some(incoming) == captain_id;
        let table = *self.attribute_table_for(&incoming);
        self.impulse.substitute_player(
            outgoing,
            incoming,
            is_home,
            &table,
            captain_influence,
            is_captain,
        );
    }

    pub fn set_all_impulse_to_initial(&mut self) {
        self.impulse.set_all_to_initial();
    }

    pub fn reset_all_impulse_to_baseline(&mut self) {
        self.impulse.reset_all_to_baseline(&self.teams);
    }

    pub fn advance_impulse_dynamics(&mut self, dt_seconds: f64) {
        self.impulse.advance_impulse_dynamics(
            dt_seconds,
            &self.teams,
            &self.fatigue,
        );
    }

    pub fn impulse_for(&self, player_id: &Uuid) -> ImpulseState {
        self.impulse.impulse_for(player_id)
    }

    pub fn apply_impulse_event(
        &mut self,
        player_id: Uuid,
        event: &ImpulseEvent,
    ) -> Option<ImpulseShift> {
        let is_home = self.teams.is_home_player(&player_id);
        let score_deficit = if is_home {
            (self.scoreboard.away_score().total_points as i32)
                - (self.scoreboard.home_score().total_points as i32)
        } else {
            (self.scoreboard.home_score().total_points as i32)
                - (self.scoreboard.away_score().total_points as i32)
        };
        self.impulse.apply_impulse_event(
            player_id,
            event,
            &self.teams,
            &self.fatigue,
            score_deficit,
        )
    }
}