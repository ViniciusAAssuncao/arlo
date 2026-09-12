use crate::artrine::DistributionFlightInfo;
use crate::kick_foul::KickFoulPending;
use crate::manager_ai::event_translation::{
    translate_challenge_resolved, translate_play_call_selected, translate_substitution_made,
    translate_tactical_profile_activated, translate_time_call_used,
};
use crate::match_decision::event_translation::{
    create_envelope, translate_countdown_started, translate_distribution_completed,
    translate_down_advanced, translate_drive_recorded, translate_duel_resolved,
    translate_out_of_bounds, translate_physical_strain_recorded, translate_possession_time,
    translate_reception_resolved, translate_recovery_interval_processed,
    translate_scoring_decision, translate_turnover,
};
use crate::match_decision::scoring::ScoringDecision;
use crate::officiating::event_translation::{
    translate_availability_changed, translate_foul_raised,
};
use crate::officiating::foul::FoulResolution;
use crate::officiating::ReviewableCallKind;
use crate::psychology::event_translation::{
    translate_impulse_critical_reached, translate_impulse_shift_recorded,
};
use crate::psychology::systems::critical::ImpulseCriticalReached as EngineImpulseCritical;
use crate::psychology::systems::events::{ImpulseEvent, ImpulseShift};
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::world_state::match_state::availability::AvailabilityState;
use crate::world_state::match_state::MatchState;
use arlo_domain::sport_constants::ARTRO_ROW_SPACING_MIRIM;
use arlo_domain::{KickFoulDecisionKind, KickFoulScoringTier, PitchZone};
use arlo_events::{
    CountdownReason, EventArtroPlacement, EventSink, KickFoulAwarded, KickFoulDecisionMade,
    MatchClockInstant, MatchEvent, SubstitutionReason,
};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::PlayCallCategory;
use uuid::Uuid;

pub struct EventPublisher<'a, S: EventSink> {
    state: &'a mut MatchState,
    sink: &'a mut S,
}

impl<'a, S: EventSink> EventPublisher<'a, S> {
    pub fn new(state: &'a mut MatchState, sink: &'a mut S) -> Self {
        Self { state, sink }
    }

    pub fn state(&self) -> &MatchState {
        self.state
    }

    pub fn state_mut(&mut self) -> &mut MatchState {
        self.state
    }

    pub fn sink_mut(&mut self) -> &mut S {
        self.sink
    }

    pub fn publish<E: Into<MatchEvent>>(&mut self, event: E) {
        let seq = self.state.next_sequence();
        let clock_inst = self.state.clock().to_instant();
        self.sink.record(create_envelope(seq, clock_inst, event));
    }

    pub fn emit_drives(&mut self, artrine_id: Uuid, drive_row_indices: &[usize]) {
        for &row_index in drive_row_indices {
            self.state.increment_drives();
            let rx = (row_index as f64 + 1.0) * ARTRO_ROW_SPACING_MIRIM;
            let drive_event = translate_drive_recorded(
                artrine_id,
                row_index,
                EventArtroPlacement::Central,
                self.state.drives_in_current_series(),
                rx,
            );
            self.publish(drive_event);
        }
    }

    pub fn emit_distribution_flight(&mut self, flight_info: &DistributionFlightInfo) {
        let dist_event = translate_distribution_completed(flight_info);
        self.publish(dist_event);
    }

