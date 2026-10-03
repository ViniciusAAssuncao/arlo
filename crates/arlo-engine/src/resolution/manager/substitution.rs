use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::resolution::context::validate_match_state;
use crate::state::MatchState;
use crate::step::StepResult;
use arlo_events::{MatchClockInstant, MatchEvent, SubstitutionMade, SubstitutionReason};
use arlo_manager_control::SubstitutionIntent;
use uuid::Uuid;

pub fn resolve_substitution_segment(
    input: &MatchInput,
    state: &mut MatchState,
    team_id: Uuid,
    intents: &[SubstitutionIntent],
    reason: SubstitutionReason,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    let mut next = state.clone();
    let mut events = Vec::with_capacity(intents.len());
    for intent in intents {
        next.substitute_voluntarily(
            team_id,
            intent.outgoing_player_id(),
            intent.incoming_player_id(),
        )?;
        let clock = MatchClockInstant::with_total_elapsed_seconds(
            next.clock().period(),
            next.clock().seconds_in_period(),
            next.clock().total_elapsed_seconds(),
        );
        events.push(
            next.emit(MatchEvent::SubstitutionMade(SubstitutionMade::new(
                team_id,
                intent.outgoing_player_id(),
                intent.incoming_player_id(),
                clock,
                reason,
            )))?,
        );
    }
    *state = next;
    Ok(StepResult::resolved(events))
}
