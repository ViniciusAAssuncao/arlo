use crate::instructions::axes::{Directness, FlankBias, Mentality, Structure, Tempo, Width};
use crate::instructions::derivation::ChannelDistribution;
use crate::instructions::mentality_defaults::{default_directness, default_tempo, default_width};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InPossessionInstructions {
    mentality: Mentality,
    tempo: Tempo,
    width: Width,
    flank_bias: FlankBias,
    directness: Directness,
    structure: Structure,
}

impl InPossessionInstructions {
    pub fn new(
        mentality: Mentality,
        tempo: Tempo,
        width: Width,
        flank_bias: FlankBias,
        directness: Directness,
        structure: Structure,
    ) -> Self {
        Self {
            mentality,
            tempo,
            width,
            flank_bias,
            directness,
            structure,
        }
    }

    pub fn from_mentality(mentality: Mentality) -> Self {
        Self {
            mentality,
            tempo: default_tempo(&mentality),
            width: default_width(&mentality),
            flank_bias: FlankBias::default(),
            directness: default_directness(&mentality),
            structure: Structure::default(),
        }
    }

    pub fn mentality(&self) -> Mentality {
        self.mentality
    }

    pub fn tempo(&self) -> Tempo {
        self.tempo
    }

    pub fn width(&self) -> Width {
        self.width
    }

    pub fn flank_bias(&self) -> FlankBias {
        self.flank_bias
    }

    pub fn directness(&self) -> Directness {
        self.directness
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn channel_distribution(&self) -> ChannelDistribution {
        ChannelDistribution::from_width_and_flank_bias(self.width, self.flank_bias)
    }
}

impl Default for InPossessionInstructions {
    fn default() -> Self {
        Self::from_mentality(Mentality::default())
    }
}
