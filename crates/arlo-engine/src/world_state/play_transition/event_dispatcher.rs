use crate::artrine::DistributionFlightInfo;
use crate::match_decision::event_translation::{
    create_envelope, translate_countdown_started, translate_distribution_completed,
    translate_down_advanced, translate_drive_recorded, translate_duel_resolved,
    translate_out_of_bounds, translate_physical_strain_recorded,
    translate_reception_resolved, translate_recovery_interval_processed,
    translate_scoring_decision, translate_turnover,
};
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::world_state::match_state::MatchState;
use arlo_domain::sport_constants::ARTRO_ROW_SPACING_MIRIM;
use arlo_domain::PitchZone;
use arlo_events::{CountdownReason, EventArtroPlacement, EventSink};
use arlo_math::units::Position as VectorPosition;
use uuid::Uuid;

pub fn emit_drives(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    artrine_id: Uuid,
    drive_row_indices: &[usize],
) {
    for &row_index in drive_row_indices {
        state.increment_drives();
        let rx = (row_index as f64 + 1.0) * ARTRO_ROW_SPACING_MIRIM;
        let drive_event = translate_drive_recorded(
            artrine_id,
            row_index,
            EventArtroPlacement::Central,
            state.drives_in_current_series(),
            rx,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, drive_event));
    }
}

pub fn emit_distribution_flight(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    flight_info: &DistributionFlightInfo,
) {
    let dist_event = translate_distribution_completed(flight_info);
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, dist_event));
}

pub fn emit_duel_events(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    duels: &[AttributedDuelOutcome],
    default_receiver_id: Uuid,
) {
    for duel in duels {
        let duel_event = translate_duel_resolved(
            duel.outcome(),
            duel.attacker_ids().to_vec(),
            duel.defender_ids().to_vec(),
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, duel_event));

        if matches!(
            duel.outcome().kind(),
            DuelKind::RouteContest | DuelKind::AerialDuel
        ) {
            let receiver_id = duel
                .attacker_ids()
                .first()
                .copied()
                .unwrap_or(default_receiver_id);
            let reception_event = translate_reception_resolved(
                receiver_id,
                default_receiver_id,
                duel.outcome().attacker_won(),
                duel.outcome().kind() == DuelKind::AerialDuel,
            );
            let seq = state.next_sequence();
            let clock_inst = state.clock().to_instant();
            sink.record(create_envelope(seq, clock_inst, reception_event));
        }
    }
}

pub fn emit_scoring_event(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    scoring_decision: &ScoringDecision,
) {
    if let Some(match_event) = translate_scoring_decision(scoring_decision) {
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, match_event));
    }
}

pub fn emit_turnover_event(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    offense_team_id: Uuid,
    new_offense: Uuid,
    recovering_player_id: Option<Uuid>,
    lost_by_player_id: Option<Uuid>,
    in_live_play: bool,
    point: VectorPosition,
) {
    let turnover_event = translate_turnover(
        offense_team_id,
        new_offense,
        recovering_player_id,
        lost_by_player_id,
        in_live_play,
        point,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, turnover_event));
}

pub fn emit_out_of_bounds_event(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    offense_team_id: Uuid,
    last_player_id: Option<Uuid>,
    point: VectorPosition,
    was_immediate: bool,
) {
    let oob_event = translate_out_of_bounds(
        offense_team_id,
        last_player_id,
        point,
        was_immediate,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, oob_event));
}

pub fn emit_down_advanced_event(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    previous_down: u32,
    new_down: u32,
    mirins_advanced_this_down: f64,
    total_advanced_in_series: f64,
    is_first_down: bool,
    end_x_mirim: f64,
) {
    let down_advanced_event = translate_down_advanced(
        previous_down,
        new_down,
        mirins_advanced_this_down,
        total_advanced_in_series,
        is_first_down,
        end_x_mirim,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, down_advanced_event));
}

pub fn emit_countdown_event(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    offense_team_id: Uuid,
    end_x_mirim: f64,
    reason: CountdownReason,
) {
    let countdown_event = translate_countdown_started(
        offense_team_id,
        end_x_mirim,
        reason,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, countdown_event));
}

pub fn emit_physical_strain(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    player_id: Uuid,
    energy: f64,
    w_bal: f64,
    distance_delta_mirim: f64,
    high_intensity_distance_mirim: f64,
    low_intensity_distance_mirim: f64,
    metabolic_energy_joules: f64,
    zone: PitchZone,
    peak_speed_meters_per_sec: f64,
) {
    let strain_ev = translate_physical_strain_recorded(
        player_id,
        energy,
        w_bal,
        distance_delta_mirim,
        high_intensity_distance_mirim,
        low_intensity_distance_mirim,
        metabolic_energy_joules,
        zone,
        peak_speed_meters_per_sec,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, strain_ev));
}

pub fn emit_recovery_processed(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    player_id: Uuid,
    recovery_amount: f64,
    duration_seconds: f64,
    new_w_bal: f64,
) {
    let rec_ev = translate_recovery_interval_processed(
        player_id,
        recovery_amount,
        duration_seconds,
        new_w_bal,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, rec_ev));
}