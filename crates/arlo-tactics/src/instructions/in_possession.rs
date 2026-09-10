use crate::instructions::axes::{
    Aeriality, Directness, FlankBias, Mentality, PassingRange, Physicality, ScoringPatience,
    Structure, Tempo, Width,
};
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
    passing_range: PassingRange,
    aeriality: Aeriality,
    physicality: Physicality,
    scoring_patience: ScoringPatience,
}

impl InPossessionInstructions {
    pub fn new(
        mentality: Mentality,
        tempo: Tempo,
        width: Width,
        flank_bias: FlankBias,
        directness: Directness,
        structure: Structure,
        passing_range: PassingRange,
        aeriality: Aeriality,
        physicality: Physicality,
        scoring_patience: ScoringPatience,
    ) -> Self {
        Self {
            mentality,
            tempo,
            width,
            flank_bias,
            directness,
            structure,
            passing_range,
            aeriality,
            physicality,
            scoring_patience,
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
            passing_range: PassingRange::default(),
            aeriality: Aeriality::default(),
            physicality: Physicality::new_clamped(0.5),
            scoring_patience: ScoringPatience::new_clamped(0.5),
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

    pub fn passing_range(&self) -> PassingRange {
        self.passing_range
    }

    pub fn aeriality(&self) -> Aeriality {
        self.aeriality
    }

    pub fn physicality(&self) -> Physicality {
        self.physicality
    }

    pub fn scoring_patience(&self) -> ScoringPatience {
        self.scoring_patience
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
