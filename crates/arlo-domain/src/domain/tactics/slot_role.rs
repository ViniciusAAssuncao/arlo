use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SlotRole {
    #[default]
    Standard,
    FalseArtrine,
    Launcher,
    Safeguard,
    Blocker,
    Kicker,
}
