use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::Lineup;
use arlo_domain::{AttributeKey, Manager, Player, Position as DomainPosition, SlotRole};
use arlo_tactics::{PlayCall, PlayerInstructions, TeamInstructions, TeamTacticalProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

fn compute_lineup_indices(
    lineup: &Lineup,
) -> (
    Arc<HashMap<Uuid, DomainPosition>>,
    Arc<HashMap<Uuid, DomainPosition>>,
    Arc<HashMap<Uuid, SlotRole>>,
    Arc<HashMap<Uuid, PlayerInstructions>>,
) {
    (
        Arc::new(lineup.offensive_position_index()),
        Arc::new(lineup.defensive_position_index()),
        Arc::new(lineup.role_index()),
        Arc::new(lineup.instructions_index()),
    )
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRegistry {
    home_team_id: Uuid,
    away_team_id: Uuid,
    home_lineup: Arc<Lineup>,
    away_lineup: Arc<Lineup>,
    home_tactical_profile: TeamTacticalProfile,
    away_tactical_profile: TeamTacticalProfile,
    home_manager: Manager,
    away_manager: Manager,
    home_available_profiles: Vec<TeamTacticalProfile>,
    away_available_profiles: Vec<TeamTacticalProfile>,
    home_playbook: Vec<PlayCall>,
    away_playbook: Vec<PlayCall>,
    home_offensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    home_defensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    away_offensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    away_defensive_position_index: Arc<HashMap<Uuid, DomainPosition>>,
    home_role_index: Arc<HashMap<Uuid, SlotRole>>,
    away_role_index: Arc<HashMap<Uuid, SlotRole>>,
    home_instructions_index: Arc<HashMap<Uuid, PlayerInstructions>>,
    away_instructions_index: Arc<HashMap<Uuid, PlayerInstructions>>,
    player_attribute_tables: HashMap<Uuid, PlayerAttributeTable>,
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

    pub fn home_offensive_position_index_arc(&self) -> Arc<HashMap<Uuid, DomainPosition>> {
        Arc::clone(&self.home_offensive_position_index)
    }

    pub fn home_defensive_position_index_arc(&self) -> Arc<HashMap<Uuid, DomainPosition>> {
        Arc::clone(&self.home_defensive_position_index)
    }

    pub fn away_offensive_position_index_arc(&self) -> Arc<HashMap<Uuid, DomainPosition>> {
        Arc::clone(&self.away_offensive_position_index)
    }

    pub fn away_defensive_position_index_arc(&self) -> Arc<HashMap<Uuid, DomainPosition>> {
        Arc::clone(&self.away_defensive_position_index)
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

    pub fn offensive_position_index_for_team_arc(
        &self,
        team_id: Uuid,
    ) -> Arc<HashMap<Uuid, DomainPosition>> {
        if team_id == self.home_team_id {
            Arc::clone(&self.home_offensive_position_index)
        } else {
            Arc::clone(&self.away_offensive_position_index)
        }
    }

    pub fn defensive_position_index_for_team_arc(
        &self,
        team_id: Uuid,
    ) -> Arc<HashMap<Uuid, DomainPosition>> {
        if team_id == self.home_team_id {
            Arc::clone(&self.home_defensive_position_index)
        } else {
            Arc::clone(&self.away_defensive_position_index)
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

    pub fn position_index_for_team_arc(
        &self,
        team_id: Uuid,
        is_offense: bool,
    ) -> Arc<HashMap<Uuid, DomainPosition>> {
        if is_offense {
            self.offensive_position_index_for_team_arc(team_id)
        } else {
            self.defensive_position_index_for_team_arc(team_id)
        }
    }

    pub fn home_role_index(&self) -> &HashMap<Uuid, SlotRole> {
        &self.home_role_index
    }

    pub fn away_role_index(&self) -> &HashMap<Uuid, SlotRole> {
        &self.away_role_index
    }

    pub fn home_role_index_arc(&self) -> Arc<HashMap<Uuid, SlotRole>> {
        Arc::clone(&self.home_role_index)
    }

    pub fn away_role_index_arc(&self) -> Arc<HashMap<Uuid, SlotRole>> {
        Arc::clone(&self.away_role_index)
    }

    pub fn role_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, SlotRole> {
        if team_id == self.home_team_id {
            &self.home_role_index
        } else {
            &self.away_role_index
        }
    }

    pub fn role_index_for_team_arc(&self, team_id: Uuid) -> Arc<HashMap<Uuid, SlotRole>> {
        if team_id == self.home_team_id {
            Arc::clone(&self.home_role_index)
        } else {
            Arc::clone(&self.away_role_index)
        }
    }

    pub fn home_instructions_index(&self) -> &HashMap<Uuid, PlayerInstructions> {
        &self.home_instructions_index
    }

    pub fn away_instructions_index(&self) -> &HashMap<Uuid, PlayerInstructions> {
        &self.away_instructions_index
    }

    pub fn home_instructions_index_arc(&self) -> Arc<HashMap<Uuid, PlayerInstructions>> {
        Arc::clone(&self.home_instructions_index)
    }

    pub fn away_instructions_index_arc(&self) -> Arc<HashMap<Uuid, PlayerInstructions>> {
        Arc::clone(&self.away_instructions_index)
    }

    pub fn instructions_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, PlayerInstructions> {
        if team_id == self.home_team_id {
            &self.home_instructions_index
        } else {
            &self.away_instructions_index
        }
    }

    pub fn instructions_index_for_team_arc(
        &self,
        team_id: Uuid,
    ) -> Arc<HashMap<Uuid, PlayerInstructions>> {
        if team_id == self.home_team_id {
            Arc::clone(&self.home_instructions_index)
        } else {
            Arc::clone(&self.away_instructions_index)
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

    pub fn find_player<'a>(&'a self, player_id: &Uuid) -> Option<&'a Player> {
        self.home_lineup
            .assignments()
            .iter()
            .chain(self.away_lineup.assignments().iter())
            .map(|a| a.player())
            .find(|p| p.id() == *player_id)
    }

    pub fn home_captain<'a>(
        &'a self,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        let players: Vec<&Player> = self
            .home_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();
        crate::psychology::systems::baseline::find_active_captain(&players, attribute_keys)
    }

    pub fn away_captain<'a>(
        &'a self,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        let players: Vec<&Player> = self
            .away_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();
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