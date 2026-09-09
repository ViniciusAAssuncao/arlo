use crate::lineup_runtime::Lineup;
use arlo_domain::{AttributeKey, Manager, Player, Position as DomainPosition, SlotRole};
use arlo_tactics::{PlayCall, PlayerInstructions, TeamInstructions, TeamTacticalProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

fn compute_lineup_indices(
    lineup: &Lineup,
) -> (
    HashMap<Uuid, DomainPosition>,
    HashMap<Uuid, DomainPosition>,
    HashMap<Uuid, SlotRole>,
    HashMap<Uuid, PlayerInstructions>,
) {
    (
        lineup.offensive_position_index(),
        lineup.defensive_position_index(),
        lineup.role_index(),
        lineup.instructions_index(),
    )
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRegistry {
    home_team_id: Uuid,
    away_team_id: Uuid,
    home_lineup: Lineup,
    away_lineup: Lineup,
    home_tactical_profile: TeamTacticalProfile,
    away_tactical_profile: TeamTacticalProfile,
    home_manager: Manager,
    away_manager: Manager,
    home_available_profiles: Vec<TeamTacticalProfile>,
    away_available_profiles: Vec<TeamTacticalProfile>,
    home_playbook: Vec<PlayCall>,
    away_playbook: Vec<PlayCall>,
    home_offensive_position_index: HashMap<Uuid, DomainPosition>,
    home_defensive_position_index: HashMap<Uuid, DomainPosition>,
    away_offensive_position_index: HashMap<Uuid, DomainPosition>,
    away_defensive_position_index: HashMap<Uuid, DomainPosition>,
    home_role_index: HashMap<Uuid, SlotRole>,
    away_role_index: HashMap<Uuid, SlotRole>,
    home_instructions_index: HashMap<Uuid, PlayerInstructions>,
    away_instructions_index: HashMap<Uuid, PlayerInstructions>,
}

impl TeamRegistry {
    pub fn new(
        home_team_id: Uuid,
        away_team_id: Uuid,
        home_lineup: Lineup,
        away_lineup: Lineup,
        home_tactical_profile: TeamTacticalProfile,
        away_tactical_profile: TeamTacticalProfile,
        home_manager: Manager,
        away_manager: Manager,
        home_available_profiles: Vec<TeamTacticalProfile>,
        away_available_profiles: Vec<TeamTacticalProfile>,
        home_playbook: Vec<PlayCall>,
        away_playbook: Vec<PlayCall>,
    ) -> Self {
        let (
            home_offensive_position_index,
            home_defensive_position_index,
            home_role_index,
            home_instructions_index,
        ) = compute_lineup_indices(&home_lineup);
        let (
            away_offensive_position_index,
            away_defensive_position_index,
            away_role_index,
            away_instructions_index,
        ) = compute_lineup_indices(&away_lineup);

        Self {
            home_team_id,
            away_team_id,
            home_lineup,
            away_lineup,
            home_tactical_profile,
            away_tactical_profile,
            home_manager,
            away_manager,
            home_available_profiles,
            away_available_profiles,
            home_playbook,
            away_playbook,
            home_offensive_position_index,
            home_defensive_position_index,
            away_offensive_position_index,
            away_defensive_position_index,
            home_role_index,
            away_role_index,
            home_instructions_index,
            away_instructions_index,
        }
    }

    pub fn replace_lineup(&mut self, team_id: Uuid, new_lineup: Lineup) {
        let (off_pos, def_pos, role_idx, instr_idx) = compute_lineup_indices(&new_lineup);
        if team_id == self.home_team_id {
            self.home_lineup = new_lineup;
            self.home_offensive_position_index = off_pos;
            self.home_defensive_position_index = def_pos;
            self.home_role_index = role_idx;
            self.home_instructions_index = instr_idx;
        } else {
            self.away_lineup = new_lineup;
            self.away_offensive_position_index = off_pos;
            self.away_defensive_position_index = def_pos;
            self.away_role_index = role_idx;
            self.away_instructions_index = instr_idx;
        }
    }

    pub fn home_team_id(&self) -> Uuid {
        self.home_team_id
    }

    pub fn away_team_id(&self) -> Uuid {
        self.away_team_id
    }

    pub fn home_lineup(&self) -> &Lineup {
        &self.home_lineup
    }

    pub fn away_lineup(&self) -> &Lineup {
        &self.away_lineup
    }

    pub fn home_tactical_profile(&self) -> &TeamTacticalProfile {
        &self.home_tactical_profile
    }

    pub fn away_tactical_profile(&self) -> &TeamTacticalProfile {
        &self.away_tactical_profile
    }

    pub fn home_manager(&self) -> &Manager {
        &self.home_manager
    }

    pub fn away_manager(&self) -> &Manager {
        &self.away_manager
    }

    pub fn home_available_profiles(&self) -> &[TeamTacticalProfile] {
        &self.home_available_profiles
    }

    pub fn away_available_profiles(&self) -> &[TeamTacticalProfile] {
        &self.away_available_profiles
    }

    pub fn available_profiles_for_team(&self, team_id: Uuid) -> &[TeamTacticalProfile] {
        if team_id == self.home_team_id {
            &self.home_available_profiles
        } else {
            &self.away_available_profiles
        }
    }

    pub fn home_playbook(&self) -> &[PlayCall] {
        &self.home_playbook
    }

    pub fn away_playbook(&self) -> &[PlayCall] {
        &self.away_playbook
    }

    pub fn playbook_for_team(&self, team_id: Uuid) -> &[PlayCall] {
        if team_id == self.home_team_id {
            &self.home_playbook
        } else {
            &self.away_playbook
        }
    }

    pub fn manager_for_team(&self, team_id: Uuid) -> &Manager {
        if team_id == self.home_team_id {
            &self.home_manager
        } else {
            &self.away_manager
        }
    }

    pub fn tactical_profile_for_team(&self, team_id: Uuid) -> &TeamTacticalProfile {
        if team_id == self.home_team_id {
            &self.home_tactical_profile
        } else {
            &self.away_tactical_profile
        }
    }

    pub fn activate_tactical_profile(&mut self, team_id: Uuid, profile: TeamTacticalProfile) {
        if team_id == self.home_team_id {
            self.home_tactical_profile = profile;
        } else {
            self.away_tactical_profile = profile;
        }
    }

    pub fn home_instructions(&self) -> &TeamInstructions {
        self.home_tactical_profile.instructions()
    }

    pub fn away_instructions(&self) -> &TeamInstructions {
        self.away_tactical_profile.instructions()
    }

    pub fn instructions_for_team(&self, team_id: Uuid) -> &TeamInstructions {
        self.tactical_profile_for_team(team_id).instructions()
    }

    pub fn home_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.home_offensive_position_index
    }

    pub fn home_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.home_defensive_position_index
    }

    pub fn away_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.away_offensive_position_index
    }

    pub fn away_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.away_defensive_position_index
    }

    pub fn offensive_position_index_for_team(
        &self,
        team_id: Uuid,
    ) -> &HashMap<Uuid, DomainPosition> {
        if team_id == self.home_team_id {
            &self.home_offensive_position_index
        } else {
            &self.away_offensive_position_index
        }
    }

    pub fn defensive_position_index_for_team(
        &self,
        team_id: Uuid,
    ) -> &HashMap<Uuid, DomainPosition> {
        if team_id == self.home_team_id {
            &self.home_defensive_position_index
        } else {
            &self.away_defensive_position_index
        }
    }

    pub fn position_index_for_team(
        &self,
        team_id: Uuid,
        is_offense: bool,
    ) -> &HashMap<Uuid, DomainPosition> {
        if is_offense {
            self.offensive_position_index_for_team(team_id)
        } else {
            self.defensive_position_index_for_team(team_id)
        }
    }

    pub fn home_role_index(&self) -> &HashMap<Uuid, SlotRole> {
        &self.home_role_index
    }

    pub fn away_role_index(&self) -> &HashMap<Uuid, SlotRole> {
        &self.away_role_index
    }

    pub fn role_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, SlotRole> {
        if team_id == self.home_team_id {
            &self.home_role_index
        } else {
            &self.away_role_index
        }
    }

    pub fn home_instructions_index(&self) -> &HashMap<Uuid, PlayerInstructions> {
        &self.home_instructions_index
    }

    pub fn away_instructions_index(&self) -> &HashMap<Uuid, PlayerInstructions> {
        &self.away_instructions_index
    }

    pub fn instructions_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, PlayerInstructions> {
        if team_id == self.home_team_id {
            &self.home_instructions_index
        } else {
            &self.away_instructions_index
        }
    }

    pub fn player_instructions(&self, player_id: &Uuid) -> PlayerInstructions {
        self.home_instructions_index
            .get(player_id)
            .or_else(|| self.away_instructions_index.get(player_id))
            .copied()
            .unwrap_or_default()
    }

    pub fn is_home_player(&self, player_id: &Uuid) -> bool {
        self.home_offensive_position_index.contains_key(player_id)
    }

    pub fn find_player(&self, player_id: &Uuid) -> Option<&Player> {
        self.home_lineup
            .players()
            .into_iter()
            .chain(self.away_lineup.players())
            .find(|p| p.id() == *player_id)
    }

    pub fn home_captain<'a>(
        &'a self,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        let players = self.home_lineup.players();
        crate::psychology::systems::baseline::find_active_captain(&players, attribute_keys)
    }

    pub fn away_captain<'a>(
        &'a self,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        let players = self.away_lineup.players();
        crate::psychology::systems::baseline::find_active_captain(&players, attribute_keys)
    }

    pub fn team_captain<'a>(
        &'a self,
        team_id: Uuid,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        if team_id == self.home_team_id {
            self.home_captain(attribute_keys)
        } else {
            self.away_captain(attribute_keys)
        }
    }
}