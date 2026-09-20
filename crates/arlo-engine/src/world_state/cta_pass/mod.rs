pub mod duel_resolution;
pub mod kinematics;
pub mod participants;

pub use crate::lineup_runtime::find_player_by_position;
pub use duel_resolution::resolve_pass_protection_duel;
pub use kinematics::{calculate_pass_kinematics, PassKinematicsResult};
pub use participants::{extract_participants, PhaseParticipants};

use crate::error::EngineResult;
use crate::possession::{locate_zone, TouchActionType};
use crate::resolution::AttributedDuelOutcome;
use crate::time::DurationLedger;
use crate::world_state::match_state::MatchState;
use arlo_domain::sport_constants::AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM;
use arlo_domain::{Player, Position as DomainPosition, SlotRole};
use arlo_events::EventSink;
use std::collections::HashMap;
use uuid::Uuid;

pub struct PassPhaseResult<'a> {
    pub passer: &'a Player,
    pub artrine: &'a Player,
    pub pass_rusher: &'a Player,
    pub goalguard: &'a Player,
    pub pass_duel_outcome: AttributedDuelOutcome,
    pub pass_completed: bool,
    pub is_aerial: bool,
    pub reception_x_mirim: f64,
    pub reception_y_mirim: f64,
    pub down_number: u32,
    pub scrimmage_x_mirim: f64,
    pub duration_ledger: DurationLedger,
}

pub fn resolve_pass_phase<'a>(
    state: &mut MatchState,
    offense_players: &[&'a Player],
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    defense_players: &[&'a Player],
    is_home_offense: bool,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    sink: &mut impl EventSink,
) -> EngineResult<PassPhaseResult<'a>> {
    let defense_pos_index = state.defensive_position_index_for_team_arc(defense_team_id);
    let participants = extract_participants(
        offense_players,
        offense_pos_index,
        offense_role_index,
        defense_players,
        &defense_pos_index,
    )?;

    let down_number = state.possession().down() as u32;
    let scrimmage_x_mirim = state.possession().scrimmage_x_mirim();

    let pitch_length_mirim = state.pitch().length_mirim();
    let norm_prox = if is_home_offense {
        (scrimmage_x_mirim / pitch_length_mirim.max(1.0)).clamp(0.0, 1.0)
    } else {
        ((pitch_length_mirim - scrimmage_x_mirim) / pitch_length_mirim.max(1.0)).clamp(0.0, 1.0)
    };
    let passer_zone = locate_zone(
        norm_prox,
        pitch_length_mirim,
        AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM,
    );

    let current_time = state.clock().seconds_in_period();
    state.possession_mut().live_sequence_mut().record_touch(
        participants.passer.id(),
        TouchActionType::InitialHandoff,
        passer_zone,
        current_time,
    );

    let pass_duel_outcome = resolve_pass_protection_duel(
        state,
        &participants,
        is_home_offense,
        offense_team_id,
        defense_team_id,
        down_number,
        scrimmage_x_mirim,
        sink,
    );

    let pass_won = pass_duel_outcome.outcome().attacker_won();
    let kinematics = calculate_pass_kinematics(
        state,
        &participants,
        pass_won,
        sink,
    );

    if kinematics.pass_completed {
        let artrine_zone = passer_zone;
        state.possession_mut().live_sequence_mut().record_touch(
            participants.artrine.id(),
            TouchActionType::Reception,
            artrine_zone,
            current_time + kinematics.duration_ledger.total_live().value(),
        );
    }

    Ok(PassPhaseResult {
        passer: participants.passer,
        artrine: participants.artrine,
        pass_rusher: participants.pass_rusher,
        goalguard: participants.goalguard,
        pass_duel_outcome,
        pass_completed: kinematics.pass_completed,
        is_aerial: kinematics.is_aerial,
        reception_x_mirim: kinematics.reception_x_mirim,
        reception_y_mirim: kinematics.reception_y_mirim,
        down_number,
        scrimmage_x_mirim,
        duration_ledger: kinematics.duration_ledger,
    })
}