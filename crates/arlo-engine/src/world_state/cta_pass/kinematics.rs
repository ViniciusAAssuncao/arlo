use crate::match_decision::event_translation::{create_envelope, translate_pass_completed};
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::participants::PhaseParticipants;
use crate::world_state::match_state::MatchState;
use arlo_events::EventSink;
use arlo_math::units::Duration;

pub struct PassKinematicsResult {
    pub pass_completed: bool,
    pub is_aerial: bool,
    pub reception_x_mirim: f64,
    pub reception_y_mirim: f64,
    pub duration_ledger: DurationLedger,
}

pub fn calculate_pass_kinematics(
    state: &mut MatchState,
    participants: &PhaseParticipants<'_>,
    pass_won: bool,
    sink: &mut impl EventSink,
) -> PassKinematicsResult {
    let pass_completed = pass_won;
    let is_aerial = false;
    let reception_x_mirim = state.possession().scrimmage_x_mirim();
    let reception_y_mirim = 42.5;
    let pass_distance_mirim = 2.5;

    let mut duration_ledger = DurationLedger::new();
    let pass_protection_duration = Duration::new(1.8);
    duration_ledger.record_live(
        DurationComponentKind::PassProtectionEngagement,
        pass_protection_duration,
    );

    if pass_completed {
        let flight_duration = Duration::new(0.8);
        duration_ledger.record_live(DurationComponentKind::InitialHandoffFlight, flight_duration);

        let pass_event = translate_pass_completed(
            participants.passer.id(),
            participants.artrine.id(),
            is_aerial,
            reception_x_mirim,
            reception_y_mirim,
            pass_distance_mirim,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, pass_event));
    }

    PassKinematicsResult {
        pass_completed,
        is_aerial,
        reception_x_mirim,
        reception_y_mirim,
        duration_ledger,
    }
}
