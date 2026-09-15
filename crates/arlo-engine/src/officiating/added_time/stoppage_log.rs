use crate::officiating::added_time::stoppage_event_kind::StoppageEventKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct PeriodStoppageLog {
    foul_count: u32,
    injury_count: u32,
    challenge_count: u32,
    time_call_count: u32,
    kick_foul_count: u32,
    scoring_count: u32,
    dead_ball_seconds: f64,
}

impl PeriodStoppageLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, kind: StoppageEventKind) {
        match kind {
            StoppageEventKind::Foul => self.foul_count += 1,
            StoppageEventKind::Injury => self.injury_count += 1,
            StoppageEventKind::Challenge => self.challenge_count += 1,
            StoppageEventKind::TimeCall => self.time_call_count += 1,
            StoppageEventKind::KickFoulAwarded => self.kick_foul_count += 1,
            StoppageEventKind::Scoring => self.scoring_count += 1,
        }
    }

    pub fn add_dead_ball_seconds(&mut self, seconds: f64) {
        self.dead_ball_seconds += seconds;
    }

    pub fn foul_count(&self) -> u32 {
        self.foul_count
    }

    pub fn injury_count(&self) -> u32 {
        self.injury_count
    }

    pub fn challenge_count(&self) -> u32 {
        self.challenge_count
    }

    pub fn time_call_count(&self) -> u32 {
        self.time_call_count
    }

    pub fn kick_foul_count(&self) -> u32 {
        self.kick_foul_count
    }

    pub fn scoring_count(&self) -> u32 {
        self.scoring_count
    }

    pub fn dead_ball_seconds(&self) -> f64 {
        self.dead_ball_seconds
    }
}
