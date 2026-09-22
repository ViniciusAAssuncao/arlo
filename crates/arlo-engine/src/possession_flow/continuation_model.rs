use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::physical::PhysicalState;
use crate::team_identity::pressing::contest_radius_multiplier;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use arlo_tactics::{PassingRange, PressingIntensity, Tempo};
use rand::Rng;

pub const CONTINUATION_BASE_LOGIT: f64 = 1.00;
pub const CONTINUATION_CONTROL_WEIGHT: f64 = 1.50;
pub const CONTINUATION_TEMPO_WEIGHT: f64 = 0.40;
pub const CONTINUATION_PASSING_RANGE_WEIGHT: f64 = 0.35;
pub const CONTINUATION_URGENCY_WEIGHT: f64 = 0.25;
pub const CONTINUATION_RISK_WEIGHT: f64 = 0.20;
pub const CONTINUATION_FATIGUE_PENALTY_WEIGHT: f64 = 1.00;
pub const CONTINUATION_PRESSING_PENALTY_WEIGHT: f64 = 1.20;

pub fn evaluate_continuation_probability(
    offense_control_power: f64,
    offense_tempo: Tempo,
    offense_passing_range: PassingRange,
    game_state_pressure: &GameStatePressure,
    carrier_physical_state: &PhysicalState,
    defense_pressing_intensity: PressingIntensity,
) -> Probability {
    let control_advantage = (offense_control_power - 1.0) * CONTINUATION_CONTROL_WEIGHT;
    let tactical_sustain = -(offense_tempo.value() * CONTINUATION_TEMPO_WEIGHT)
        - (offense_passing_range.value() * CONTINUATION_PASSING_RANGE_WEIGHT);
    let urgency_pressure = -(game_state_pressure.urgency_index() * CONTINUATION_URGENCY_WEIGHT)
        - (game_state_pressure.offensive_risk_bias() * CONTINUATION_RISK_WEIGHT);
    let exhaustion = calculate_physical_exhaustion(carrier_physical_state);
    let fatigue_penalty = -(exhaustion * CONTINUATION_FATIGUE_PENALTY_WEIGHT);
    let pressing_mult = contest_radius_multiplier(defense_pressing_intensity);
    let defense_pressure = -((pressing_mult - 1.0) * CONTINUATION_PRESSING_PENALTY_WEIGHT);

    let logit = CONTINUATION_BASE_LOGIT
        + control_advantage
        + tactical_sustain
        + urgency_pressure
        + fatigue_penalty
        + defense_pressure;

    Probability::new_clamped(logistic(logit))
}

pub fn sample_continuation<R: Rng + ?Sized>(
    offense_control_power: f64,
    offense_tempo: Tempo,
    offense_passing_range: PassingRange,
    game_state_pressure: &GameStatePressure,
    carrier_physical_state: &PhysicalState,
    defense_pressing_intensity: PressingIntensity,
    rng: &mut R,
) -> bool {
    evaluate_continuation_probability(
        offense_control_power,
        offense_tempo,
        offense_passing_range,
        game_state_pressure,
        carrier_physical_state,
        defense_pressing_intensity,
    )
    .sample(rng)
}