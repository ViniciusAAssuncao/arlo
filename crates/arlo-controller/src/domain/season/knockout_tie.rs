use crate::domain::season::bracket_seed::BracketSeed;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KnockoutTie {
    id: Uuid,
    season_stage_id: Uuid,
    round_index: u32,
    tie_index: u32,
    high_seed: BracketSeed,
    low_seed: BracketSeed,
    leg_one_fixture_id: Uuid,
    leg_two_fixture_id: Option<Uuid>,
    aggregate_winner_team_id: Option<Uuid>,
}

impl KnockoutTie {
    pub fn new(
        id: Uuid,
        season_stage_id: Uuid,
        round_index: u32,
        tie_index: u32,
        high_seed: BracketSeed,
        low_seed: BracketSeed,
        leg_one_fixture_id: Uuid,
        leg_two_fixture_id: Option<Uuid>,
        aggregate_winner_team_id: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            season_stage_id,
            round_index,
            tie_index,
            high_seed,
            low_seed,
            leg_one_fixture_id,
            leg_two_fixture_id,
            aggregate_winner_team_id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn season_stage_id(&self) -> Uuid {
        self.season_stage_id
    }

    pub fn round_index(&self) -> u32 {
        self.round_index
    }

    pub fn tie_index(&self) -> u32 {
        self.tie_index
    }

    pub fn high_seed(&self) -> BracketSeed {
        self.high_seed
    }

    pub fn low_seed(&self) -> BracketSeed {
        self.low_seed
    }

    pub fn leg_one_fixture_id(&self) -> Uuid {
        self.leg_one_fixture_id
    }

    pub fn leg_two_fixture_id(&self) -> Option<Uuid> {
        self.leg_two_fixture_id
    }

    pub fn aggregate_winner_team_id(&self) -> Option<Uuid> {
        self.aggregate_winner_team_id
    }

    pub fn loser_team_id(&self) -> Option<Uuid> {
        let winner_id = self.aggregate_winner_team_id?;
        if winner_id == self.high_seed.team_id() {
            Some(self.low_seed.team_id())
        } else if winner_id == self.low_seed.team_id() {
            Some(self.high_seed.team_id())
        } else {
            None
        }
    }
}
