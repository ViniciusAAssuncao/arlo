pub mod duel_resolution;
pub mod kinematics;
pub mod participants;

pub use crate::lineup_runtime::find_player_by_position;
pub use duel_resolution::resolve_pass_protection_duel;
pub use kinematics::{calculate_pass_kinematics, PassKinematicsResult};
pub use participants::{extract_participants, PhaseParticipants};

use crate::error::EngineResult;
use crate::possession::TouchActionType;
use crate::resolution::AttributedDuelOutcome;
use crate::time::DurationLedger;
use crate::world_state::match_state::MatchState;
use arlo_domain::{Player, Position as DomainPosition, SlotRole};
use arlo_events::EventSink;
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
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
    pub reception_point: VectorPosition,
    pub down_number: u32,
    pub scrimmage_point: VectorPosition,
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
    let participants = extract_participants(
        offense_players,
        offense_pos_index,
        offense_role_index,
        defense_players,
    )?;

    let down_number = state.possession().down() as u32;
    let scrimmage_point = state.possession().scrimmage_point();
    let scrimmage_x_mirim = scrimmage_point.raw().0 / MIRIM_TO_METERS;

    let passer_pos = state
        .spatial_map()
        .get_position(&participants.passer.id())
        .unwrap_or(scrimmage_point);
    let artrine_pos = state
        .spatial_map()
        .get_position(&participants.artrine.id())
        .unwrap_or(scrimmage_point);
    let pass_rusher_pos = state
        .spatial_map()
        .get_position(&participants.pass_rusher.id())
        .unwrap_or(scrimmage_point);

    let passer_zone = state.pitch().zone_at_position(passer_pos);
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
        passer_pos,
        artrine_pos,
        pass_rusher_pos,
        sink,
    );

    if kinematics.pass_completed {
        let artrine_zone = state.pitch().zone_at_position(kinematics.reception_point);
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
        reception_point: kinematics.reception_point,
        down_number,
        scrimmage_point,
        scrimmage_x_mirim,
        duration_ledger: kinematics.duration_ledger,
    })
}