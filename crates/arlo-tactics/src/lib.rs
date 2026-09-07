pub mod error;
pub mod instructions;
pub mod lineup;
pub mod persistence;
pub mod playcall;
pub mod series;

pub use error::{TacticsError, TacticsResult};
pub use instructions::{
    default_aggression, default_counter_attack_intensity, default_counter_press_intensity,
    default_defensive_line_height, default_directness, default_pressing_intensity, default_tempo,
    default_width, Aggression, ChannelDistribution, Compactness, CounterAttackIntensity,
    CounterPressIntensity, CreativeLicense, DefensiveLineHeight, DepthDiscipline, Directness,
    EngagementBias, EngagementLine, FlankBias, InPossessionInstructions,
    InPossessionPlayerInstructions, InvolvementPriority, MarkingAssignment, Mentality,
    OutOfPossessionInstructions, OutOfPossessionPlayerInstructions, PlayerInstructions,
    PlayerInstructionsBuilder, PositioningBias, PressingIntensity, ReleaseTempo, Structure,
    TeamInstructions, TeamInstructionsBuilder, Tempo, TransitionInstructions,
    TransitionPlayerInstructions, TransitionUrgency, Width,
};
pub use lineup::{
    derive_tier, is_role_eligible_for_position, max_concurrent_count, validate_tactical_lineup,
    ProficiencyTier, SlotAssignment, TacticalLineup, TacticalLineupBuilder,
};
pub use persistence::models::{
    build_player_instructions, instruction_key_to_code, marking_assignment_to_columns,
    parse_instruction_key, parse_marking_assignment, parse_player_instruction_key,
    parse_slot_role, parse_tactical_phase, player_instruction_key_to_code, slot_role_to_code,
    tactical_phase_to_code, InstructionKey, PlayerInstructionKey, TacticalInstructionValueRow,
    TacticalLineupRow, TacticalLineupSlotInstructionRow, TacticalLineupSlotRow, TacticalPhase,
    TeamTacticalProfileRow,
};
pub use persistence::repositories::{tactical_lineup, team_instructions};
pub use playcall::*;