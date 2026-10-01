use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMatchRating {
    player_id: Uuid,
    rating: f64,
}

impl PlayerMatchRating {
    pub fn new(player_id: Uuid, rating: f64) -> Self {
        Self { player_id, rating }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn rating(&self) -> f64 {
        self.rating
    }
}