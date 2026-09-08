use crate::instructions::axes::{
    Aggression, CounterAttackIntensity, CounterPressIntensity, DefensiveLineHeight, Directness,
    Mentality, PressingIntensity, Tempo, Width,
};
use crate::instructions::mentality_defaults_constants::{
    AGGRESSION_SENSITIVITY, COUNTER_ATTACK_SENSITIVITY, COUNTER_PRESS_SENSITIVITY,
    DIRECTNESS_SENSITIVITY, LINE_HEIGHT_SENSITIVITY, PRESSING_SENSITIVITY, TEMPO_SENSITIVITY,
    WIDTH_SENSITIVITY,
};

pub fn default_tempo(m: &Mentality) -> Tempo {
    Tempo::new_clamped(m.value() * TEMPO_SENSITIVITY)
}

pub fn default_defensive_line_height(m: &Mentality) -> DefensiveLineHeight {
    DefensiveLineHeight::new_clamped(m.value() * LINE_HEIGHT_SENSITIVITY)
}

pub fn default_directness(m: &Mentality) -> Directness {
    Directness::new_clamped(m.value() * DIRECTNESS_SENSITIVITY)
}

pub fn default_width(m: &Mentality) -> Width {
    Width::new_clamped(m.value() * WIDTH_SENSITIVITY)
}

pub fn default_pressing_intensity(m: &Mentality) -> PressingIntensity {
    PressingIntensity::new_clamped(0.5 + (m.value() * PRESSING_SENSITIVITY) / 2.0)
}

pub fn default_counter_press_intensity(m: &Mentality) -> CounterPressIntensity {
    CounterPressIntensity::new_clamped(0.5 + (m.value() * COUNTER_PRESS_SENSITIVITY) / 2.0)
}

pub fn default_aggression(m: &Mentality) -> Aggression {
    Aggression::new_clamped(0.5 + (m.value() * AGGRESSION_SENSITIVITY) / 2.0)
}

pub fn default_counter_attack_intensity(m: &Mentality) -> CounterAttackIntensity {
    CounterAttackIntensity::new_clamped(0.5 + (m.value() * COUNTER_ATTACK_SENSITIVITY) / 2.0)
}
