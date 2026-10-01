
pub mod discipline;
pub mod duels;
pub mod passes;
pub mod possession;
pub mod scoring;
pub mod tactics;

use crate::context::MatchAnalysisContext;
use crate::performance::observation::{PerformanceObservation, PossessionPhase};
use arlo_events::{MatchClockInstant, MatchEvent, MatchEventEnvelope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EventPerformanceTranslator {
    initial_context: Option<MatchAnalysisContext>,
    context: Option<MatchAnalysisContext>,
    current_offense_team_id: Option<Uuid>,
    current_down_number: u32,
    scrimmage_x_mirim: f64,
    current_drives_in_series: u32,
    #[serde(default)]
    pending_missed_shot_recovery: bool,
    last_clock: MatchClockInstant,
}

impl EventPerformanceTranslator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_context(context: MatchAnalysisContext) -> Self {
        Self {
            initial_context: Some(context.clone()),
            context: Some(context),
            ..Default::default()
        }
    }

    pub fn set_context(&mut self, context: MatchAnalysisContext) {
        self.initial_context = Some(context.clone());
        self.context = Some(context);
    }

    pub fn context(&self) -> Option<&MatchAnalysisContext> {
        self.context.as_ref()
    }

    pub fn initial_context(&self) -> Option<&MatchAnalysisContext> {
        self.initial_context.as_ref()
    }

    pub fn current_offense_team_id(&self) -> Option<Uuid> {
        self.current_offense_team_id
    }

    pub fn current_down_number(&self) -> u32 {
        self.current_down_number
    }

    pub fn scrimmage_x_mirim(&self) -> f64 {
        self.scrimmage_x_mirim
    }

    pub fn current_drives_in_series(&self) -> u32 {
        self.current_drives_in_series
    }

    pub fn resolve_phase_for_team(&self, team_id: Uuid) -> PossessionPhase {
        match self.current_offense_team_id {
            Some(off) if off == team_id => PossessionPhase::Offense,
            Some(_) => PossessionPhase::Defense,
            None => PossessionPhase::Neutral,
        }
    }

    pub fn translate_envelope(&mut self, envelope: &MatchEventEnvelope) -> Vec<PerformanceObservation> {
        let clock = envelope.clock();
        self.last_clock = clock;
        self.translate_event(envelope.event(), clock)
    }

    pub fn translate_event(
        &mut self,
        event: &MatchEvent,
        clock: MatchClockInstant,
    ) -> Vec<PerformanceObservation> {
        self.update_internal_state(event);
        let follows_missed_shot = self.pending_missed_shot_recovery;

        let observations = match event {
            MatchEvent::DuelResolved(e) => {
                duels::translate_duel(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::PassCompleted(e) => {
                passes::translate_pass_completed(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::ReceptionResolved(e) => {
                passes::translate_reception_resolved(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::DistributionCompleted(e) => {
                passes::translate_distribution_completed(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::CarryResolved(e) => {
                passes::translate_carry_resolved(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::ArtrineDecisionMade(e) => {
                tactics::translate_artrine_decision(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::DriveRecorded(e) => {
                tactics::translate_drive_recorded(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::PasserContactResolved(e) => {
                tactics::translate_passer_contact(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::GoalPoint(e) => {
                scoring::translate_goal_point(e, clock, self.context.as_ref())
            }
            MatchEvent::FieldPoint(e) => {
                scoring::translate_field_point(e, clock, self.context.as_ref())
            }
            MatchEvent::FieldGoal(e) => {
                scoring::translate_field_goal(e, clock, self.context.as_ref())
            }
            MatchEvent::ScoringAttemptMissed(e) => {
                scoring::translate_scoring_missed(e, clock, self.context.as_ref())
            }
            MatchEvent::Turnover(e) => {
                possession::translate_turnover(
                    e,
                    clock,
                    self.context.as_ref(),
                    follows_missed_shot,
                )
            }
            MatchEvent::GoalguardRecoveryResolved(e) => {
                possession::translate_goalguard_recovery(e, clock, self.context.as_ref())
            }
            MatchEvent::FoulRaised(e) => {
                discipline::translate_foul_raised(e, clock, self.context.as_ref())
            }
            MatchEvent::RefereeDecisionResolved(e) => {
                discipline::translate_referee_decision(e, clock, self.context.as_ref())
            }
            MatchEvent::PunishmentApplied(e) => {
                discipline::translate_punishment_applied(e, clock, self.context.as_ref())
            }
            MatchEvent::KickFoulAwarded(e) => {
                discipline::translate_kick_foul_awarded(e, clock, self.context.as_ref())
            }
            MatchEvent::KickFoulDecisionMade(e) => {
                discipline::translate_kick_foul_decision(e, clock, self.current_offense_team_id, self.context.as_ref())
            }
            MatchEvent::SubstitutionMade(e) => {
                if let Some(ctx) = &mut self.context {
                    let _ = ctx.handle_substitution_event(e);
                }
                Vec::new()
            }
            MatchEvent::PlayInvalidated(_) => Vec::new(),
            _ => Vec::new(),
        };

        if matches!(event, MatchEvent::Turnover(_)) {
            self.pending_missed_shot_recovery = false;
        }

        observations
    }

    pub fn translate_envelopes<'a>(
        &mut self,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) -> Vec<PerformanceObservation> {
        let mut all = Vec::new();
        for env in envelopes {
            all.extend(self.translate_envelope(env));
        }
        all
    }

    pub fn reset(&mut self) {
        self.current_offense_team_id = None;
        self.current_down_number = 1;
        self.scrimmage_x_mirim = 0.0;
        self.current_drives_in_series = 0;
        self.pending_missed_shot_recovery = false;
        self.last_clock = MatchClockInstant::zero();
        self.context = self.initial_context.clone();
    }

    fn update_internal_state(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::CallToActionStarted(e) => {
                self.pending_missed_shot_recovery = false;
                self.current_offense_team_id = Some(e.offense_team_id());
                self.current_down_number = e.down_number();
                self.scrimmage_x_mirim = e.scrimmage_x_mirim();
            }
            MatchEvent::DownAdvanced(e) => {
                self.current_down_number = e.new_down();
                self.scrimmage_x_mirim = e.scrimmage_x_mirim();
                if e.first_down_achieved() {
                    self.current_drives_in_series = 0;
                }
            }
            MatchEvent::DriveRecorded(e) => {
                self.current_drives_in_series = e.drives_in_series();
            }
            MatchEvent::ScoringAttemptMissed(_) => {
                self.pending_missed_shot_recovery = true;
            }
            MatchEvent::Turnover(e) => {
                self.current_offense_team_id = Some(e.new_offense());
                self.current_down_number = 1;
                self.current_drives_in_series = 0;
            }
            MatchEvent::CountdownToSizeStarted(e) => {
                self.current_offense_team_id = Some(e.offense_team_id());
                self.current_down_number = 1;
                self.current_drives_in_series = 0;
            }
            MatchEvent::GoalPoint(_) | MatchEvent::FieldPoint(_) | MatchEvent::FieldGoal(_) => {
                self.pending_missed_shot_recovery = false;
                self.current_drives_in_series = 0;
                self.current_down_number = 1;
            }
            _ => {}
        }
    }
}
