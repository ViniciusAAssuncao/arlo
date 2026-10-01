use super::super::context::validate_match_state;
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState};
use crate::step::StepResult;
use arlo_domain::{KickFoulScoringTier, SecondZone};
use arlo_events::{KickFoulAwarded, MatchEvent};
use uuid::Uuid;

pub fn award_kick_foul_segment(
    input: &MatchInput,
    state: &mut MatchState,
    awarded_team_id: Uuid,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    if state.phase() != MatchPhase::Live {
        return Err(EngineError::InvalidTransition(
            "Kick Foul can only be awarded during live play".into(),
        ));
    }
    let is_home = awarded_team_id == input.home().team_id();
    let offending_team_id = if is_home {
        input.away().team_id()
    } else if awarded_team_id == input.away().team_id() {
        input.home().team_id()
    } else {
        return Err(EngineError::InvalidInput(
            "Kick Foul awarded to an unknown team".into(),
        ));
    };
    let position = state.possession().ball_position_mirim();
    let distance_to_goal = if is_home {
        input.pitch().length_mirim() - position
    } else {
        position
    };
    let zone = input
        .pitch()
        .zone_at_distance_to_goal(distance_to_goal, SecondZone::default_awc().depth_mirim());
    let mut next = state.clone();
    next.award_kick_foul(awarded_team_id)?;
    let events = vec![next.emit(MatchEvent::KickFoulAwarded(KickFoulAwarded::new(
        awarded_team_id,
        offending_team_id,
        KickFoulScoringTier::from_zone(zone),
    )))?];
    *state = next;
    Ok(StepResult::resolved(events))
}
