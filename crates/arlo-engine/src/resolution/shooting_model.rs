use super::actors::select_primary_defender;
use super::goalguard::active_goalguard_id;
use super::ratings::RatingIndex;
use super::tuning::*;
use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use arlo_domain::sport_constants::{
    BONUS_FIELDPOST_DISTANCE_TO_GOAL_MIRIM, BONUS_GOALPOST_DISTANCE_TO_GOAL_MIRIM,
};
use arlo_domain::{AttributeKey, PositionLine};
use arlo_events::ScoringPost;
use arlo_tactics::PlayCall;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) struct ShotSample {
    pub post: ScoringPost,
    pub shooter_id: Uuid,
    pub defender_id: Uuid,
    pub converted: bool,
    pub defense_recovers: bool,
    pub out_of_bounds: bool,
    pub conversion_probability: f64,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct BonusShotSample {
    pub post: ScoringPost,
    pub defender_id: Uuid,
    pub converted: bool,
    pub duration_seconds: f64,
    pub conversion_probability: f64,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ScoringContestSample {
    pub defender_id: Uuid,
    pub converted: bool,
    pub conversion_probability: f64,
}

pub(super) fn sample_scoring_contest(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    shooter_id: Uuid,
    post: ScoringPost,
    distance_ratio: f64,
    rng: &mut ChaCha8Rng,
) -> EngineResult<ScoringContestSample> {
    let defender_id = select_shot_defender(ratings, defense, post, rng)?;
    let conversion_probability = conversion_probability(
        ratings,
        offense,
        defense,
        shooter_id,
        defender_id,
        post,
        distance_ratio,
    )?;
    Ok(ScoringContestSample {
        defender_id,
        converted: rng.gen_range(0.0..1.0) < conversion_probability,
        conversion_probability,
    })
}

pub(super) fn sample_bonus_shot(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    kicker_id: Uuid,
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
    let contest = sample_scoring_contest(
        ratings,
        offense,
        defense,
        kicker_id,
        post,
        distance / pitch_length_mirim,
        rng,
    )?;
    Ok(BonusShotSample {
        post,
        defender_id: contest.defender_id,
        converted: contest.converted,
        duration_seconds: BONUS_DURATION_MIN_SECONDS
            + rng.gen_range(0.0..1.0) * BONUS_DURATION_RANGE_SECONDS,
        conversion_probability: contest.conversion_probability,
    })
}

pub(super) fn sample_regular_shot(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    shooter_id: Uuid,
    selected_play_call: Option<&PlayCall>,
    distance_to_goal_mirim: f64,
    pitch_length_mirim: f64,
    has_drive: bool,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Option<ShotSample>> {
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| ratings.instructions(offense).default_decision_emphasis());
    let finishing = ratings.player_value(offense, shooter_id, AttributeKey::Finishing)?;
    let composure = ratings.player_value(offense, shooter_id, AttributeKey::Composure)?;
    let anticipation = ratings.player_value(offense, shooter_id, AttributeKey::Anticipation)?;
    let shooter_readiness = ((0.50 * finishing + 0.30 * composure + 0.20 * anticipation - 10.0)
        / 10.0)
        .clamp(-0.8, 1.0);
    let proximity = 1.0 - (distance_to_goal_mirim / pitch_length_mirim).clamp(0.0, 1.0);
    let patience = ratings
        .instructions(offense)
        .in_possession()
        .scoring_patience()
        .value();
    let attempt_probability = (BASE_SHOT_ATTEMPT_PROBABILITY
        + emphasis.self_finish().value() * FINISH_EMPHASIS_SHOT_WEIGHT
        + shooter_readiness * SHOOTER_READINESS_ATTEMPT_WEIGHT
        + proximity * TERRITORY_SHOT_WEIGHT)
        .clamp(MIN_SHOT_ATTEMPT_PROBABILITY, MAX_SHOT_ATTEMPT_PROBABILITY)
        * if has_drive { 1.0 } else { 1.0 - 0.5 * patience };
    if rng.gen_range(0.0..1.0) >= attempt_probability {
        return Ok(None);
    }
    let goalpost_probability = (BASE_GOALPOST_CHOICE_PROBABILITY
        + proximity * TERRITORY_GOALPOST_CHOICE_WEIGHT
        + shooter_readiness * SHOOTER_READINESS_GOALPOST_WEIGHT
        + (emphasis.self_finish().value() - 0.5) * TACTICAL_GOALPOST_CHOICE_WEIGHT
        + (patience - 0.5) * TACTICAL_GOALPOST_CHOICE_WEIGHT)
        .clamp(
            MIN_GOALPOST_CHOICE_PROBABILITY,
            MAX_GOALPOST_CHOICE_PROBABILITY,
        );
    let post = if has_drive && rng.gen_range(0.0..1.0) < goalpost_probability {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    };
    let contest = sample_scoring_contest(
        ratings,
        offense,
        defense,
        shooter_id,
        post,
        distance_to_goal_mirim / pitch_length_mirim,
        rng,
    )?;
    Ok(Some(ShotSample {
        post,
        shooter_id,
        defender_id: contest.defender_id,
        converted: contest.converted,
        defense_recovers: rng.gen_range(0.0..1.0) < DEFENSIVE_REBOUND_PROBABILITY,
        out_of_bounds: rng.gen_range(0.0..1.0) < MISSED_SHOT_OUT_PROBABILITY,
        conversion_probability: contest.conversion_probability,
    }))
}

fn select_shot_defender(
    ratings: &RatingIndex,
    defense: &TeamInput,
    post: ScoringPost,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Uuid> {
    match post {
        ScoringPost::Goalpost => {
            if let Some(goalguard_id) = active_goalguard_id(ratings, defense) {
                Ok(goalguard_id)
            } else {
                select_primary_defender(ratings, defense, rng)
            }
        }
        ScoringPost::Fieldpost => {
            let has_active_field_defender =
                ratings
                    .lineup(defense)
                    .assignments()
                    .iter()
                    .any(|assignment| {
                        assignment.position().line() != PositionLine::Goalguard
                            && ratings.is_active_slot(defense, assignment.player_id())
                    });
            if has_active_field_defender {
                select_primary_defender(ratings, defense, rng)
            } else {
                active_goalguard_id(ratings, defense).ok_or_else(|| {
                    EngineError::InvalidInput("lineup has no active shot defender".into())
                })
            }
        }
    }
}

fn conversion_probability(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    shooter_id: Uuid,
    defender_id: Uuid,
    post: ScoringPost,
    distance_ratio: f64,
) -> EngineResult<f64> {
    let probability = match post {
        ScoringPost::Goalpost => {
            let finishing = ratings.player_value(offense, shooter_id, AttributeKey::Finishing)?;
            let composure = ratings.player_value(offense, shooter_id, AttributeKey::Composure)?;
            let reflexes = ratings.player_value(defense, defender_id, AttributeKey::Reflexes)?;
            BASE_GOALPOST_CONVERSION
                + finishing * FINISHING_CONVERSION_WEIGHT
                + (composure - 10.0) * COMPOSURE_GOALPOST_CONVERSION_WEIGHT
                - reflexes * GOALGUARD_CONVERSION_WEIGHT
                - distance_ratio * DISTANCE_CONVERSION_WEIGHT
        }
        ScoringPost::Fieldpost => {
            let finishing = ratings.player_value(offense, shooter_id, AttributeKey::Finishing)?;
            let technique = ratings.player_value(offense, shooter_id, AttributeKey::Technique)?;
            let containment =
                ratings.player_value(defense, defender_id, AttributeKey::DefensiveContainment)?;
            BASE_FIELDPOST_CONVERSION + (finishing + technique) * 0.5 * KICKING_CONVERSION_WEIGHT
                - containment * BLOCKING_CONVERSION_WEIGHT
                - distance_ratio * DISTANCE_CONVERSION_WEIGHT
        }
    };
    Ok(ratings.reliable_probability(
        shooter_id,
        probability.clamp(MIN_SHOT_CONVERSION, MAX_SHOT_CONVERSION),
        MAX_SHOT_CONVERSION,
    ))
}
