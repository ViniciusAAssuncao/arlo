use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TitleWinner {
    Team(Uuid),
    Federation(Uuid),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Title {
    id: Uuid,
    competition_id: Uuid,
    season_label: String,
    winner: TitleWinner,
}

impl Title {
    pub fn new(
        id: Uuid,
        competition_id: Uuid,
        season_label: impl Into<String>,
        winner: TitleWinner,
    ) -> DomainResult<Self> {
        let season_label = season_label.into();
        validate_not_empty(&season_label, "season_label")?;

        Ok(Self {
            id,
            competition_id,
            season_label,
            winner,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn competition_id(&self) -> Uuid {
        self.competition_id
    }

    pub fn season_label(&self) -> &str {
        &self.season_label
    }

    pub fn winner(&self) -> TitleWinner {
        self.winner
    }
}
