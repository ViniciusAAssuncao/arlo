use arlo_tactics::{Tempo, TransitionUrgency};

pub fn effort_multiplier(tempo: Tempo) -> f64 {
    1.0 + tempo.value()
}

pub fn effort_multiplier_from_value(value: f64) -> f64 {
    1.0 + value
}

pub fn individual_transition_effort_multiplier(
    team_multiplier: f64,
    transition_urgency: TransitionUrgency,
) -> f64 {
    team_multiplier * (1.0 + transition_urgency.value())
}

pub fn huddle_duration_scale(tempo: Tempo) -> f64 {
    1.0 - tempo.value().max(0.0) * 0.5
}
