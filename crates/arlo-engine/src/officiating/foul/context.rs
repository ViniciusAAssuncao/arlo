use crate::attributes::{PlayerAttributeTable, RefereeAttributeTable};
use crate::physical::PhysicalState;
use crate::resolution::{DuelContext, DuelOutcome};
use crate::world_state::context_analyzer::GameStatePressure;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct FoulEvaluationContext {
    pub carrier_id: Uuid,
    pub carrier_team_id: Uuid,
    pub defender_id: Uuid,
    pub defender_team_id: Uuid,
    pub carrier_table: PlayerAttributeTable,
    pub defender_table: PlayerAttributeTable,
    pub carrier_physical_state: PhysicalState,
    pub defender_physical_state: PhysicalState,
    pub head_referee_table: RefereeAttributeTable,
    pub peace_referee_table: RefereeAttributeTable,
    pub duel_outcome: DuelOutcome,
    pub duel_context: DuelContext,
    pub contact_severity: f64,
    pub game_state_pressure: GameStatePressure,
}

impl FoulEvaluationContext {
    pub fn new(
        carrier_id: Uuid,
        carrier_team_id: Uuid,
        defender_id: Uuid,
        defender_team_id: Uuid,
        carrier_table: PlayerAttributeTable,
        defender_table: PlayerAttributeTable,
        carrier_physical_state: PhysicalState,
        defender_physical_state: PhysicalState,
        head_referee_table: RefereeAttributeTable,
        peace_referee_table: RefereeAttributeTable,
        duel_outcome: DuelOutcome,
        duel_context: DuelContext,
        contact_severity: f64,
        game_state_pressure: GameStatePressure,
    ) -> Self {
        Self {
            carrier_id,
            carrier_team_id,
            defender_id,
            defender_team_id,
            carrier_table,
            defender_table,
            carrier_physical_state,
            defender_physical_state,
            head_referee_table,
            peace_referee_table,
            duel_outcome,
            duel_context,
            contact_severity,
            game_state_pressure,
        }
    }
}
