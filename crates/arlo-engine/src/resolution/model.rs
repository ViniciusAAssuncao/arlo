use super::ratings::RatingIndex;
use super::ratings::TeamRatings;
use super::tuning::*;
use crate::error::EngineResult;
use crate::input::TeamInput;
use arlo_domain::AttributeKey;
use arlo_tactics::PlayCall;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) struct SampledCall {
    pub gain_mirim: f64,
    pub duration_seconds: f64,
    pub successful: bool,
    pub success_probability: f64,
    pub contested: bool,
}

pub(super) fn sample_call(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    offense_rating: TeamRatings,
    defense_rating: TeamRatings,
    carrier_id: Uuid,
    defender_id: Uuid,
    is_home: bool,
    selected_play_call: Option<&PlayCall>,
    rng: &mut ChaCha8Rng,
) -> EngineResult<SampledCall> {
    let instructions = offense.tactics().instructions().in_possession();
    let mentality = instructions.mentality().value();
    let tempo = instructions.tempo().value();
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| offense.tactics().instructions().default_decision_emphasis());
    let carry_bias = emphasis.self_carry().value() - 0.5;
    let pressing = defense
        .tactics()
        .instructions()
        .out_of_possession()
        .pressing_intensity()
        .value();
    let contest_probability = (0.16 + pressing * 0.32 + carry_bias * 0.08).clamp(0.10, 0.55);
    let contested = rng.gen_range(0.0..1.0) < contest_probability;
    let carrier_ability =
        0.45 * ratings.player_value(offense, carrier_id, AttributeKey::ArloControl)?
            + 0.35 * ratings.player_value(offense, carrier_id, AttributeKey::Dribbling)?
            + 0.10 * ratings.player_value(offense, carrier_id, AttributeKey::Balance)?
            + 0.10 * ratings.player_value(offense, carrier_id, AttributeKey::Acceleration)?;
    let defender_ability =
        (ratings.player_value(defense, defender_id, AttributeKey::DefensiveContainment)?
            + ratings.player_value(defense, defender_id, AttributeKey::Pace)?)
            * 0.5;
    let success_probability = (BASE_SUCCESS_PROBABILITY
        + (offense_rating.offense - defense_rating.defense) * ATTRIBUTE_DIFFERENCE_WEIGHT
        + (carrier_ability - offense_rating.offense) * INDIVIDUAL_CARRY_WEIGHT
        - if contested {
            (defender_ability - defense_rating.defense) * INDIVIDUAL_CARRY_WEIGHT
        } else {
            0.0
        }
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
    Ok(SampledCall {
        gain_mirim,
        duration_seconds,
        successful,
        success_probability,
        contested,
    })
}
