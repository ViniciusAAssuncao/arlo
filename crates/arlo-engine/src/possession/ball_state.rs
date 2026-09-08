use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BallState {
    InPlay,
    OutOfBounds,
    #[default]
    Dead,
}

impl BallState {
    pub fn is_in_play(&self) -> bool {
        matches!(self, Self::InPlay)
    }

    pub fn is_out_of_bounds(&self) -> bool {
        matches!(self, Self::OutOfBounds)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self, Self::Dead)
    }
}
