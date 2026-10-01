use crate::instructions::player::axes::{
    CreativeLicense, DepthDiscipline, EngagementBias, InvolvementPriority, PositioningBias,
    ReleaseTempo, TransitionUrgency,
};
use crate::instructions::player::in_possession::InPossessionPlayerInstructions;
use crate::instructions::player::marking::MarkingAssignment;
use crate::instructions::player::out_of_possession::OutOfPossessionPlayerInstructions;
use crate::instructions::player::player_instructions::PlayerInstructions;
use crate::instructions::player::transition::TransitionPlayerInstructions;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PlayerInstructionsBuilder {
    positioning_bias: PositioningBias,
    involvement_priority: InvolvementPriority,
    creative_license: CreativeLicense,
    engagement_bias: EngagementBias,
    depth_discipline: DepthDiscipline,
    marking: Option<MarkingAssignment>,
    transition_urgency: TransitionUrgency,
    release_tempo: ReleaseTempo,
}

impl PlayerInstructionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_positioning_bias(mut self, positioning_bias: PositioningBias) -> Self {
        self.positioning_bias = positioning_bias;
        self
    }

    pub fn with_involvement_priority(mut self, involvement_priority: InvolvementPriority) -> Self {
        self.involvement_priority = involvement_priority;
        self
    }

    pub fn with_creative_license(mut self, creative_license: CreativeLicense) -> Self {
        self.creative_license = creative_license;
        self
    }

    pub fn with_engagement_bias(mut self, engagement_bias: EngagementBias) -> Self {
        self.engagement_bias = engagement_bias;
        self
    }

    pub fn with_depth_discipline(mut self, depth_discipline: DepthDiscipline) -> Self {
        self.depth_discipline = depth_discipline;
        self
    }

    pub fn with_marking(mut self, marking: MarkingAssignment) -> Self {
        self.marking = Some(marking);
        self
    }

    pub fn with_transition_urgency(mut self, transition_urgency: TransitionUrgency) -> Self {
        self.transition_urgency = transition_urgency;
        self
    }

    pub fn with_release_tempo(mut self, release_tempo: ReleaseTempo) -> Self {
        self.release_tempo = release_tempo;
        self
    }

    pub fn build(self) -> PlayerInstructions {
        PlayerInstructions::new(
            InPossessionPlayerInstructions::new(
                self.positioning_bias,
                self.involvement_priority,
                self.creative_license,
            ),
            OutOfPossessionPlayerInstructions::new(
                self.engagement_bias,
                self.depth_discipline,
                self.marking,
            ),
            TransitionPlayerInstructions::new(self.transition_urgency, self.release_tempo),
        )
    }
}
