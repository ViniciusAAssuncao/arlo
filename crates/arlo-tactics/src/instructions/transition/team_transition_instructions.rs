use crate::instructions::axes::{CounterAttackIntensity, CounterPressIntensity, Mentality};
use crate::instructions::mentality_defaults::{
    default_counter_attack_intensity, default_counter_press_intensity,
};
use crate::instructions::transition::press_block_shape::PressBlockShape;
use crate::instructions::transition::regroup_discipline_constants::{
    REGROUP_DISCIPLINE_COUNTER_PRESS_WEIGHT, REGROUP_DISCIPLINE_COVER_RATIO_WEIGHT,
};
use arlo_math::stats::UnipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TransitionInstructions {
    counter_attack_intensity: CounterAttackIntensity,
    counter_press_intensity: CounterPressIntensity,
    press_block_shape: PressBlockShape,
}

impl TransitionInstructions {
    pub fn new(
        counter_attack_intensity: CounterAttackIntensity,
        counter_press_intensity: CounterPressIntensity,
        press_block_shape: PressBlockShape,
    ) -> Self {
        Self {
            counter_attack_intensity,
            counter_press_intensity,
            press_block_shape,
        }
    }

    pub fn from_mentality(mentality: Mentality) -> Self {
        Self {
            counter_attack_intensity: default_counter_attack_intensity(&mentality),
            counter_press_intensity: default_counter_press_intensity(&mentality),
            press_block_shape: PressBlockShape::new_clamped(0.5),
        }
    }

    pub fn counter_attack_intensity(&self) -> CounterAttackIntensity {
        self.counter_attack_intensity
    }

    pub fn counter_press_intensity(&self) -> CounterPressIntensity {
        self.counter_press_intensity
    }

    pub fn press_block_shape(&self) -> PressBlockShape {
        self.press_block_shape
    }

    pub fn regroup_discipline(&self) -> UnipolarScalar {
        let val = (1.0 - self.counter_press_intensity.value())
            * REGROUP_DISCIPLINE_COUNTER_PRESS_WEIGHT
            + self.press_block_shape.cover_ratio() * REGROUP_DISCIPLINE_COVER_RATIO_WEIGHT;
        UnipolarScalar::new_clamped(val)
    }
}

impl Default for TransitionInstructions {
    fn default() -> Self {
        Self::from_mentality(Mentality::default())
    }
}
