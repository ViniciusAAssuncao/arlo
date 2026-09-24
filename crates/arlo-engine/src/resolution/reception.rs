use super::ratings::RatingIndex;
use super::tuning::*;
use crate::error::EngineResult;
use crate::input::TeamInput;
use arlo_domain::AttributeKey;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) struct ReceptionSample {
    pub caught: bool,
    pub distance_mirim: f64,
    pub probability: f64,
    pub contested: bool,
}

pub(super) fn sample_reception(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    passer_id: Uuid,
    receiver_id: Uuid,
    defender_id: Uuid,
    rng: &mut ChaCha8Rng,
) -> EngineResult<ReceptionSample> {
    let passing = ratings.player_value(offense, passer_id, AttributeKey::Passing)?;
    let hands = ratings.player_value(offense, receiver_id, AttributeKey::HandsReception)?;
    let control = ratings.player_value(offense, receiver_id, AttributeKey::ArloControl)?;
    let pressing = defense
        .tactics()
        .instructions()
        .out_of_possession()
        .pressing_intensity()
        .value();
    let contested = rng.gen_range(0.0..1.0) < 0.18 + 0.35 * pressing;
    let pressure = ratings.player_value(defense, defender_id, AttributeKey::PasserPressure)?
        * if contested { 0.8 + 0.4 * pressing } else { 0.2 };
    let probability = (BASE_RECEPTION_PROBABILITY
        + passing * PASSING_RECEPTION_WEIGHT
        + hands * HANDS_RECEPTION_WEIGHT
        + control * CONTROL_RECEPTION_WEIGHT
        - pressure * DEFENSIVE_PRESSURE_RECEPTION_WEIGHT)
        .clamp(MIN_RECEPTION_PROBABILITY, MAX_RECEPTION_PROBABILITY);
    let caught = rng.gen_range(0.0..1.0) < probability;
    let distance_mirim =
        PASS_DISTANCE_MIN_MIRIM + rng.gen_range(0.0..1.0) * PASS_DISTANCE_RANGE_MIRIM;
    Ok(ReceptionSample {
        caught,
        distance_mirim,
        probability,
        contested,
    })
}
