use crate::physical::FatigueState;
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

    pub fn substitute_fatigue_player(&mut self, outgoing: Uuid, incoming: Uuid, is_home: bool) {
        self.fatigue.substitute_player(outgoing, incoming, is_home);
    }

    pub fn record_distance(&mut self, player_id: Uuid, mirim: f64) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let player = self.teams.find_player(&player_id);
        self.fatigue
            .record_distance(player_id, mirim, is_home, player, &self.attribute_keys)
    }

    pub fn apply_duel_anaerobic_cost(
        &mut self,
        player_id: Uuid,
        duration_seconds: f64,
        intensity: f64,
    ) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let player = self.teams.find_player(&player_id);
        self.fatigue.apply_duel_anaerobic_cost(
            player_id,
            duration_seconds,
            intensity,
            is_home,
            player,
            &self.attribute_keys,
        )
    }

    pub fn apply_dead_ball_recovery(&mut self, dead_ball_seconds: f64) -> Vec<(Uuid, f64, f64)> {
        let home_players = self.teams.home_lineup().players();
        let away_players = self.teams.away_lineup().players();
        self.fatigue.apply_dead_ball_recovery(
            dead_ball_seconds,
            &home_players,
            &away_players,
            &self.attribute_keys,
        )
    }

    pub fn player_fatigue_multiplier(&self, player: &Player) -> f64 {
        self.fatigue
            .player_fatigue_multiplier(player, &self.attribute_keys)
    }
}