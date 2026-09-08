use crate::error::TacticsResult;
use crate::instructions::player::axes::{
    CreativeLicense, DepthDiscipline, EngagementBias, InvolvementPriority, PositioningBias,
    ReleaseTempo, TransitionUrgency,
};
use crate::instructions::player::marking::MarkingAssignment;
use crate::instructions::player::PlayerInstructions;
use crate::persistence::models::player_instruction_key_code::{
    parse_player_instruction_key, PlayerInstructionKey,
};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct TacticalLineupSlotInstructionRow {
    pub id: String,
    pub tactical_lineup_slot_id: String,
    pub phase: String,
    pub instruction_key: String,
    pub value: f64,
}

pub fn build_player_instructions(
    rows: &[TacticalLineupSlotInstructionRow],
    marking: Option<MarkingAssignment>,
) -> TacticsResult<PlayerInstructions> {
    let mut builder = PlayerInstructions::builder();
    if let Some(m) = marking {
        builder = builder.with_marking(m);
    }

    for row in rows {
        let key = parse_player_instruction_key(&row.instruction_key)?;
        match key {
            PlayerInstructionKey::PositioningBias => {
                builder = builder.with_positioning_bias(PositioningBias::new_clamped(row.value));
            }
            PlayerInstructionKey::InvolvementPriority => {
                builder = builder
                    .with_involvement_priority(InvolvementPriority::new_clamped(row.value));
            }
            PlayerInstructionKey::CreativeLicense => {
                builder = builder.with_creative_license(CreativeLicense::new_clamped(row.value));
            }
            PlayerInstructionKey::EngagementBias => {
                builder = builder.with_engagement_bias(EngagementBias::new_clamped(row.value));
            }
            PlayerInstructionKey::DepthDiscipline => {
                builder = builder.with_depth_discipline(DepthDiscipline::new_clamped(row.value));
            }
            PlayerInstructionKey::TransitionUrgency => {
                builder =
                    builder.with_transition_urgency(TransitionUrgency::new_clamped(row.value));
            }
            PlayerInstructionKey::ReleaseTempo => {
                builder = builder.with_release_tempo(ReleaseTempo::new_clamped(row.value));
            }
        }
    }

    Ok(builder.build())
}