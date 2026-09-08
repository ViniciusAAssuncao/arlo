use crate::instructions::axes::{
    Aggression, Compactness, DefensiveLineHeight, Mentality, PressingIntensity,
};
use crate::instructions::derivation::EngagementLine;
use crate::instructions::mentality_defaults::{
    default_aggression, default_defensive_line_height, default_pressing_intensity,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OutOfPossessionInstructions {
    defensive_line_height: DefensiveLineHeight,
    compactness: Compactness,
    pressing_intensity: PressingIntensity,
    aggression: Aggression,
}

impl OutOfPossessionInstructions {
    pub fn new(
        defensive_line_height: DefensiveLineHeight,
        compactness: Compactness,
        pressing_intensity: PressingIntensity,
        aggression: Aggression,
    ) -> Self {
        Self {
            defensive_line_height,
            compactness,
            pressing_intensity,
            aggression,
        }
    }

    pub fn from_mentality(mentality: Mentality) -> Self {
        Self {
            defensive_line_height: default_defensive_line_height(&mentality),
            compactness: Compactness::new_clamped(0.5),
            pressing_intensity: default_pressing_intensity(&mentality),
            aggression: default_aggression(&mentality),
        }
    }

    pub fn defensive_line_height(&self) -> DefensiveLineHeight {
        self.defensive_line_height
    }

    pub fn compactness(&self) -> Compactness {
        self.compactness
    }

    pub fn pressing_intensity(&self) -> PressingIntensity {
        self.pressing_intensity
    }

    pub fn aggression(&self) -> Aggression {
        self.aggression
    }

    pub fn engagement_line(&self) -> EngagementLine {
        EngagementLine::from_defensive_line_and_compactness(
            self.defensive_line_height,
            self.compactness,
        )
    }
}

impl Default for OutOfPossessionInstructions {
    fn default() -> Self {
        Self::from_mentality(Mentality::default())
    }
}