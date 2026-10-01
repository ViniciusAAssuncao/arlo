use super::context::validate_match_state;
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::MatchState;
use crate::step::StepResult;
use arlo_domain::sport_constants::GOAL_POINT_REQUIRED_DRIVES;
use arlo_events::{MatchEvent, ScoringAttemptMissed, ScoringPost, Turnover};
use uuid::Uuid;

pub fn resolve_missed_shot_recovery(
    input: &MatchInput,
    state: &mut MatchState,
    shooter_id: Uuid,
    attempted_post: ScoringPost,
    recovering_team_id: Uuid,
    recovery_position_mirim: f64,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    let shooting_team_id = state.possessor_team_id();
    let shooter_is_active = if shooting_team_id == input.home().team_id() {
        state.home().active_player_ids().contains(&shooter_id)
    } else {
        state.away().active_player_ids().contains(&shooter_id)
    };
    if !shooter_is_active {
        return Err(EngineError::InvalidInput(
            "shot was not taken by an active player".into(),
        ));
    }
    let shooting_team = if shooting_team_id == input.home().team_id() {
        state.home()
    } else {
        state.away()
    };
    if attempted_post == ScoringPost::Goalpost
        && shooting_team.drive_progress().completed_drives() < GOAL_POINT_REQUIRED_DRIVES
    {
        return Err(EngineError::InvalidTransition(
            "regular Goal Point attempt requires a Drive".into(),
        ));
    }
    let mut next = state.clone();
    next.recover_missed_shot(
        shooting_team_id,
        recovering_team_id,
        recovery_position_mirim,
    )?;
    let mut events =
        vec![
            next.emit(MatchEvent::ScoringAttemptMissed(ScoringAttemptMissed::new(
                shooting_team_id,
                shooter_id,
                attempted_post,
            )))?,
        ];
    if recovering_team_id != shooting_team_id {
        events.push(next.emit(MatchEvent::Turnover(Turnover::new(
            shooting_team_id,
            recovering_team_id,
            None,
            None,
            true,
        )))?);
    }
    *state = next;
    super::officiating::resolve_officiating(input, state, StepResult::resolved(events), None)
}
