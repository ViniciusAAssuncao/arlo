use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TurnoverCategory {
    Interception,
    Dispossession,
    Downs,
    OutOfBounds,
    MissedShot,
}

impl TurnoverCategory {
    pub fn allows_recovering_player(&self) -> bool {
        matches!(self, Self::Interception | Self::Dispossession)
    }

    pub fn sanitize_recovering_player(&self, recovering_player: Option<Uuid>) -> Option<Uuid> {
        if self.allows_recovering_player() {
            recovering_player
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnoverResolution {
    category: TurnoverCategory,
    new_offense_team_id: Uuid,
    recovering_player_id: Option<Uuid>,
}

impl TurnoverResolution {
    pub fn new(
        category: TurnoverCategory,
        new_offense_team_id: Uuid,
        recovering_player_id: Option<Uuid>,
    ) -> Self {
        let recovering_player_id = category.sanitize_recovering_player(recovering_player_id);
        Self {
            category,
            new_offense_team_id,
            recovering_player_id,
        }
    }

    pub fn category(&self) -> TurnoverCategory {
        self.category
    }

    pub fn new_offense_team_id(&self) -> Uuid {
        self.new_offense_team_id
    }

    pub fn recovering_player_id(&self) -> Option<Uuid> {
        self.recovering_player_id
    }
}