use super::ratings::RatingIndex;
use super::tuning::*;
use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use arlo_domain::{pitch::ArtroPlacement, AttributeKey, Position};
use arlo_tactics::PlayCall;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Poisson};

pub(super) fn sample_artros(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    selected_play_call: Option<&PlayCall>,
    duration_seconds: f64,
    rng: &mut ChaCha8Rng,
) -> EngineResult<Vec<ArtroPlacement>> {
    let technique = ratings.specialist(offense, Position::Artrine, AttributeKey::DriveTechnique)?;
    let containment = ratings.active_average(defense, AttributeKey::DefensiveContainment, false)?;
    let emphasis = selected_play_call
        .map(|call| *call.decision_emphasis())
        .unwrap_or_else(|| offense.tactics().instructions().default_decision_emphasis());
    let probability = (BASE_ARTRO_PROBABILITY + technique * DRIVE_TECHNIQUE_ARTRO_WEIGHT
        - containment * DEFENSIVE_CONTAINMENT_ARTRO_WEIGHT
        + emphasis.self_carry().value() * CARRY_EMPHASIS_ARTRO_WEIGHT)
        .clamp(MIN_ARTRO_PROBABILITY, MAX_ARTRO_PROBABILITY);
    let lambda = -(1.0 - probability).ln() * duration_seconds / 10.0;
    let distribution = Poisson::new(lambda)
        .map_err(|_| EngineError::InvalidTransition("invalid Artro rate".into()))?;
    let count = distribution.sample(rng) as usize;
    let channels = offense.tactics().instructions().channel_distribution();
    let mut placements = Vec::with_capacity(count);
    for _ in 0..count {
        let roll = rng.gen_range(0.0..1.0);
        let placement = if roll < channels.left() {
            ArtroPlacement::LeftLateral
        } else if roll < channels.left() + channels.central() {
            ArtroPlacement::Central
        } else {
            ArtroPlacement::RightLateral
        };
        placements.push(placement);
    }
    Ok(placements)
}
