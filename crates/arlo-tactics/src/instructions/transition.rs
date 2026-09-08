use crate::instructions::axes::{CounterAttackIntensity, CounterPressIntensity, Mentality};
use crate::instructions::mentality_defaults::{
    default_counter_attack_intensity, default_counter_press_intensity,
};
use arlo_math::stats::UnipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TransitionInstructions {
    counter_attack_intensity: CounterAttackIntensity,
    counter_press_intensity: CounterPressIntensity,
}

impl TransitionInstructions {
    pub fn new(
        counter_attack_intensity: CounterAttackIntensity,
        counter_press_intensity: CounterPressIntensity,
    ) -> Self {
        Self {
            counter_attack_intensity,
            counter_press_intensity,
        }
    }

    pub fn from_mentality(mentality: Mentality) -> Self {
        Self {
            counter_attack_intensity: default_counter_attack_intensity(&mentality),
            counter_press_intensity: default_counter_press_intensity(&mentality),
        }
    }

    pub fn counter_attack_intensity(&self) -> CounterAttackIntensity {
        self.counter_attack_intensity
    }

    pub fn counter_press_intensity(&self) -> CounterPressIntensity {
        self.counter_press_intensity
    }

    pub fn regroup_discipline(&self) -> UnipolarScalar {
        UnipolarScalar::new_clamped(1.0 - self.counter_press_intensity.value())
    }
}

impl Default for TransitionInstructions {
    fn default() -> Self {
        Self::from_mentality(Mentality::default())
    }
}