    pub fn emit_duel_events(&mut self, duels: &[AttributedDuelOutcome], default_receiver_id: Uuid) {
        for duel in duels {
            let duel_event = translate_duel_resolved(
                duel.outcome(),
                duel.attacker_ids().to_vec(),
                duel.defender_ids().to_vec(),
            );
            self.publish(duel_event);

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
                self.publish(reception_event);
            }
        }
    }

    pub fn emit_foul_raised(&mut self, resolution: &FoulResolution) {
        let event = translate_foul_raised(resolution);
        self.publish(event);
    }

    pub fn emit_scoring_event(&mut self, scoring_decision: &ScoringDecision) {
        if let Some(match_event) = translate_scoring_decision(scoring_decision) {
            self.publish(match_event);
        }
    }

    pub fn emit_turnover_event(
        &mut self,
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
        self.publish(turnover_event);
    }

    pub fn emit_out_of_bounds_event(
        &mut self,
        offense_team_id: Uuid,
        last_player_id: Option<Uuid>,
        point: VectorPosition,
        was_immediate: bool,
    ) {
        let oob_event =
            translate_out_of_bounds(offense_team_id, last_player_id, point, was_immediate);
        self.publish(oob_event);
    }

    pub fn emit_down_advanced_event(
        &mut self,
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
        self.publish(down_advanced_event);
    }

    pub fn emit_countdown_event(
        &mut self,
        offense_team_id: Uuid,
        end_x_mirim: f64,
        reason: CountdownReason,
    ) {
        let countdown_event = translate_countdown_started(offense_team_id, end_x_mirim, reason);
        self.publish(countdown_event);
    }

    pub fn emit_possession_time_recorded(&mut self, team_id: Uuid, duration: f64) {
        let event = translate_possession_time(team_id, duration);
        self.publish(event);
    }

    pub fn emit_physical_strain(
        &mut self,
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
        self.publish(strain_ev);
    }

    pub fn emit_recovery_processed(
        &mut self,
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
        self.publish(rec_ev);
    }

    pub fn emit_impulse_shift(
        &mut self,
        player_id: Uuid,
        shift: &ImpulseShift,
        event: &ImpulseEvent,
    ) {
        let shift_event = translate_impulse_shift_recorded(player_id, shift, event);
        self.publish(shift_event);
    }

    pub fn emit_impulse_critical(&mut self, critical: &EngineImpulseCritical) {
        let critical_event = translate_impulse_critical_reached(critical);
        self.publish(critical_event);
    }

    pub fn emit_substitution_made(
        &mut self,
        team_id: Uuid,
        player_out: Uuid,
        player_in: Uuid,
        match_clock: MatchClockInstant,
        reason: SubstitutionReason,
    ) {
        let event =
            translate_substitution_made(team_id, player_out, player_in, match_clock, reason);
        self.publish(event);
    }

    pub fn emit_time_call_used(&mut self, team_id: Uuid, remaining_time_calls_after: u32) {
        let event = translate_time_call_used(team_id, remaining_time_calls_after);
        self.publish(event);
    }

    pub fn emit_challenge_resolved(
        &mut self,
        team_id: Uuid,
        call_kind: ReviewableCallKind,
        success: bool,
        remaining_challenges_after: u32,
    ) {
        let event =
            translate_challenge_resolved(team_id, call_kind, success, remaining_challenges_after);
        self.publish(event);
    }

    pub fn emit_tactical_profile_activated(
        &mut self,
        team_id: Uuid,
        profile_id: Uuid,
        profile_name: impl Into<String>,
    ) {
        let event = translate_tactical_profile_activated(team_id, profile_id, profile_name);
        self.publish(event);
    }

    pub fn emit_play_call_selected(
        &mut self,
        team_id: Uuid,
        play_call_id: Uuid,
        play_call_name: impl Into<String>,
        category: PlayCallCategory,
    ) {
        let event = translate_play_call_selected(team_id, play_call_id, play_call_name, category);
        self.publish(event);
    }

    pub fn emit_player_availability_changed(
        &mut self,
        player_id: Uuid,
        team_id: Uuid,
        previous: AvailabilityState,
        new: AvailabilityState,
    ) {
        let event = translate_availability_changed(player_id, team_id, previous, new);
        self.publish(event);
    }

    pub fn emit_kick_foul_awarded(
        &mut self,
        pending: &KickFoulPending,
        offending_team_id: Uuid,
    ) {
        let event = KickFoulAwarded::new(
            pending.awarded_team_id(),
            offending_team_id,
            pending.spot(),
            pending.scoring_tier(),
        );
        self.publish(event);
    }

    pub fn emit_kick_foul_decision_made(
        &mut self,
        taker_id: Uuid,
        decision: KickFoulDecisionKind,
        scoring_tier: KickFoulScoringTier,
    ) {
        let event = KickFoulDecisionMade::new(taker_id, decision, scoring_tier);
        self.publish(event);
    }
}