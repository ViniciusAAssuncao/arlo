use arlo_domain::sport_constants::FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_GOALPOST;
use arlo_events::ScoringPost;

pub fn select_kick_post(finisher_rating: f64, territory_advance_mirim: f64) -> ScoringPost {
    if territory_advance_mirim >= FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_GOALPOST && finisher_rating >= 12.0 {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    }
}