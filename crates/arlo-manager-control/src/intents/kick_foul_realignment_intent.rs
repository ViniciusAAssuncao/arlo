use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct KickFoulRealignmentIntent;

impl KickFoulRealignmentIntent {
    pub fn new() -> Self {
        Self
    }
}