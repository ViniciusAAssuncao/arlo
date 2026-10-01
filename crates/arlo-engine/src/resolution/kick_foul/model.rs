use super::super::ratings::RatingIndex;
use super::super::shooting_model::conversion_probability;
use crate::error::EngineResult;
use crate::input::TeamInput;
use arlo_domain::{AttributeKey, KickFoulDecisionKind};
use arlo_events::ScoringPost;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

pub(super) struct KickFoulSample {
    pub duration_seconds: f64,
    pub converted: bool,
    pub defense_recovers: bool,
    pub out_of_bounds: bool,
    pub distance_mirim: f64,
}

pub(super) fn default_decision(rng: &mut ChaCha8Rng) -> KickFoulDecisionKind {
    match rng.gen_range(0.0..1.0) {
        roll if roll < 0.55 => KickFoulDecisionKind::Shoot,
        roll if roll < 0.70 => KickFoulDecisionKind::Cross,
        roll if roll < 0.88 => KickFoulDecisionKind::ShortPass,
        _ => KickFoulDecisionKind::LongLaunch,
    }
}

pub(super) fn sample_post(distance_ratio: f64, rng: &mut ChaCha8Rng) -> ScoringPost {
    let goalpost_probability = (0.65 - distance_ratio * 0.45).clamp(0.15, 0.65);
    if rng.gen_range(0.0..1.0) < goalpost_probability {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    }
}

pub(super) fn sample_attempt(
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    taker_id: Uuid,
    receiver_id: Uuid,
    decision: KickFoulDecisionKind,
    post: Option<ScoringPost>,
    distance_ratio: f64,
    rng: &mut ChaCha8Rng,
) -> EngineResult<KickFoulSample> {
    let duration_seconds = 2.0 + rng.gen_range(0.0..1.0) * 4.0;
    let (converted, distance_mirim) = if let Some(post) = post {
        let probability =
            conversion_probability(ratings, offense, defense, taker_id, post, distance_ratio)?;
        (rng.gen_range(0.0..1.0) < probability, 0.0)
    } else {
        let passing = ratings.player_value(offense, taker_id, AttributeKey::Passing)?;
        let hands = ratings.player_value(offense, receiver_id, AttributeKey::HandsReception)?;
        let containment =
            ratings.active_average(defense, AttributeKey::DefensiveContainment, false)?;
        let distance_mirim = match decision {
            KickFoulDecisionKind::ShortPass => 1.0 + rng.gen_range(0.0..1.0) * 3.0,
            KickFoulDecisionKind::Cross => 4.0 + rng.gen_range(0.0..1.0) * 8.0,
            KickFoulDecisionKind::LongLaunch => 10.0 + rng.gen_range(0.0..1.0) * 15.0,
            KickFoulDecisionKind::Shoot => 0.0,
        };
        let probability =
            (0.70 + passing * 0.012 + hands * 0.008 - containment * 0.012 - distance_mirim * 0.011)
                .clamp(0.18, 0.92);
        (rng.gen_range(0.0..1.0) < probability, distance_mirim)
    };
    Ok(KickFoulSample {
        duration_seconds,
        converted,
        defense_recovers: rng.gen_range(0.0..1.0) < 0.62,
        out_of_bounds: rng.gen_range(0.0..1.0) < 0.08,
        distance_mirim,
    })
}
