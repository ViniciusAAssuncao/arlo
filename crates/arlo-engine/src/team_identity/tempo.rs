use arlo_tactics::Tempo;

pub fn effort_multiplier(tempo: Tempo) -> f64 {
    1.0 + tempo.value()
}

pub fn effort_multiplier_from_value(value: f64) -> f64 {
    1.0 + value
}

pub fn huddle_duration_scale(tempo: Tempo) -> f64 {
    1.0 - tempo.value().max(0.0) * 0.5
}
