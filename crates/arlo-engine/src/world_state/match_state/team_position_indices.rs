use crate::lineup_runtime::Lineup;
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::{Position as DomainPosition, SlotRole};
use arlo_tactics::PlayerInstructions;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) fn compute_lineup_indices(
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

impl TeamRegistry {
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
}