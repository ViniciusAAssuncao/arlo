use crate::tactics::Lineup;
use arlo_domain::{Player, Position as DomainPosition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRegistry {
    home_team_id: Uuid,
    away_team_id: Uuid,
    home_lineup: Lineup,
    away_lineup: Lineup,
    home_offensive_position_index: HashMap<Uuid, DomainPosition>,
    home_defensive_position_index: HashMap<Uuid, DomainPosition>,
    away_offensive_position_index: HashMap<Uuid, DomainPosition>,
    away_defensive_position_index: HashMap<Uuid, DomainPosition>,
}

impl TeamRegistry {
    pub fn new(
        home_team_id: Uuid,
        away_team_id: Uuid,
        home_lineup: Lineup,
        away_lineup: Lineup,
    ) -> Self {
        let home_offensive_position_index = home_lineup.offensive_position_index();
        let home_defensive_position_index = home_lineup.defensive_position_index();
        let away_offensive_position_index = away_lineup.offensive_position_index();
        let away_defensive_position_index = away_lineup.defensive_position_index();

        Self {
            home_team_id,
            away_team_id,
            home_lineup,
            away_lineup,
            home_offensive_position_index,
            home_defensive_position_index,
            away_offensive_position_index,
            away_defensive_position_index,
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

    pub fn offensive_position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        if team_id == self.home_team_id {
            &self.home_offensive_position_index
        } else {
            &self.away_offensive_position_index
        }
    }

    pub fn defensive_position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
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

    pub fn is_home_player(&self, player_id: &Uuid) -> bool {
        self.home_offensive_position_index.contains_key(player_id)
    }

    pub fn find_player(&self, player_id: &Uuid) -> Option<&Player> {
        self.home_lineup
            .players()
            .into_iter()
            .chain(self.away_lineup.players().into_iter())
            .find(|p| p.id() == *player_id)
    }
}