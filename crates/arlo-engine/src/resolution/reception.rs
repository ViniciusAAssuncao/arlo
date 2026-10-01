use super::ratings::RatingIndex;
use super::tuning::*;
use crate::error::EngineResult;
use crate::input::TeamInput;
use arlo_domain::AttributeKey;
use arlo_events::DuelKind;
use arlo_tactics::PlayCall;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DistributionIntent {
    Route,
    Short,
    Long,
    Cross,
}

impl DistributionIntent {
    pub(super) fn duel_kind(self, is_aerial: bool) -> DuelKind {
        match self {
            Self::Route if is_aerial => DuelKind::AerialDuel,
            Self::Route => DuelKind::RouteContest,
            Self::Short => DuelKind::ShortDistribution,
            Self::Long => DuelKind::LongDistribution,
            Self::Cross => DuelKind::CrossDistribution,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ReceptionSample {
    pub caught: bool,
    pub distance_mirim: f64,
    pub probability: f64,
    pub contested: bool,
    pub is_aerial: bool,
    pub intent: DistributionIntent,
}

pub(super) fn sample_distribution_intent(
    ratings: &RatingIndex,
    offense: &TeamInput,
    selected_play_call: Option<&PlayCall>,
    rng: &mut ChaCha8Rng,
) -> DistributionIntent {
    let team_instructions = ratings.instructions(offense);
    let instructions = team_instructions.in_possession();
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| team_instructions.default_decision_emphasis());

    let passing_range = instructions.passing_range().value();
    let directness = instructions.directness().value();
    let aeriality = instructions.aeriality().value();
    let width = instructions.width().value();
    let structure = instructions.structure().value();

    let route_weight = 0.90 + structure * 0.35;
    let short_weight =
        0.35 + emphasis.short_pass().value() + (1.0 - passing_range) * 0.30;
    let long_weight = (0.12
        + emphasis.long_launch().value() * 0.75
        + passing_range * 0.45
        + directness * 0.20)
        .max(0.05);
    let cross_weight = (0.10
        + emphasis.cross().value() * 0.65
        + aeriality * 0.30
        + width * 0.20)
        .max(0.05);

    let mut draw = rng.gen_range(0.0..route_weight + short_weight + long_weight + cross_weight);
    if draw < route_weight {
        return DistributionIntent::Route;
    }
    draw -= route_weight;
    if draw < short_weight {
        return DistributionIntent::Short;
    }
    draw -= short_weight;
    if draw < long_weight {
        DistributionIntent::Long
    } else {
        DistributionIntent::Cross
    }
}

pub(super) fn sample_reception(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    passer_id: Uuid,
    receiver_id: Uuid,
    defender_id: Uuid,
    intent: DistributionIntent,
    rng: &mut ChaCha8Rng,
) -> EngineResult<ReceptionSample> {
    let passing = ratings.player_value(offense, passer_id, AttributeKey::Passing)?;
    let hands = ratings.player_value(offense, receiver_id, AttributeKey::HandsReception)?;
    let control = ratings.player_value(offense, receiver_id, AttributeKey::ArloControl)?;
    let team_instructions = ratings.instructions(offense);
    let instructions = team_instructions.in_possession();
    let passing_range = instructions.passing_range().value();
    let aeriality = instructions.aeriality().value();
    let crossing = ratings.player_value(offense, passer_id, AttributeKey::Crossing)?;
    let reach = ratings.player_value(offense, receiver_id, AttributeKey::JumpingReach)?;
    let pressing = ratings
        .instructions(defense)
        .out_of_possession()
        .pressing_intensity()
        .value();

    let contest_bias = match intent {
        DistributionIntent::Route => 0.0,
        DistributionIntent::Short => -0.05,
        DistributionIntent::Long => 0.08,
        DistributionIntent::Cross => 0.12,
    };
    let contested =
        rng.gen_range(0.0..1.0) < (0.18 + 0.35 * pressing + contest_bias).clamp(0.08, 0.72);
    let pressure = ratings.player_value(defense, defender_id, AttributeKey::PasserPressure)?
        * if contested { 0.8 + 0.4 * pressing } else { 0.2 };

    let aerial_probability = match intent {
        DistributionIntent::Route => 0.08 + aeriality * 0.35,
        DistributionIntent::Short => 0.02 + aeriality * 0.10,
        DistributionIntent::Long => 0.30 + aeriality * 0.45,
        DistributionIntent::Cross => 0.45 + aeriality * 0.45,
    }
    .clamp(0.0, 0.92);
    let is_aerial = rng.gen_range(0.0..1.0) < aerial_probability;

    let intent_penalty = match intent {
        DistributionIntent::Route => 0.01,
        DistributionIntent::Short => 0.0,
        DistributionIntent::Long => 0.10,
        DistributionIntent::Cross => 0.07,
    };
    let aerial_skill = if is_aerial {
        ((crossing + reach) * 0.5 - 10.0) * 0.004
    } else {
        0.0
    };
    let probability = ratings.reliable_probability(
        receiver_id,
        (BASE_RECEPTION_PROBABILITY
            + passing * PASSING_RECEPTION_WEIGHT
            + hands * HANDS_RECEPTION_WEIGHT
            + control * CONTROL_RECEPTION_WEIGHT
            - pressure * DEFENSIVE_PRESSURE_RECEPTION_WEIGHT
            - passing_range * 0.012
            + aerial_skill
            - intent_penalty)
            .clamp(MIN_RECEPTION_PROBABILITY, MAX_RECEPTION_PROBABILITY),
        MAX_RECEPTION_PROBABILITY,
    );

    let caught = rng.gen_range(0.0..1.0) < probability;
    let (distance_min, distance_range) = match intent {
        DistributionIntent::Route => (
            ROUTE_PASS_DISTANCE_MIN_MIRIM,
            ROUTE_PASS_DISTANCE_RANGE_MIRIM,
        ),
        DistributionIntent::Short => (
            SHORT_PASS_DISTANCE_MIN_MIRIM,
            SHORT_PASS_DISTANCE_RANGE_MIRIM,
        ),
        DistributionIntent::Long => (
            LONG_PASS_DISTANCE_MIN_MIRIM,
            LONG_PASS_DISTANCE_RANGE_MIRIM,
        ),
        DistributionIntent::Cross => (
            CROSS_PASS_DISTANCE_MIN_MIRIM,
            CROSS_PASS_DISTANCE_RANGE_MIRIM,
        ),
    };
    let distance_mirim = (distance_min + rng.gen_range(0.0..1.0) * distance_range)
        * (0.90 + passing_range * 0.20);

    Ok(ReceptionSample {
        caught,
        distance_mirim,
        probability,
        contested,
        is_aerial,
        intent,
    })
}
