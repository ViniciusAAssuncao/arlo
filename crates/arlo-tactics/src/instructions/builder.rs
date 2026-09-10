use crate::instructions::axes::{
    Aeriality, Aggression, Compactness, CounterAttackIntensity, CounterPressIntensity,
    DefensiveLineHeight, Directness, FlankBias, Mentality, PassingRange, Physicality,
    PressingIntensity, ScoringPatience, Structure, Tempo, Width,
};
use crate::instructions::in_possession::InPossessionInstructions;
use crate::instructions::mentality_defaults::{
    default_aggression, default_counter_attack_intensity, default_counter_press_intensity,
    default_defensive_line_height, default_directness, default_pressing_intensity, default_tempo,
    default_width,
};
use crate::instructions::out_of_possession::OutOfPossessionInstructions;
use crate::instructions::team_instructions::TeamInstructions;
use crate::instructions::transition::{PressBlockShape, TransitionInstructions};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TeamInstructionsBuilder {
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
    defensive_line_height: DefensiveLineHeight,
    compactness: Compactness,
    pressing_intensity: PressingIntensity,
    aggression: Aggression,
    counter_attack_intensity: CounterAttackIntensity,
    counter_press_intensity: CounterPressIntensity,
    press_block_shape: PressBlockShape,
}

impl TeamInstructionsBuilder {
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
            defensive_line_height: default_defensive_line_height(&mentality),
            compactness: Compactness::new_clamped(0.5),
            pressing_intensity: default_pressing_intensity(&mentality),
            aggression: default_aggression(&mentality),
            counter_attack_intensity: default_counter_attack_intensity(&mentality),
            counter_press_intensity: default_counter_press_intensity(&mentality),
            press_block_shape: PressBlockShape::new_clamped(0.5),
        }
    }

    pub fn with_mentality(mut self, mentality: Mentality) -> Self {
        self.mentality = mentality;
        self
    }

    pub fn with_tempo(mut self, tempo: Tempo) -> Self {
        self.tempo = tempo;
        self
    }

    pub fn with_width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    pub fn with_flank_bias(mut self, flank_bias: FlankBias) -> Self {
        self.flank_bias = flank_bias;
        self
    }

    pub fn with_directness(mut self, directness: Directness) -> Self {
        self.directness = directness;
        self
    }

    pub fn with_structure(mut self, structure: Structure) -> Self {
        self.structure = structure;
        self
    }

    pub fn with_passing_range(mut self, passing_range: PassingRange) -> Self {
        self.passing_range = passing_range;
        self
    }

    pub fn with_aeriality(mut self, aeriality: Aeriality) -> Self {
        self.aeriality = aeriality;
        self
    }

    pub fn with_physicality(mut self, physicality: Physicality) -> Self {
        self.physicality = physicality;
        self
    }

    pub fn with_scoring_patience(mut self, scoring_patience: ScoringPatience) -> Self {
        self.scoring_patience = scoring_patience;
        self
    }

    pub fn with_defensive_line_height(
        mut self,
        defensive_line_height: DefensiveLineHeight,
    ) -> Self {
        self.defensive_line_height = defensive_line_height;
        self
    }

    pub fn with_compactness(mut self, compactness: Compactness) -> Self {
        self.compactness = compactness;
        self
    }

    pub fn with_pressing_intensity(mut self, pressing_intensity: PressingIntensity) -> Self {
        self.pressing_intensity = pressing_intensity;
        self
    }

    pub fn with_aggression(mut self, aggression: Aggression) -> Self {
        self.aggression = aggression;
        self
    }

    pub fn with_counter_attack_intensity(
        mut self,
        counter_attack_intensity: CounterAttackIntensity,
    ) -> Self {
        self.counter_attack_intensity = counter_attack_intensity;
        self
    }

    pub fn with_counter_press_intensity(
        mut self,
        counter_press_intensity: CounterPressIntensity,
    ) -> Self {
        self.counter_press_intensity = counter_press_intensity;
        self
    }

    pub fn with_press_block_shape(mut self, press_block_shape: PressBlockShape) -> Self {
        self.press_block_shape = press_block_shape;
        self
    }

    pub fn with_in_possession(mut self, in_possession: InPossessionInstructions) -> Self {
        self.mentality = in_possession.mentality();
        self.tempo = in_possession.tempo();
        self.width = in_possession.width();
        self.flank_bias = in_possession.flank_bias();
        self.directness = in_possession.directness();
        self.structure = in_possession.structure();
        self.passing_range = in_possession.passing_range();
        self.aeriality = in_possession.aeriality();
        self.physicality = in_possession.physicality();
        self.scoring_patience = in_possession.scoring_patience();
        self
    }

    pub fn with_out_of_possession(
        mut self,
        out_of_possession: OutOfPossessionInstructions,
    ) -> Self {
        self.defensive_line_height = out_of_possession.defensive_line_height();
        self.compactness = out_of_possession.compactness();
        self.pressing_intensity = out_of_possession.pressing_intensity();
        self.aggression = out_of_possession.aggression();
        self
    }

    pub fn with_transition(mut self, transition: TransitionInstructions) -> Self {
        self.counter_attack_intensity = transition.counter_attack_intensity();
        self.counter_press_intensity = transition.counter_press_intensity();
        self.press_block_shape = transition.press_block_shape();
        self
    }

    pub fn build(self) -> TeamInstructions {
        TeamInstructions::new(
            InPossessionInstructions::new(
                self.mentality,
                self.tempo,
                self.width,
                self.flank_bias,
                self.directness,
                self.structure,
                self.passing_range,
                self.aeriality,
                self.physicality,
                self.scoring_patience,
            ),
            OutOfPossessionInstructions::new(
                self.defensive_line_height,
                self.compactness,
                self.pressing_intensity,
                self.aggression,
            ),
            TransitionInstructions::new(
                self.counter_attack_intensity,
                self.counter_press_intensity,
                self.press_block_shape,
            ),
        )
    }
}

impl Default for TeamInstructionsBuilder {
    fn default() -> Self {
        Self::from_mentality(Mentality::default())
    }
}
