use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::resolution::AttributedDuelOutcome;
use crate::time::DurationLedger;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::dead_ball_clock::handle_dead_ball_and_clock;
use crate::world_state::play_transition::fatigue_applier::{
    apply_duel_strain, apply_kinematic_movement_strain,
};
use crate::world_state::play_transition::impulse_coordinator::coordinate_play_impulse;
use crate::world_state::play_transition::possession_resolver::{
    build_detailed_play_outcome, classify_play_outcome,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::{
    apply_match_score, enrich_scoring_decision_assister,
};
use crate::world_state::play_transition::turnover_and_down_events::resolve_turnover_and_down_events;
use arlo_domain::ArtrineDecisionKind;
use arlo_events::EventSink;
use uuid::Uuid;

pub struct TransitionPipeline<'a, 'b, S: EventSink> {
    publisher: EventPublisher<'a, S>,
    pass_phase: PassPhaseResult<'b>,
    execution_outcome: ArtrineExecutionOutcome,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    active_play_call_id: Option<Uuid>,
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
        active_play_call_id: Option<Uuid>,
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
            active_play_call_id,
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
        self.publisher
            .emit_duel_events(&self.execution_outcome.duels, self.pass_phase.artrine.id());

        apply_kinematic_movement_strain(
            &mut self.publisher,
            &self.execution_outcome.kinematic_trajectories,
        );
    }

    fn process_scoring(&mut self) {
        enrich_scoring_decision_assister(
            &mut self.execution_outcome.scoring_decision,
            self.publisher.state().possession().live_sequence(),
        );
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

    pub fn run(mut self) -> DetailedPlayOutcome {
        self.apply_strains();
        self.process_scoring();

        let live_seconds = self.play_ledger.total_live().value();
        self.publisher
            .emit_possession_time_recorded(self.offense_team_id, live_seconds);

        let detailed_outcome = self.build_outcome();
        let previous_down = self.publisher.state().possession().down() as u32;

        let transition_result = coordinate_play_impulse(
            &mut self.publisher,
            &self.play_duels,
            self.pass_phase.artrine.id(),
            &self.execution_outcome,
            &detailed_outcome,
        );

        resolve_turnover_and_down_events(
            &mut self.publisher,
            &detailed_outcome,
            &transition_result,
            &self.play_duels,
            previous_down,
        );

        let failed = detailed_outcome.turnover.is_some()
            || !detailed_outcome.pass_completed
            || detailed_outcome.mirins_advanced <= 0.0;
        self.publisher
            .state_mut()
            .set_last_play_outcome_summary(self.active_play_call_id.map(|id| (id, failed)));

        handle_dead_ball_and_clock(
            &mut self.publisher,
            &detailed_outcome,
            transition_result,
            &mut self.play_ledger,
        );

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
    active_play_call_id: Option<Uuid>,
    sink: &mut impl EventSink,
) -> DetailedPlayOutcome {
    TransitionPipeline::new(
        state,
        pass_phase,
        decision,
        execution_outcome,
        offense_team_id,
        defense_team_id,
        active_play_call_id,
        sink,
    )
    .run()
}
