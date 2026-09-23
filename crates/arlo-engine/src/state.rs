use crate::input::{MatchInput, TeamInput};
use arlo_domain::sport_constants::MINIMUM_ADVANCE_MIRINS_PER_SERIES;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchPhase {
    Pregame,
    Ready,
    Live,
    Stopped,
    BonusPhase,
    KickFoul,
    PeriodBreak,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClockState {
    period: u32,
    seconds_in_period: f64,
    total_elapsed_seconds: f64,
    added_seconds: f64,
    running: bool,
}

impl ClockState {
    pub fn period(&self) -> u32 {
        self.period
    }
    pub fn seconds_in_period(&self) -> f64 {
        self.seconds_in_period
    }
    pub fn total_elapsed_seconds(&self) -> f64 {
        self.total_elapsed_seconds
    }
    pub fn added_seconds(&self) -> f64 {
        self.added_seconds
    }
    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DriveProgress {
    partial_artros: u8,
    completed_drives: u32,
}

impl DriveProgress {
    pub fn partial_artros(&self) -> u8 {
        self.partial_artros
    }
    pub fn completed_drives(&self) -> u32 {
        self.completed_drives
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    goal_points: u32,
    field_points: u32,
    field_goals: u32,
    total_points: u32,
}

impl Score {
    pub fn goal_points(&self) -> u32 {
        self.goal_points
    }
    pub fn field_points(&self) -> u32 {
        self.field_points
    }
    pub fn field_goals(&self) -> u32 {
        self.field_goals
    }
    pub fn total_points(&self) -> u32 {
        self.total_points
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeriesState {
    team_id: Uuid,
    down: u8,
    distance_to_gain_mirim: f64,
    origin_mirim: f64,
    valid_advance_mirim: f64,
    suspended: bool,
}

impl SeriesState {
    pub fn team_id(&self) -> Uuid {
        self.team_id
    }
    pub fn down(&self) -> u8 {
        self.down
    }
    pub fn distance_to_gain_mirim(&self) -> f64 {
        self.distance_to_gain_mirim
    }
    pub fn origin_mirim(&self) -> f64 {
        self.origin_mirim
    }
    pub fn valid_advance_mirim(&self) -> f64 {
        self.valid_advance_mirim
    }
    pub fn is_suspended(&self) -> bool {
        self.suspended
    }
}

#[derive(Debug, Clone)]
pub struct TeamState {
    team_id: Uuid,
    active_player_ids: Vec<Uuid>,
    reserve_player_ids: Vec<Uuid>,
    drive_progress: DriveProgress,
    score: Score,
}

impl TeamState {
    fn from_input(input: &TeamInput) -> Self {
        let active_player_ids: Vec<_> = input
            .lineup()
            .assignments()
            .iter()
            .map(|a| a.player_id())
            .collect();
        let reserve_player_ids = input
            .roster()
            .iter()
            .map(|player| player.id())
            .filter(|id| !active_player_ids.contains(id))
            .collect();
        Self {
            team_id: input.team_id(),
            active_player_ids,
            reserve_player_ids,
            drive_progress: DriveProgress::default(),
            score: Score::default(),
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }
    pub fn active_player_ids(&self) -> &[Uuid] {
        &self.active_player_ids
    }
    pub fn reserve_player_ids(&self) -> &[Uuid] {
        &self.reserve_player_ids
    }
    pub fn drive_progress(&self) -> DriveProgress {
        self.drive_progress
    }
    pub fn score(&self) -> Score {
        self.score
    }
}

#[derive(Debug, Clone)]
pub struct MatchState {
    match_id: Uuid,
    phase: MatchPhase,
    clock: ClockState,
    home: TeamState,
    away: TeamState,
    possessor_team_id: Option<Uuid>,
    next_call_team_id: Option<Uuid>,
    series: Option<SeriesState>,
    last_valid_possession_mirim: Option<f64>,
    next_event_sequence: u64,
    rng: ChaCha8Rng,
}

impl MatchState {
    pub fn new(input: &MatchInput) -> Self {
        let home_team_id = input.home().team_id();
        let midfield_mirim = input.pitch().length_mirim() / 2.0;
        Self {
            match_id: input.match_id(),
            phase: MatchPhase::Ready,
            clock: ClockState {
                period: 1,
                seconds_in_period: 0.0,
                total_elapsed_seconds: 0.0,
                added_seconds: 0.0,
                running: false,
            },
            home: TeamState::from_input(input.home()),
            away: TeamState::from_input(input.away()),
            possessor_team_id: Some(home_team_id),
            next_call_team_id: Some(home_team_id),
            series: Some(SeriesState {
                team_id: home_team_id,
                down: 1,
                distance_to_gain_mirim: MINIMUM_ADVANCE_MIRINS_PER_SERIES,
                origin_mirim: midfield_mirim,
                valid_advance_mirim: 0.0,
                suspended: false,
            }),
            last_valid_possession_mirim: Some(midfield_mirim),
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
    pub fn possessor_team_id(&self) -> Option<Uuid> {
        self.possessor_team_id
    }
    pub fn next_call_team_id(&self) -> Option<Uuid> {
        self.next_call_team_id
    }
    pub fn series(&self) -> Option<SeriesState> {
        self.series
    }
    pub fn last_valid_possession_mirim(&self) -> Option<f64> {
        self.last_valid_possession_mirim
    }
    pub fn next_event_sequence(&self) -> u64 {
        self.next_event_sequence
    }
}
