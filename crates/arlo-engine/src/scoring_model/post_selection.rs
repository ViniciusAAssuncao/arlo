use crate::scoring_model::scoring_situation::ScoringSituation;
use arlo_domain::PitchZone;
use arlo_events::ScoringPost;

pub fn select_post_for_field_goal(situation: &ScoringSituation) -> ScoringPost {
    if situation.zone == PitchZone::FirstZone && situation.finisher_rating >= 12.0 {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    }
}