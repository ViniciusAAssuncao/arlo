use crate::error::{EngineError, EngineResult};
use crate::{MatchInput, MatchState};
use arlo_domain::AttributeKey;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub(super) struct PlayerFactors {
    physical: f64,
    injury: f64,
    composure: f64,
    pub(super) morale: f64,
}

impl PlayerFactors {
    pub(super) fn new(state: &MatchState, player_id: Uuid) -> Self {
        let physical = (0.58 + 0.42 * state.player_energy(player_id))
            * state.player_settling_factor(player_id);
        let injury = if state.home().injured_player_ids().contains(&player_id)
            || state.away().injured_player_ids().contains(&player_id)
        {
            0.72
        } else {
            1.0
        };
        let morale = state.player_morale(player_id);
        let composure = if morale >= 100.0 {
            1.0
        } else {
            0.72 + 0.0028 * morale
        };
        Self {
            physical,
            injury,
            composure,
            morale,
        }
    }

    pub(super) fn apply(self, value: f64) -> f64 {
        value * self.physical * self.injury * self.composure
    }
}

pub(in crate::resolution) fn current_player_value(
    input: &MatchInput,
    state: &MatchState,
    player_id: Uuid,
    key: AttributeKey,
) -> EngineResult<f64> {
    let values = input
        .player_attributes(player_id)
        .ok_or_else(|| EngineError::InvalidInput("active player is missing from roster".into()))?;
    let value = values[key.index()]
        .ok_or_else(|| EngineError::InvalidInput(format!("player {player_id} lacks {key:?}")))?;
    Ok(PlayerFactors::new(state, player_id).apply(value))
}
