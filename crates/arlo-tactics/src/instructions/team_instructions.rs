use crate::instructions::axes::Mentality;
use crate::instructions::builder::TeamInstructionsBuilder;
use crate::instructions::derivation::{ChannelDistribution, EngagementLine};
use crate::instructions::in_possession::InPossessionInstructions;
use crate::instructions::out_of_possession::OutOfPossessionInstructions;
use crate::instructions::transition::TransitionInstructions;
use arlo_math::stats::UnipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamInstructions {
    in_possession: InPossessionInstructions,
    out_of_possession: OutOfPossessionInstructions,
    transition: TransitionInstructions,
}

impl TeamInstructions {
    pub fn new(
        in_possession: InPossessionInstructions,
        out_of_possession: OutOfPossessionInstructions,
        transition: TransitionInstructions,
    ) -> Self {
        Self {
            in_possession,
            out_of_possession,
            transition,
        }
    }

    pub fn builder(mentality: Mentality) -> TeamInstructionsBuilder {
        TeamInstructionsBuilder::from_mentality(mentality)
    }

    pub fn in_possession(&self) -> &InPossessionInstructions {
        &self.in_possession
    }

    pub fn out_of_possession(&self) -> &OutOfPossessionInstructions {
        &self.out_of_possession
    }

    pub fn transition(&self) -> &TransitionInstructions {
        &self.transition
    }

    pub fn channel_distribution(&self) -> ChannelDistribution {
        self.in_possession.channel_distribution()
    }

    pub fn engagement_line(&self) -> EngagementLine {
        self.out_of_possession.engagement_line()
    }

    pub fn regroup_discipline(&self) -> UnipolarScalar {
        self.transition.regroup_discipline()
    }
}