use crate::world_state::match_state::availability::AvailabilityState;
use crate::world_state::match_state::state::MatchState;
use std::collections::HashSet;
use uuid::Uuid;

impl MatchState {
    pub fn is_player_available(&self, player_id: &Uuid) -> bool {
        self.availability.availability_for(player_id).is_active()
    }

    pub fn unavailable_player_ids(&self) -> HashSet<Uuid> {
        let mut set = HashSet::new();
        for (&id, &state) in self.availability.home_availability() {
            if !state.is_active() {
                set.insert(id);
            }
        }
        for (&id, &state) in self.availability.away_availability() {
            if !state.is_active() {
                set.insert(id);
            }
        }
        set
    }

    pub fn availability_for(&self, player_id: &Uuid) -> AvailabilityState {
        self.availability.availability_for(player_id)
    }

    pub fn suspend_player(&mut self, player_id: Uuid, remaining_seconds: f64) {
        let is_home = self.teams.is_home_player(&player_id);
        self.availability
            .suspend_player(player_id, is_home, remaining_seconds);
    }

    pub fn expel_player(&mut self, player_id: Uuid) {
        let is_home = self.teams.is_home_player(&player_id);
        self.availability.expel_player(player_id, is_home);
    }

    pub fn restore_player_availability(&mut self, player_id: Uuid, state: AvailabilityState) {
        let is_home = self.teams.is_home_player(&player_id);
        self.availability.restore_player(player_id, is_home, state);
    }

    pub fn substitute_availability_player(
        &mut self,
        outgoing: Uuid,
        incoming: Uuid,
        is_home: bool,
    ) {
        self.availability
            .substitute_player(outgoing, incoming, is_home);
    }

    pub fn tick_player_availability(
        &mut self,
        dt_seconds: f64,
    ) -> Vec<(Uuid, bool, AvailabilityState, AvailabilityState)> {
        self.availability.tick(dt_seconds)
    }
}