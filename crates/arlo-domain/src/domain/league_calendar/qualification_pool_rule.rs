use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualificationPoolRule {
    AllTeams,
    TopN { count: u32 },
    BottomN { count: u32 },
    GroupWinners,
    GroupRunnersUp,
    BestAtGroupPosition { position_index: u32, count: u32 },
    PositionRange { start_position: u32, end_position: u32 },
}

impl QualificationPoolRule {
    pub fn requires_groups(&self) -> bool {
        matches!(
            self,
            Self::GroupWinners | Self::GroupRunnersUp | Self::BestAtGroupPosition { .. }
        )
    }
}
