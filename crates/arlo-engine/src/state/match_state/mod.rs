mod live;
mod officiating;
mod period;
mod scoring;

use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{ClockState, MatchPhase, PossessionState, SeriesState, TeamState};
use arlo_events::{MatchClockInstant, MatchEvent, MatchEventEnvelope};
use arlo_events::RefereeDecisionResolved;
use arlo_events::PlayerAvailabilityChanged;
use arlo_domain::PunishmentKind;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
struct SuspendedRestart {
    team_id: Uuid,
    series: SeriesState,
    position_mirim: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PendingCallOutcome {
    pub prior_down: u8,
    pub gain_mirim: f64,
    pub total_advance_mirim: f64,
    pub first_down: bool,
}

#[derive(Debug, Clone)]
pub struct MatchState {
    match_id: Uuid,
    phase: MatchPhase,
    clock: ClockState,
    home: TeamState,
    away: TeamState,
    possession: PossessionState,
    series: SeriesState,
    suspended_restart: Option<SuspendedRestart>,
    pending_call_outcome: Option<PendingCallOutcome>,
    pending_referee_decisions: Vec<RefereeDecisionResolved>,
    pending_availability_events: Vec<PlayerAvailabilityChanged>,
    deferred_series_penalties: Vec<(Uuid, PunishmentKind, i32)>,
    pitch_length_mirim: f64,
    next_event_sequence: u64,
    rng: ChaCha8Rng,
}

impl MatchState {
    pub fn new(input: &MatchInput) -> Self {
        let home_id = input.home().team_id();
        let pitch_length_mirim = input.pitch().length_mirim();
        let midfield = pitch_length_mirim / 2.0;
        Self {
            match_id: input.match_id(),
            phase: MatchPhase::Ready,
            clock: ClockState::default(),
            home: TeamState::from_input(input.home()),
            away: TeamState::from_input(input.away()),
            possession: PossessionState::new(home_id, midfield, pitch_length_mirim)
                .expect("validated pitch has a midfield"),
            series: SeriesState::new(home_id, midfield, pitch_length_mirim)
                .expect("validated pitch has a midfield"),
            suspended_restart: None,
            pending_call_outcome: None,
            pending_referee_decisions: Vec::new(),
            pending_availability_events: Vec::new(),
            deferred_series_penalties: Vec::new(),
            pitch_length_mirim,
            next_event_sequence: 1,
            rng: ChaCha8Rng::seed_from_u64(input.seed()),
        }
    }

    pub fn match_id(&self) -> Uuid {
        self.match_id
    }
    pub fn phase(&self) -> MatchPhase {
        self.phase
    }
    pub fn clock(&self) -> ClockState {
        self.clock
    }
    pub fn home(&self) -> &TeamState {
        &self.home
    }
    pub fn away(&self) -> &TeamState {
        &self.away
    }
    pub fn possession(&self) -> PossessionState {
        self.possession
    }
    pub fn possessor_team_id(&self) -> Uuid {
        self.possession.possessor_team_id()
    }
    pub fn next_call_team_id(&self) -> Uuid {
        self.possession.next_call_team_id()
    }

    pub fn carrier_id(&self) -> Option<Uuid> {
        self.possession.carrier_id()
    }
    pub fn last_passer_id(&self) -> Option<Uuid> {
        self.possession.last_passer_id()
    }
    pub fn series(&self) -> SeriesState {
        self.series
    }
    pub fn last_valid_possession_mirim(&self) -> f64 {
        self.possession.ball_position_mirim()
    }
    pub fn next_event_sequence(&self) -> u64 {
        self.next_event_sequence
    }

    pub(crate) fn rng_mut(&mut self) -> &mut ChaCha8Rng {
        &mut self.rng
    }

    pub(crate) fn set_carrier(&mut self, player_id: Uuid) -> EngineResult<()> {
        if self.phase != MatchPhase::Live
            || !self
                .team(self.possessor_team_id())?
                .active_player_ids()
                .contains(&player_id)
        {
            return Err(EngineError::InvalidTransition(
                "ball carrier must be active for the possessing team".into(),
            ));
        }
        self.possession = self.possession.with_carrier(player_id);
        Ok(())
    }

    pub(crate) fn complete_pass(&mut self, passer_id: Uuid, receiver_id: Uuid) -> EngineResult<()> {
        let active = self.team(self.possessor_team_id())?.active_player_ids();
        if self.phase != MatchPhase::Live
            || passer_id == receiver_id
            || self.carrier_id().is_some_and(|carrier_id| carrier_id != passer_id)
            || !active.contains(&passer_id)
            || !active.contains(&receiver_id)
        {
            return Err(EngineError::InvalidTransition(
                "completed pass requires active teammates and the current carrier".into(),
            ));
        }
        self.possession = self.possession.with_completed_pass(passer_id, receiver_id);
        Ok(())
    }

    pub(crate) fn pending_call_outcome(&self) -> Option<PendingCallOutcome> {
        self.pending_call_outcome
    }

    pub(crate) fn record_call_outcome(&mut self, outcome: PendingCallOutcome) {
        self.pending_call_outcome = Some(outcome);
    }

    pub(crate) fn emit(&mut self, event: MatchEvent) -> EngineResult<MatchEventEnvelope> {
        let next = self
            .next_event_sequence
            .checked_add(1)
            .ok_or_else(|| EngineError::InvalidTransition("event sequence overflow".into()))?;
        let envelope = MatchEventEnvelope::new(
            self.next_event_sequence,
            MatchClockInstant::with_total_elapsed_seconds(
                self.clock.period(),
                self.clock.seconds_in_period(),
                self.clock.total_elapsed_seconds(),
            ),
            event,
        );
        self.next_event_sequence = next;
        Ok(envelope)
    }

    fn team(&self, team_id: Uuid) -> EngineResult<&TeamState> {
        if team_id == self.home.team_id() {
            Ok(&self.home)
        } else if team_id == self.away.team_id() {
            Ok(&self.away)
        } else {
            Err(EngineError::InvalidTransition("unknown team".into()))
        }
    }

    fn team_mut(&mut self, team_id: Uuid) -> EngineResult<&mut TeamState> {
        if team_id == self.home.team_id() {
            Ok(&mut self.home)
        } else if team_id == self.away.team_id() {
            Ok(&mut self.away)
        } else {
            Err(EngineError::InvalidTransition("unknown team".into()))
        }
    }

    fn opponent_id(&self, team_id: Uuid) -> EngineResult<Uuid> {
        if team_id == self.home.team_id() {
            Ok(self.away.team_id())
        } else if team_id == self.away.team_id() {
            Ok(self.home.team_id())
        } else {
            Err(EngineError::InvalidTransition("unknown team".into()))
        }
    }
}
