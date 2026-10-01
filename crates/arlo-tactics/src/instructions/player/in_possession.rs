use crate::instructions::player::axes::{CreativeLicense, InvolvementPriority, PositioningBias};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct InPossessionPlayerInstructions {
    positioning_bias: PositioningBias,
    involvement_priority: InvolvementPriority,
    creative_license: CreativeLicense,
}

impl InPossessionPlayerInstructions {
    pub fn new(
        positioning_bias: PositioningBias,
        involvement_priority: InvolvementPriority,
        creative_license: CreativeLicense,
    ) -> Self {
        Self {
            positioning_bias,
            involvement_priority,
            creative_license,
        }
    }

    pub fn positioning_bias(&self) -> PositioningBias {
        self.positioning_bias
    }

    pub fn involvement_priority(&self) -> InvolvementPriority {
        self.involvement_priority
    }

    pub fn creative_license(&self) -> CreativeLicense {
        self.creative_license
    }
}
