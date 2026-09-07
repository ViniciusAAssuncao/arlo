pub mod error;
pub mod instructions;
pub mod lineup;
pub mod persistence;

pub use error::{TacticsError, TacticsResult};
pub use instructions::{
    default_aggression, default_counter_attack_intensity, default_counter_press_intensity,
    default_defensive_line_height, default_directness, default_pressing_intensity, default_tempo,
    default_width, Aggression, ChannelDistribution, Compactness, CounterAttackIntensity,
    CounterPressIntensity, DefensiveLineHeight, Directness, EngagementLine, FlankBias,
    InPossessionInstructions, Mentality, OutOfPossessionInstructions, PressingIntensity,
    Structure, TeamInstructions, TeamInstructionsBuilder, Tempo, TransitionInstructions, Width,
};
pub use lineup::{
    derive_tier, is_role_eligible_for_position, max_concurrent_count, validate_tactical_lineup,
    ProficiencyTier, SlotAssignment, TacticalLineup, TacticalLineupBuilder,
};
pub use persistence::models::{
    instruction_key_to_code, parse_instruction_key, parse_slot_role, parse_tactical_phase,
    slot_role_to_code, tactical_phase_to_code, InstructionKey, TacticalInstructionValueRow,
    TacticalLineupRow, TacticalLineupSlotRow, TacticalPhase, TeamTacticalProfileRow,
};
pub use persistence::repositories::{tactical_lineup, team_instructions};