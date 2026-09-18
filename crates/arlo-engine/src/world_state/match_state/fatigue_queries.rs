use crate::physical::FatigueState;
use crate::world_state::match_state::fatigue::FatigueLookup;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::Player;
use std::collections::HashMap;
use uuid::Uuid;

impl MatchState {
    pub fn home_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        self.fatigue.home_fatigue()
    }

    pub fn away_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        self.fatigue.away_fatigue()
    }

    pub fn fatigue_for(&self, player_id: &Uuid) -> FatigueState {
        self.fatigue.fatigue_for(player_id)
    }

    pub fn fatigue_lookup(&self) -> FatigueLookup<'_> {
        self.fatigue.lookup()
    }

    pub fn substitute_fatigue_player(&mut self, outgoing: Uuid, incoming: Uuid, is_home: bool) {
        self.fatigue.substitute_player(outgoing, incoming, is_home);
    }

    pub fn apply_event_energy_decay(
        &mut self,
        player_id: Uuid,
        live_duration_seconds: f64,
    ) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let age_years = self
            .teams
            .find_player(&player_id)
            .map(|p| crate::physical::models::aerobic::calculate_player_age(p, 0))
            .unwrap_or(25.0);
        let table = *self.attribute_table_for(&player_id);
        self.fatigue.apply_event_energy_decay(
            player_id,
            live_duration_seconds,
            is_home,
            &table,
            age_years,
        )
    }

    pub fn apply_duel_contest_strain(
        &mut self,
        player_id: Uuid,
        intensity: f64,
    ) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let table = *self.attribute_table_for(&player_id);
        self.fatigue.apply_contest_strain(player_id, intensity, is_home, &table)
    }

    pub fn apply_dead_ball_recovery(&mut self, dead_ball_seconds: f64) -> Vec<(Uuid, f64, f64)> {
        let home_players: Vec<&Player> = self
            .teams
            .home_lineup()
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();
        let away_players: Vec<&Player> = self
            .teams
            .away_lineup()
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();
        self.fatigue.apply_dead_ball_recovery(
            dead_ball_seconds,
            &home_players,
            &away_players,
            self.teams.player_attribute_tables(),
        )
    }

    pub fn player_fatigue_multiplier(&self, player_id: &Uuid) -> f64 {
        self.fatigue.player_fatigue_multiplier(player_id)
    }
}