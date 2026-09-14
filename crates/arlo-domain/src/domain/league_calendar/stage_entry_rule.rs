use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StageEntryRule {
    AllTeams,
    TopN { count: u32 },
    BottomN { count: u32 },
}