use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct TimeCallIntent;

impl TimeCallIntent {
    pub fn new() -> Self {
        Self
    }
}