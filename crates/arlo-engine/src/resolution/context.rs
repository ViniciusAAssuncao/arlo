use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::MatchState;

pub(super) fn validate_match_state(input: &MatchInput, state: &MatchState) -> EngineResult<()> {
    if input.match_id() != state.match_id()
        || input.home().team_id() != state.home().team_id()
        || input.away().team_id() != state.away().team_id()
    {
        return Err(EngineError::InvalidInput(
            "state and match input differ".into(),
        ));
    }
    Ok(())
}
