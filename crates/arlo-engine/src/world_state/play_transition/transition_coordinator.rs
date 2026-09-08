use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::possession::TransitionResult;
use crate::resolution::AttributedDuelOutcome;
use crate::time::DurationComponentKind;
use crate::time::DurationLedger;
use crate::world_state::constants::IMMEDIATE_CONTROL_THRESHOLD_SECONDS;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::period_resolution::resolve_period_end;
use crate::world_state::play_transition::fatigue_applier::{
    apply_dead_ball_recovery, apply_duel_strain, apply_kinematic_movement_strain,
};
use crate::world_state::play_transition::possession_resolver::{
    build_detailed_play_outcome, classify_play_outcome, determine_countdown_reason,
    resolve_possession_transition,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::{
    apply_match_score, post_transition_score_reset, publish_scoring_impulse,
};
use crate::world_state::reorganization::derive_and_apply_reorganization;
use arlo_domain::ArtrineDecisionKind;
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
use uuid::Uuid;

pub struct TransitionPipeline<'a, 'b, S: EventSink> {
    publisher: EventPublisher<'a, S>,
    pass_phase: PassPhaseResult<'b>,
    execution_outcome: ArtrineExecutionOutcome,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    play_duels: Vec<AttributedDuelOutcome>,
    play_ledger: DurationLedger,
}

impl<'a, 'b, S: EventSink> TransitionPipeline<'a, 'b, S> {
    pub fn new(
        state: &'a mut MatchState,
        pass_phase: PassPhaseResult<'b>,
        _decision: ArtrineDecisionKind,
        execution_outcome: ArtrineExecutionOutcome,
        offense_team_id: Uuid,
        defense_team_id: Uuid,
        sink: &'a mut S,
    ) -> Self {
        let mut play_duels = Vec::with_capacity(1 + execution_outcome.duels.len());
        play_duels.push(pass_phase.pass_duel_outcome.clone());
        play_duels.extend(execution_outcome.duels.iter().cloned());

        let mut play_ledger = pass_phase.duration_ledger.clone();
        play_ledger.merge(execution_outcome.duration_ledger.clone());

        Self {
            publisher: EventPublisher::new(state, sink),
            pass_phase,
            execution_outcome,
            offense_team_id,
            defense_team_id,
            play_duels,
            play_ledger,
        }
    }

    fn apply_strains(&mut self) {
        self.publisher.emit_drives(
            self.pass_phase.artrine.id(),
            &self.execution_outcome.drive_row_indices,
        );

        if let Some(flight_info) = &self.execution_outcome.distribution_flight {
            self.publisher.emit_distribution_flight(flight_info);
        }

        apply_duel_strain(&mut self.publisher, &self.play_duels);
        self.publisher.emit_duel_events(
            &self.execution_outcome.duels,
            self.pass_phase.artrine.id(),
        );

        apply_kinematic_movement_strain(
            &mut self.publisher,
            &self.execution_outcome.kinematic_trajectories,
        );
    }

    fn process_scoring(&mut self) {
        apply_match_score(
            self.publisher.state_mut(),
            self.offense_team_id,
            &self.execution_outcome.scoring_decision,
        );
        self.publisher
            .emit_scoring_event(&self.execution_outcome.scoring_decision);
    }

    fn build_outcome(&self) -> DetailedPlayOutcome {
        let classification = classify_play_outcome(
            &self.pass_phase,
            &self.execution_outcome,
            self.defense_team_id,
        );

        let possession_control_seconds = if self.pass_phase.pass_completed {
            Some(self.play_ledger.total_live().value())
        } else {
            None
        };

        build_detailed_play_outcome(
            &self.pass_phase,
            &self.execution_outcome,
            &classification,
            self.play_duels.clone(),
            possession_control_seconds,
            self.offense_team_id,
            self.defense_team_id,
        )
    }

    fn process_impulse(&mut self, detailed_outcome: &DetailedPlayOutcome) -> TransitionResult {
        let offense_lineup = if self.offense_team_id == self.publisher.state().home_team_id() {
            self.publisher.state().home_lineup().clone()
        } else {
            self.publisher.state().away_lineup().clone()
        };
        let defense_lineup = if self.defense_team_id == self.publisher.state().home_team_id() {
            self.publisher.state().home_lineup().clone()
        } else {
            self.publisher.state().away_lineup().clone()
        };

        let offense_players = offense_lineup.players();
        let defense_players = defense_lineup.players();

        for duel in &self.play_duels {
            self.publisher
                .state_mut()
                .impulse_bus_mut()
                .publish_attributed_duel(duel, &offense_players, &defense_players);
        }

        let finisher_id = self
            .execution_outcome
            .receiver_id
            .unwrap_or(self.pass_phase.artrine.id());

        publish_scoring_impulse(
            self.publisher.state_mut(),
            &self.execution_outcome.scoring_decision,
            finisher_id,
            &defense_players,
            &offense_players,
        );

        let transition_result =
            resolve_possession_transition(self.publisher.state(), detailed_outcome);

        self.publisher
            .state_mut()
            .impulse_bus_mut()
            .publish_events(transition_result.impulse_events.clone());

        let current_period_seconds = self.publisher.state().clock().seconds_in_period();
        let shifts = self
            .publisher
            .state_mut()
            .process_impulse_bus(current_period_seconds);
        for (pid, shift, ev) in shifts {
            self.publisher.emit_impulse_shift(pid, &shift, &ev);
        }

        transition_result
    }

    fn resolve_turnovers(
        &mut self,
        detailed_outcome: &DetailedPlayOutcome,
        transition_result: &TransitionResult,
        previous_down: u32,
    ) {
        if let Some(new_offense) = detailed_outcome.turnover {
            self.publisher.emit_turnover_event(
                self.offense_team_id,
                new_offense,
                detailed_outcome.recovering_player_id,
                detailed_outcome.lost_by_player_id,
                !detailed_outcome.out_of_bounds,
                self.execution_outcome.end_position,
            );
        }

        if detailed_outcome.out_of_bounds {
            let was_immediate = detailed_outcome
                .possession_control_seconds
                .map(|s| s < IMMEDIATE_CONTROL_THRESHOLD_SECONDS)
                .unwrap_or(false);
            self.publisher.emit_out_of_bounds_event(
                self.offense_team_id,
                Some(self.pass_phase.artrine.id()),
                self.execution_outcome.end_position,
                was_immediate,
            );
        }

        let end_x_mirim = self.execution_outcome.end_position.raw().0 / MIRIM_TO_METERS;
        let new_down = transition_result.snapshot.down() as u32;
        let is_possession_change =
            transition_result.snapshot.role().offense() != self.offense_team_id;
        let is_first_down = (transition_result.snapshot.down() == 1 && previous_down > 1)
            || is_possession_change
            || transition_result.snapshot.down() == 1;

        self.publisher.emit_down_advanced_event(
            previous_down,
            new_down,
            self.execution_outcome.mirins_advanced,
            transition_result.snapshot.advanced_mirins(),
            is_first_down,
            end_x_mirim,
        );

        if transition_result.countdown_to_size_triggered {
            let reason = determine_countdown_reason(detailed_outcome, is_possession_change);
            self.publisher.emit_countdown_event(
                transition_result.snapshot.offense(),
                end_x_mirim,
                reason,
            );
        }
    }

    fn handle_dead_ball_and_clock(
        mut self,
        detailed_outcome: &DetailedPlayOutcome,
        transition_result: TransitionResult,
    ) {
        let is_possession_change =
            transition_result.snapshot.role().offense() != self.offense_team_id;

        let next_snapshot = post_transition_score_reset(
            self.publisher.state_mut(),
            &detailed_outcome.scoring_decision,
            is_possession_change,
            transition_result.snapshot,
        );

        let is_post_turnover = detailed_outcome.turnover.is_some();

        if transition_result.countdown_to_size_triggered {
            let next_scrimmage_x_mirim =
                next_snapshot.series_state().scrimmage_point().raw().0 / MIRIM_TO_METERS;
            let (reorg_duration, huddle_duration) = derive_and_apply_reorganization(
                &mut self.publisher,
                next_scrimmage_x_mirim,
                is_post_turnover,
                detailed_outcome.recovering_player_id,
            );
            self.play_ledger
                .record_dead_ball(DurationComponentKind::Reorganization, reorg_duration);
            self.play_ledger
                .record_dead_ball(DurationComponentKind::Huddle, huddle_duration);
        }

        let dead_ball_seconds = self.play_ledger.total_dead_ball().value();
        apply_dead_ball_recovery(&mut self.publisher, dead_ball_seconds);

        let live_seconds = self.play_ledger.total_live().value();
        if live_seconds > 0.0 {
            self.publisher
                .state_mut()
                .advance_impulse_dynamics(live_seconds);
        }
        if dead_ball_seconds > 0.0 {
            self.publisher
                .state_mut()
                .advance_impulse_dynamics(dead_ball_seconds);
        }

        let period_ended = self
            .publisher
            .state_mut()
            .clock_mut()
            .advance_seconds(live_seconds);
        self.publisher
            .state_mut()
            .real_time_mut()
            .add(self.play_ledger.total());
        *self.publisher.state_mut().possession_mut() = next_snapshot;

        if period_ended {
            resolve_period_end(self.publisher.state_mut());
        }
    }

    pub fn run(mut self) -> DetailedPlayOutcome {
        self.apply_strains();
        self.process_scoring();

        let detailed_outcome = self.build_outcome();
        let previous_down = self.publisher.state().possession().down() as u32;
        let transition_result = self.process_impulse(&detailed_outcome);

        self.resolve_turnovers(&detailed_outcome, &transition_result, previous_down);
        self.handle_dead_ball_and_clock(&detailed_outcome, transition_result);

        detailed_outcome
    }
}

pub fn apply_play_transition(
    state: &mut MatchState,
    pass_phase: PassPhaseResult<'_>,
    decision: ArtrineDecisionKind,
    execution_outcome: ArtrineExecutionOutcome,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    sink: &mut impl EventSink,
) -> DetailedPlayOutcome {
    TransitionPipeline::new(
        state,
        pass_phase,
        decision,
        execution_outcome,
        offense_team_id,
        defense_team_id,
        sink,
    )
    .run()
}