use super::ratings::RatingIndex;
use super::tuning::*;
use crate::error::EngineResult;
use crate::input::TeamInput;
use arlo_domain::{AttributeKey, Position};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, Copy)]
pub(super) struct ReceptionSample {
    pub caught: bool,
    pub distance_mirim: f64,
}

pub(super) fn sample_reception(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    rng: &mut ChaCha8Rng,
) -> EngineResult<ReceptionSample> {
    let passing = ratings.specialist(offense, Position::Passer, AttributeKey::Passing)?;
    let hands = ratings.specialist(offense, Position::Artrine, AttributeKey::HandsReception)?;
    let control = ratings.specialist(offense, Position::Artrine, AttributeKey::ArloControl)?;
    let pressure = ratings.active_average(defense, AttributeKey::PasserPressure, false)?;
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
    })
}
