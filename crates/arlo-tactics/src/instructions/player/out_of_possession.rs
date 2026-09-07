use crate::instructions::player::axes::{DepthDiscipline, EngagementBias};
use crate::instructions::player::marking::MarkingAssignment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct OutOfPossessionPlayerInstructions {
    engagement_bias: EngagementBias,
    depth_discipline: DepthDiscipline,
    marking: Option<MarkingAssignment>,
}

impl OutOfPossessionPlayerInstructions {
    pub fn new(
        engagement_bias: EngagementBias,
        depth_discipline: DepthDiscipline,
        marking: Option<MarkingAssignment>,
    ) -> Self {
        Self {
            engagement_bias,
            depth_discipline,
            marking,
        }
    }

    pub fn engagement_bias(&self) -> EngagementBias {
        self.engagement_bias
    }

    pub fn depth_discipline(&self) -> DepthDiscipline {
        self.depth_discipline
    }

    pub fn marking(&self) -> Option<MarkingAssignment> {
        self.marking
    }
}