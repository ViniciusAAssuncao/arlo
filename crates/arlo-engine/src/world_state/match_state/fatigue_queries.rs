use crate::attributes::DEFAULT_PLAYER_ATTRIBUTE_TABLE;
use crate::physical::FatigueState;
use crate::world_state::match_state::fatigue::FatigueLookup;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::{Player, Position};
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

    pub fn set_player_fatigue(&mut self, player_id: Uuid, state: FatigueState) {
        let is_home = self.teams.is_home_player(&player_id);
        self.fatigue.set_player_fatigue(player_id, is_home, state);
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
        participated: bool,
    ) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let team_id = if is_home {
            self.home_team_id()
        } else {
            self.away_team_id()
        };
        let age_years = self
            .teams
            .find_player(&player_id)
            .map(|p| crate::physical::models::age::calculate_player_age(p, self.match_date_unix_seconds))
            .unwrap_or(25.0);

        let position = self
            .offensive_position_index_for_team(team_id)
            .get(&player_id)
            .copied()
            .unwrap_or(Position::CenterOffense);

        let tempo_mult = crate::team_identity::tempo::effort_multiplier(
            self.instructions_for_team(team_id).in_possession().tempo(),
        );
        let pressing_mult = crate::team_identity::pressing::contest_radius_multiplier(
            self.instructions_for_team(team_id).out_of_possession().pressing_intensity(),
        );

        let table = self
            .teams
            .player_attribute_table(&player_id)
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);

        let energy_tuning = *self.tuning.energy_tuning();

        self.fatigue.apply_event_energy_decay(
            player_id,
            live_duration_seconds,
            is_home,
            table,
            age_years,
            position,
            tempo_mult,
            pressing_mult,
            participated,
            &energy_tuning,
        )
    }

    pub fn apply_duel_contest_strain(
        &mut self,
        player_id: Uuid,
        intensity: f64,
    ) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let table = self
            .teams
            .player_attribute_table(&player_id)
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        self.fatigue.apply_contest_strain(player_id, intensity, is_home, table)
    }

    pub fn apply_dead_ball_recovery(
        &mut self,
        dead_ball_seconds: f64,
        is_time_call: bool,
    ) -> Vec<(Uuid, f64, f64)> {
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
        let energy_tuning = *self.tuning.energy_tuning();
        self.fatigue.apply_dead_ball_recovery(
            dead_ball_seconds,
            &home_players,
            &away_players,
            self.teams.player_attribute_tables(),
            is_time_call,
            &energy_tuning,
        )
    }

    pub fn player_fatigue_multiplier(&self, player_id: &Uuid) -> f64 {
        self.fatigue.player_fatigue_multiplier(player_id)
    }
}
