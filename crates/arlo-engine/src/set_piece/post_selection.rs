use crate::scoring_model::{select_post_for_field_goal, ScoringSituation};
use arlo_domain::PitchZone;
use arlo_events::ScoringPost;

pub fn select_kick_post(finisher_rating: f64, territory_advance_mirim: f64) -> ScoringPost {
    let dummy_situation = ScoringSituation::new(
        PitchZone::FirstZone,
        0.9,
        0,
        territory_advance_mirim,
        finisher_rating,
        10.0,
        false,
    );
    select_post_for_field_goal(&dummy_situation)
}