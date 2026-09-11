use crate::match_decision::event_translation::translate_duel_kind;
use crate::officiating::foul::FoulResolution;
use arlo_events::FoulRaised;

pub fn translate_foul_raised(resolution: &FoulResolution) -> FoulRaised {
    FoulRaised::new(
        resolution.offending_player_id,
        resolution.offending_team_id,
        resolution.opposing_player_id,
        resolution.opposing_team_id,
        translate_duel_kind(resolution.engine_duel_kind),
        resolution.original_call_correct,
        resolution.peace_referee_intervened,
    )
}