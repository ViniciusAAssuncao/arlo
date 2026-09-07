use crate::instructions::axes::{
    Aggression, Compactness, CounterAttackIntensity, CounterPressIntensity, DefensiveLineHeight,
    Directness, FlankBias, Mentality, PressingIntensity, Structure, Tempo, Width,
};
use crate::instructions::in_possession::InPossessionInstructions;
use crate::instructions::mentality_defaults::{
    default_aggression, default_counter_attack_intensity, default_counter_press_intensity,
    default_defensive_line_height, default_directness, default_pressing_intensity, default_tempo,
    default_width,
};
use crate::instructions::out_of_possession::OutOfPossessionInstructions;
use crate::instructions::team_instructions::TeamInstructions;
use crate::instructions::transition::TransitionInstructions;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TeamInstructionsBuilder {
    mentality: Mentality,
    tempo: Tempo,
    width: Width,
    flank_bias: FlankBias,
    directness: Directness,
    structure: Structure,
    defensive_line_height: DefensiveLineHeight,
    compactness: Compactness,
    pressing_intensity: PressingIntensity,
    aggression: Aggression,
    counter_attack_intensity: CounterAttackIntensity,
    counter_press_intensity: CounterPressIntensity,
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
            defensive_line_height: default_defensive_line_height(&mentality),
            compactness: Compactness::new_clamped(0.5),
            pressing_intensity: default_pressing_intensity(&mentality),
            aggression: default_aggression(&mentality),
            counter_attack_intensity: default_counter_attack_intensity(&mentality),
            counter_press_intensity: default_counter_press_intensity(&mentality),
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

    pub fn with_in_possession(mut self, in_possession: InPossessionInstructions) -> Self {
        self.mentality = in_possession.mentality();
        self.tempo = in_possession.tempo();
        self.width = in_possession.width();
        self.flank_bias = in_possession.flank_bias();
        self.directness = in_possession.directness();
        self.structure = in_possession.structure();
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
            ),
        )
    }
}

impl Default for TeamInstructionsBuilder {
    fn default() -> Self {
        Self::from_mentality(Mentality::default())
    }
}