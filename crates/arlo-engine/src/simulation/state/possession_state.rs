use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PossessionState {
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    carrier_id: Option<Uuid>,
    is_in_play: bool,
    is_dead: bool,
}

impl PossessionState {
    pub fn new(offense_team_id: Uuid, defense_team_id: Uuid) -> Self {
        Self {
            offense_team_id,
            defense_team_id,
            carrier_id: None,
            is_in_play: false,
            is_dead: true,
        }
    }

    pub fn with_carrier(mut self, carrier_id: Option<Uuid>) -> Self {
        self.carrier_id = carrier_id;
        self
    }

    pub fn offense_team_id(&self) -> Uuid {
        self.offense_team_id
    }

    pub fn defense_team_id(&self) -> Uuid {
        self.defense_team_id
    }

    pub fn carrier_id(&self) -> Option<Uuid> {
        self.carrier_id
    }

    pub fn is_in_play(&self) -> bool {
        self.is_in_play
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn set_in_play(&mut self, in_play: bool) {
        self.is_in_play = in_play;
        self.is_dead = !in_play;
    }

    pub fn set_dead(&mut self, dead: bool) {
        self.is_dead = dead;
        self.is_in_play = !dead;
    }

    pub fn set_carrier_id(&mut self, carrier_id: Option<Uuid>) {
        self.carrier_id = carrier_id;
    }

    pub fn is_offense(&self, team_id: Uuid) -> bool {
        self.offense_team_id == team_id
    }

    pub fn is_defense(&self, team_id: Uuid) -> bool {
        self.defense_team_id == team_id
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.offense_team_id, &mut self.defense_team_id);
        self.carrier_id = None;
    }

    pub fn turnover(&mut self, new_offense_team_id: Uuid, new_carrier_id: Option<Uuid>) {
        if self.offense_team_id != new_offense_team_id {
            self.defense_team_id = self.offense_team_id;
            self.offense_team_id = new_offense_team_id;
        }
        self.carrier_id = new_carrier_id;
    }

    pub fn set_offense(&mut self, new_offense_team_id: Uuid) {
        if self.offense_team_id != new_offense_team_id {
            self.defense_team_id = self.offense_team_id;
            self.offense_team_id = new_offense_team_id;
            self.carrier_id = None;
        }
    }
}