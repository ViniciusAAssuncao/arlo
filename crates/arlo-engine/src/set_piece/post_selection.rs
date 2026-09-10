use crate::weighting::calculate_weighted_average;
use arlo_domain::sport_constants::{
    KICK_POST_DISTANCE_NORMALIZATION_CAP, KICK_POST_DISTANCE_WEIGHT,
    KICK_POST_GOALPOST_SCORE_THRESHOLD, KICK_POST_RATING_NORMALIZATION_CAP,
    KICK_POST_RATING_WEIGHT,
};
use arlo_events::ScoringPost;

pub fn select_kick_post(finisher_rating: f64, territory_advance_mirim: f64) -> ScoringPost {
    let normalized_rating =
        (finisher_rating / KICK_POST_RATING_NORMALIZATION_CAP).clamp(0.0, 1.0);
    let normalized_distance =
        (territory_advance_mirim / KICK_POST_DISTANCE_NORMALIZATION_CAP).clamp(0.0, 1.0);

    let items = [
        (normalized_rating, KICK_POST_RATING_WEIGHT),
        (normalized_distance, KICK_POST_DISTANCE_WEIGHT),
    ];

    let score = calculate_weighted_average(&items).unwrap_or(0.0);
    if score >= KICK_POST_GOALPOST_SCORE_THRESHOLD {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    }
}