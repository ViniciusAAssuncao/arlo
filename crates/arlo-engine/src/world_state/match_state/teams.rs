use crate::attributes::{ManagerAttributeTable, PlayerAttributeTable};
use crate::lineup_runtime::Lineup;
use crate::world_state::match_state::team_position_indices::compute_lineup_indices;
use arlo_domain::{Manager, Player, Position as DomainPosition, SlotRole};
use arlo_tactics::{PlayCall, PlayerInstructions, TeamInstructions, TeamTacticalProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRegistry {
    pub(crate) home_team_id: Uuid,
    pub(crate) away_team_id: Uuid,
    pub(crate) home_lineup: Arc<Lineup>,
    pub(crate) away_lineup: Arc<Lineup>,
    pub(crate) home_tactical_profile: TeamTacticalProfile,
    pub(crate) away_tactical_profile: TeamTacticalProfile,
    pub(crate) home_manager: Manager,
    pub(crate) away_manager: Manager,
    pub(crate) home_available_profiles: Vec<TeamTacticalProfile>,
    pub(crate) away_available_profiles: Vec<TeamTacticalProfile>,
    pub(crate) home_playbook: Vec<PlayCall>,
    pub(crate) away_playbook: Vec<PlayCall>,
    pub(crate) home_offensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    pub(crate) home_defensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    pub(crate) away_offensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    pub(crate) away_defensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    pub(crate) home_role_index: Arc<HashMap<Uuid, SlotRole>>,
    pub(crate) away_role_index: Arc<HashMap<Uuid, SlotRole>>,
    pub(crate) home_instructions_index: Arc<HashMap<Uuid, PlayerInstructions>>,
    pub(crate) away_instructions_index: Arc<HashMap<Uuid, PlayerInstructions>>,
    pub(crate) player_attribute_tables: HashMap<Uuid, PlayerAttributeTable>,
    pub(crate) home_manager_table: ManagerAttributeTable,
    pub(crate) away_manager_table: ManagerAttributeTable,
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
        player_attribute_tables: HashMap<Uuid, PlayerAttributeTable>,
        home_manager_table: ManagerAttributeTable,
        away_manager_table: ManagerAttributeTable,
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
            home_lineup: Arc::new(home_lineup),
            away_lineup: Arc::new(away_lineup),
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
            player_attribute_tables,
            home_manager_table,
            away_manager_table,
        }
    }

    pub fn replace_lineup(&mut self, team_id: Uuid, new_lineup: Lineup) {
        let (off_pos, def_pos, role_idx, instr_idx) = compute_lineup_indices(&new_lineup);
        if team_id == self.home_team_id {
            self.home_lineup = Arc::new(new_lineup);
            self.home_offensive_position_index = off_pos;
            self.home_defensive_position_index = def_pos;
            self.home_role_index = role_idx;
            self.home_instructions_index = instr_idx;
        } else {
            self.away_lineup = Arc::new(new_lineup);
            self.away_offensive_position_index = off_pos;
            self.away_defensive_position_index = def_pos;
            self.away_role_index = role_idx;
            self.away_instructions_index = instr_idx;
        }
    }

    pub fn player_attribute_tables(&self) -> &HashMap<Uuid, PlayerAttributeTable> {
        &self.player_attribute_tables
    }

    pub fn player_attribute_table(&self, player_id: &Uuid) -> Option<&PlayerAttributeTable> {
        self.player_attribute_tables.get(player_id)
    }

    pub fn insert_player_attribute_table(&mut self, player_id: Uuid, table: PlayerAttributeTable) {
        self.player_attribute_tables.insert(player_id, table);
    }

    pub fn home_manager_table(&self) -> &ManagerAttributeTable {
        &self.home_manager_table
    }

    pub fn away_manager_table(&self) -> &ManagerAttributeTable {
        &self.away_manager_table
    }

    pub fn manager_attribute_table(&self, team_id: Uuid) -> &ManagerAttributeTable {
        if team_id == self.home_team_id {
            &self.home_manager_table
        } else {
            &self.away_manager_table
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

    pub fn home_lineup_arc(&self) -> Arc<Lineup> {
        Arc::clone(&self.home_lineup)
    }

    pub fn away_lineup_arc(&self) -> Arc<Lineup> {
        Arc::clone(&self.away_lineup)
    }

    pub fn lineup_for_team_arc(&self, team_id: Uuid) -> Arc<Lineup> {
        if team_id == self.home_team_id {
            Arc::clone(&self.home_lineup)
        } else {
            Arc::clone(&self.away_lineup)
        }
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

    pub fn is_home_player(&self, player_id: &Uuid) -> bool {
        self.home_offensive_position_index.contains_key(player_id)
    }

    pub fn find_player<'a>(&'a self, player_id: &Uuid) -> Option<&'a Player> {
        self.home_lineup
            .assignments()
            .iter()
            .chain(self.away_lineup.assignments().iter())
            .map(|a| a.player())
            .find(|p| p.id() == *player_id)
    }
}
