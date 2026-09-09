use crate::lineup_runtime::Lineup;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::{Manager, Position as DomainPosition, SlotRole};
use arlo_tactics::{PlayCall, PlayerInstructions, TeamInstructions, TeamTacticalProfile};
use std::collections::HashMap;
use uuid::Uuid;

impl MatchState {
    pub fn home_team_id(&self) -> Uuid {
        self.teams.home_team_id()
    }

    pub fn away_team_id(&self) -> Uuid {
        self.teams.away_team_id()
    }

    pub fn home_lineup(&self) -> &Lineup {
        self.teams.home_lineup()
    }

    pub fn away_lineup(&self) -> &Lineup {
        self.teams.away_lineup()
    }

    pub fn home_tactical_profile(&self) -> &TeamTacticalProfile {
        self.teams.home_tactical_profile()
    }

    pub fn away_tactical_profile(&self) -> &TeamTacticalProfile {
        self.teams.away_tactical_profile()
    }

    pub fn home_manager(&self) -> &Manager {
        self.teams.home_manager()
    }

    pub fn away_manager(&self) -> &Manager {
        self.teams.away_manager()
    }

    pub fn home_available_profiles(&self) -> &[TeamTacticalProfile] {
        self.teams.home_available_profiles()
    }

    pub fn away_available_profiles(&self) -> &[TeamTacticalProfile] {
        self.teams.away_available_profiles()
    }

    pub fn available_profiles_for_team(&self, team_id: Uuid) -> &[TeamTacticalProfile] {
        self.teams.available_profiles_for_team(team_id)
    }

    pub fn home_playbook(&self) -> &[PlayCall] {
        self.teams.home_playbook()
    }

    pub fn away_playbook(&self) -> &[PlayCall] {
        self.teams.away_playbook()
    }

    pub fn playbook_for_team(&self, team_id: Uuid) -> &[PlayCall] {
        self.teams.playbook_for_team(team_id)
    }

    pub fn manager_for_team(&self, team_id: Uuid) -> &Manager {
        self.teams.manager_for_team(team_id)
    }

    pub fn tactical_profile_for_team(&self, team_id: Uuid) -> &TeamTacticalProfile {
        self.teams.tactical_profile_for_team(team_id)
    }

    pub fn activate_tactical_profile(&mut self, team_id: Uuid, profile: TeamTacticalProfile) {
        self.teams.activate_tactical_profile(team_id, profile);
    }

    pub fn home_instructions(&self) -> &TeamInstructions {
        self.teams.home_instructions()
    }

    pub fn away_instructions(&self) -> &TeamInstructions {
        self.teams.away_instructions()
    }

    pub fn instructions_for_team(&self, team_id: Uuid) -> &TeamInstructions {
        self.teams.instructions_for_team(team_id)
    }

    pub fn offense_instructions(&self) -> &TeamInstructions {
        self.instructions_for_team(self.possession.role().offense())
    }

    pub fn defense_instructions(&self) -> &TeamInstructions {
        self.instructions_for_team(self.possession.role().defense())
    }

    pub fn home_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.home_offensive_position_index()
    }

    pub fn home_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.home_defensive_position_index()
    }

    pub fn away_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.away_offensive_position_index()
    }

    pub fn away_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.away_defensive_position_index()
    }

    pub fn home_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.position_index_for_team(self.teams.home_team_id())
    }

    pub fn away_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.position_index_for_team(self.teams.away_team_id())
    }

    pub fn offensive_position_index_for_team(
        &self,
        team_id: Uuid,
    ) -> &HashMap<Uuid, DomainPosition> {
        self.teams.offensive_position_index_for_team(team_id)
    }

    pub fn defensive_position_index_for_team(
        &self,
        team_id: Uuid,
    ) -> &HashMap<Uuid, DomainPosition> {
        self.teams.defensive_position_index_for_team(team_id)
    }

    pub fn position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        self.teams
            .position_index_for_team(team_id, self.possession.role().is_offense(team_id))
    }

    pub fn home_role_index(&self) -> &HashMap<Uuid, SlotRole> {
        self.teams.home_role_index()
    }

    pub fn away_role_index(&self) -> &HashMap<Uuid, SlotRole> {
        self.teams.away_role_index()
    }

    pub fn role_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, SlotRole> {
        self.teams.role_index_for_team(team_id)
    }

    pub fn player_instructions_for(&self, player_id: &Uuid) -> PlayerInstructions {
        self.teams.player_instructions(player_id)
    }

    pub fn instructions_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, PlayerInstructions> {
        self.teams.instructions_index_for_team(team_id)
    }
}