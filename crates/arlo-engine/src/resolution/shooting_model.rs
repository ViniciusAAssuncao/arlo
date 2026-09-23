use super::ratings::RatingIndex;
use super::tuning::*;
use crate::error::EngineResult;
use crate::input::TeamInput;
use arlo_domain::sport_constants::{
    BONUS_FIELDPOST_DISTANCE_TO_GOAL_MIRIM, BONUS_GOALPOST_DISTANCE_TO_GOAL_MIRIM,
};
use arlo_domain::{AttributeKey, Position};
use arlo_events::ScoringPost;
use arlo_tactics::PlayCall;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy)]
pub(super) struct ShotSample {
    pub post: ScoringPost,
    pub converted: bool,
    pub defense_recovers: bool,
    pub out_of_bounds: bool,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct BonusShotSample {
    pub post: ScoringPost,
    pub converted: bool,
    pub duration_seconds: f64,
}

pub(super) fn sample_bonus_shot(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    pitch_length_mirim: f64,
    rng: &mut ChaCha8Rng,
) -> EngineResult<BonusShotSample> {
    let post = if rng.gen_range(0.0..1.0) < BONUS_GOALPOST_CHOICE_PROBABILITY {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    };
    let distance = match post {
        ScoringPost::Goalpost => BONUS_GOALPOST_DISTANCE_TO_GOAL_MIRIM,
        ScoringPost::Fieldpost => BONUS_FIELDPOST_DISTANCE_TO_GOAL_MIRIM,
    };
    let probability = conversion_probability(
        ratings,
        offense,
        defense,
        post,
        distance / pitch_length_mirim,
    )?;
    Ok(BonusShotSample {
        post,
        converted: rng.gen_range(0.0..1.0) < probability,
        duration_seconds: BONUS_DURATION_MIN_SECONDS
            + rng.gen_range(0.0..1.0) * BONUS_DURATION_RANGE_SECONDS,
    })
}

pub(super) fn sample_regular_shot(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    selected_play_call: Option<&PlayCall>,
    distance_to_goal_mirim: f64,
    pitch_length_mirim: f64,
    has_drive: bool,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Option<ShotSample>> {
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| offense.tactics().instructions().default_decision_emphasis());
    let proximity = 1.0 - (distance_to_goal_mirim / pitch_length_mirim).clamp(0.0, 1.0);
    let attempt_probability = (BASE_SHOT_ATTEMPT_PROBABILITY
        + emphasis.self_finish().value() * FINISH_EMPHASIS_SHOT_WEIGHT
        + proximity * TERRITORY_SHOT_WEIGHT)
        .clamp(MIN_SHOT_ATTEMPT_PROBABILITY, MAX_SHOT_ATTEMPT_PROBABILITY);
    if rng.gen_range(0.0..1.0) >= attempt_probability {
        return Ok(None);
    }
    let post = if has_drive
        && rng.gen_range(0.0..1.0)
            < BASE_GOALPOST_CHOICE_PROBABILITY + proximity * TERRITORY_GOALPOST_CHOICE_WEIGHT
    {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    };
    let conversion_probability = conversion_probability(
        ratings,
        offense,
        defense,
        post,
        distance_to_goal_mirim / pitch_length_mirim,
    )?;
    Ok(Some(ShotSample {
        post,
        converted: rng.gen_range(0.0..1.0) < conversion_probability,
        defense_recovers: rng.gen_range(0.0..1.0) < DEFENSIVE_REBOUND_PROBABILITY,
        out_of_bounds: rng.gen_range(0.0..1.0) < MISSED_SHOT_OUT_PROBABILITY,
    }))
}

pub(super) fn conversion_probability(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    post: ScoringPost,
    distance_ratio: f64,
) -> EngineResult<f64> {
    let probability = match post {
        ScoringPost::Goalpost => {
            let finishing =
                ratings.specialist(offense, Position::Artrine, AttributeKey::Finishing)?;
            let reflexes =
                ratings.specialist(defense, Position::Goalguard, AttributeKey::Reflexes)?;
            BASE_GOALPOST_CONVERSION + finishing * FINISHING_CONVERSION_WEIGHT
                - reflexes * GOALGUARD_CONVERSION_WEIGHT
                - distance_ratio * DISTANCE_CONVERSION_WEIGHT
        }
        ScoringPost::Fieldpost => {
            let finishing =
                ratings.specialist(offense, Position::Artrine, AttributeKey::Finishing)?;
            let technique =
                ratings.specialist(offense, Position::Artrine, AttributeKey::Technique)?;
            let blocking =
                ratings.active_average(defense, AttributeKey::DefensiveContainment, false)?;
            BASE_FIELDPOST_CONVERSION + (finishing + technique) * 0.5 * KICKING_CONVERSION_WEIGHT
                - blocking * BLOCKING_CONVERSION_WEIGHT
                - distance_ratio * DISTANCE_CONVERSION_WEIGHT
        }
    };
    Ok(probability.clamp(MIN_SHOT_CONVERSION, MAX_SHOT_CONVERSION))
}
