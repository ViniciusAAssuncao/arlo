use crate::match_decision::event_translation::{create_envelope, translate_pass_completed};
use crate::physical::systems::degradation::calculate_effective_player_speed_from_table;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::spatial::ball_kinematics::{ball_flight_duration, calculate_pass_speed_from_table};
use crate::spatial::proximity::calculate_distance_mirim;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::participants::PhaseParticipants;
use crate::world_state::match_state::MatchState;
use arlo_events::EventSink;
use arlo_math::units::Position as VectorPosition;

pub struct PassKinematicsResult {
    pub pass_completed: bool,
    pub is_aerial: bool,
    pub reception_point: VectorPosition,
    pub duration_ledger: DurationLedger,
}

pub fn calculate_pass_kinematics(
    state: &mut MatchState,
    participants: &PhaseParticipants<'_>,
    pass_won: bool,
    passer_pos: VectorPosition,
    artrine_pos: VectorPosition,
    pass_rusher_pos: VectorPosition,
    sink: &mut impl EventSink,
) -> PassKinematicsResult {
    let passer_state = state.fatigue_for(&participants.passer.id());
    let pass_rusher_state = state.fatigue_for(&participants.pass_rusher.id());
    let passer_table = state.attribute_table_for(&participants.passer.id());
    let pass_rusher_table = state.attribute_table_for(&participants.pass_rusher.id());

    let passer_speed = calculate_effective_player_speed_from_table(
        participants.passer,
        passer_table,
        &passer_state,
    );
    let pass_rusher_speed = calculate_effective_player_speed_from_table(
        participants.pass_rusher,
        pass_rusher_table,
        &pass_rusher_state,
    );
    let pass_protection_duration =
        derive_duel_duration(passer_pos, passer_speed, pass_rusher_pos, pass_rusher_speed);

    let pass_completed = pass_won;
    let is_aerial = false;
    let reception_point = artrine_pos;
    let pass_distance_mirim = calculate_distance_mirim(passer_pos, artrine_pos);

    let mut duration_ledger = DurationLedger::new();
    duration_ledger.record_live(
        DurationComponentKind::PassProtectionEngagement,
        pass_protection_duration,
    );

    if pass_completed {
        let pass_speed =
            calculate_pass_speed_from_table(participants.passer, passer_table, &passer_state);
        let flight_duration = ball_flight_duration(pass_distance_mirim, pass_speed);
        duration_ledger.record_live(DurationComponentKind::InitialHandoffFlight, flight_duration);

        let pass_event = translate_pass_completed(
            participants.passer.id(),
            participants.artrine.id(),
            is_aerial,
            reception_point,
            pass_distance_mirim,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, pass_event));
    }

    PassKinematicsResult {
        pass_completed,
        is_aerial,
        reception_point,
        duration_ledger,
    }
}
