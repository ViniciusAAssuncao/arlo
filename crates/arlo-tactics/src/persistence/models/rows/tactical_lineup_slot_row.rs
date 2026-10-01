use crate::error::TacticsResult;
use crate::instructions::player::PlayerInstructions;
use crate::lineup::SlotAssignment;
use crate::persistence::models::slot_role_code::parse_slot_role;
use arlo_domain::Position;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct TacticalLineupSlotRow {
    pub id: String,
    pub tactical_lineup_id: String,
    pub slot_index: i32,
    pub player_id: String,
    pub slot_role: String,
    pub marking_scheme: Option<String>,
    pub marking_target_position: Option<String>,
}

impl TacticalLineupSlotRow {
    pub fn to_domain(
        &self,
        position: Position,
        player_instructions: PlayerInstructions,
    ) -> TacticsResult<SlotAssignment> {
        let player_id = Uuid::parse_str(&self.player_id)?;
        let slot_role = parse_slot_role(&self.slot_role)?;
        Ok(SlotAssignment::new(
            self.slot_index as usize,
            position,
            player_id,
            slot_role,
            player_instructions,
        ))
    }
}
