use crate::instructions::axes::Mentality;
use crate::instructions::builder::TeamInstructionsBuilder;
use crate::instructions::derivation::{ChannelDistribution, EngagementLine};
use crate::instructions::in_possession::InPossessionInstructions;
use crate::instructions::out_of_possession::OutOfPossessionInstructions;
use crate::instructions::transition::{PressBlockShape, TransitionInstructions};
use crate::playcall::decision_emphasis::DecisionEmphasis;
use crate::playcall::decision_emphasis_defaults::derive_default_decision_emphasis;
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

    pub fn press_block_shape(&self) -> PressBlockShape {
        self.transition.press_block_shape()
    }

    pub fn default_decision_emphasis(&self) -> DecisionEmphasis {
        derive_default_decision_emphasis(
            self.in_possession.mentality(),
            self.in_possession.directness(),
            self.in_possession.width(),
            self.in_possession.passing_range(),
            self.in_possession.aeriality(),
            self.in_possession.physicality(),
            self.in_possession.scoring_patience(),
        )
    }
}
