use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayInvalidated {
    first_sequence: u64,
    last_sequence: u64,
}

impl PlayInvalidated {
    pub fn new(first_sequence: u64, last_sequence: u64) -> Self {
        Self { first_sequence, last_sequence }
    }

    pub fn first_sequence(&self) -> u64 {
        self.first_sequence
    }

    pub fn last_sequence(&self) -> u64 {
        self.last_sequence
    }
}
