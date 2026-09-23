use super::ratings::TeamRatings;
use super::tuning::*;
use crate::input::TeamInput;
use arlo_tactics::PlayCall;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy)]
pub(super) struct SampledCall {
    pub gain_mirim: f64,
    pub duration_seconds: f64,
}

pub(super) fn sample_call(
    offense: &TeamInput,
    offense_rating: TeamRatings,
    defense_rating: TeamRatings,
    is_home: bool,
    selected_play_call: Option<&PlayCall>,
    rng: &mut ChaCha8Rng,
) -> SampledCall {
    let instructions = offense.tactics().instructions().in_possession();
    let mentality = instructions.mentality().value();
    let tempo = instructions.tempo().value();
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| offense.tactics().instructions().default_decision_emphasis());
    let carry_bias = emphasis.self_carry().value() - 0.5;
    let success_probability = (BASE_SUCCESS_PROBABILITY
        + (offense_rating.offense - defense_rating.defense) * ATTRIBUTE_DIFFERENCE_WEIGHT
        + if is_home { HOME_ADVANTAGE } else { 0.0 }
        + mentality * MENTALITY_WEIGHT
        + carry_bias * CARRY_EMPHASIS_WEIGHT)
        .clamp(MIN_SUCCESS_PROBABILITY, MAX_SUCCESS_PROBABILITY);

    let successful = rng.gen_range(0.0..1.0) < success_probability;
    let gain_mirim = if successful {
        SUCCESS_GAIN_MIN_MIRIM + rng.gen_range(0.0..1.0) * SUCCESS_GAIN_RANGE_MIRIM
    } else {
        FAILURE_GAIN_MIN_MIRIM + rng.gen_range(0.0..1.0) * FAILURE_GAIN_RANGE_MIRIM
    };
    let duration_seconds = (DURATION_MIN_SECONDS
        + rng.gen_range(0.0..1.0) * DURATION_RANGE_SECONDS)
        / (1.0 + tempo * TEMPO_DURATION_WEIGHT);
    SampledCall {
        gain_mirim,
        duration_seconds,
    }
}
