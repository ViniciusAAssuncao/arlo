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
    default_width, derive_pass_range_tier, nearest_block_notation, Aeriality, Aggression,
    ChannelDistribution, Compactness, CounterAttackIntensity, CounterPressIntensity,
    CreativeLicense, DefensiveLineHeight, DepthDiscipline, Directness, EngagementBias,
    EngagementLine, FlankBias, InPossessionInstructions, InPossessionPlayerInstructions,
    InvolvementPriority, MarkingAssignment, Mentality, OutOfPossessionInstructions,
    OutOfPossessionPlayerInstructions, PassRangeTier, PassingRange, Physicality,
    PlayerInstructions, PlayerInstructionsBuilder, PositioningBias, PressBlockShape,
    PressingIntensity, ReleaseTempo, ScoringPatience, Structure, TeamInstructions,
    TeamInstructionsBuilder, TeamTacticalProfile, Tempo, TransitionInstructions,
    TransitionPlayerInstructions, TransitionUrgency, Width,
};

pub use lineup::{
    derive_tier, is_role_eligible_for_position, max_concurrent_count, validate_tactical_lineup,
    ProficiencyTier, SlotAssignment, TacticalLineup, TacticalLineupBuilder,
};

pub use playcall::{
    derive_default_decision_emphasis, derive_distance_urgency, derive_down_pressure,
    derive_drive_scarcity, derive_scoring_proximity, rank_by_situational_fit, validate_play_call,
    DecisionEmphasis, HasSituationalProfile, MisdirectionLink, PlayCall, PlayCallBuilder,
    PlayCallCategory, ReadPriority, RouteAssignment, SituationalContext, SituationalProfile,
    SITUATIONAL_SIGMA_MAX, SITUATIONAL_SIGMA_MIN,
};

pub use series::{SeriesScript, SeriesScriptBuilder};

pub use persistence::models::{
    artrine_decision_kind_to_code, artro_placement_to_code, build_decision_emphasis,
    build_player_instructions, instruction_key_to_code, marking_assignment_to_columns,
    parse_artrine_decision_kind, parse_artro_placement, parse_instruction_key,
    parse_marking_assignment, parse_play_call_category, parse_player_instruction_key,
    parse_situational_parameter_key, parse_slot_role, parse_tactical_phase,
    play_call_category_to_code, player_instruction_key_to_code, situational_parameter_key_to_code,
    situational_profile_from_pairs, situational_profile_to_pairs, slot_role_to_code,
    tactical_phase_to_code, InstructionKey, PlayCallDecisionEmphasisRow,
    PlayCallMisdirectionLinkRow, PlayCallRouteAssignmentRow, PlayCallRow,
    PlayCallSituationalParameterRow, PlayerInstructionKey, SeriesScriptEntryRow, SeriesScriptRow,
    SituationalParameterKey, TacticalInstructionValueRow, TacticalLineupRow,
    TacticalLineupSlotInstructionRow, TacticalLineupSlotRow, TacticalPhase, TeamTacticalProfileRow,
    TeamTacticalProfileSituationalParameterRow,
};

pub use persistence::repositories::{play_call, series_script, tactical_lineup, team_instructions};
