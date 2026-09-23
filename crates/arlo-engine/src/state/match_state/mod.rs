mod live;
mod period;
mod scoring;

use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{ClockState, MatchPhase, PossessionState, SeriesState, TeamState};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
struct SuspendedRestart {
    team_id: Uuid,
    series: SeriesState,
    position_mirim: f64,
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
    pitch_length_mirim: f64,
    next_event_sequence: u64,
    _rng: ChaCha8Rng,
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
            pitch_length_mirim,
            next_event_sequence: 1,
            _rng: ChaCha8Rng::seed_from_u64(input.seed()),
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
    pub fn series(&self) -> SeriesState {
        self.series
    }
    pub fn last_valid_possession_mirim(&self) -> f64 {
        self.possession.ball_position_mirim()
    }
    pub fn next_event_sequence(&self) -> u64 {
        self.next_event_sequence
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
