use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddedTimeAwarded {
    period: u32,
    added_time_seconds: f64,
    foul_count: u32,
    injury_count: u32,
    challenge_count: u32,
    time_call_count: u32,
    kick_foul_count: u32,
    scoring_count: u32,
    accumulated_dead_ball_seconds: f64,
}

impl AddedTimeAwarded {
    pub fn new(
        period: u32,
        added_time_seconds: f64,
        foul_count: u32,
        injury_count: u32,
        challenge_count: u32,
        time_call_count: u32,
        kick_foul_count: u32,
        scoring_count: u32,
        accumulated_dead_ball_seconds: f64,
    ) -> Self {
        Self {
            period,
            added_time_seconds,
            foul_count,
            injury_count,
            challenge_count,
            time_call_count,
            kick_foul_count,
            scoring_count,
            accumulated_dead_ball_seconds,
        }
    }

    pub fn period(&self) -> u32 {
        self.period
    }

    pub fn added_time_seconds(&self) -> f64 {
        self.added_time_seconds
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

    pub fn accumulated_dead_ball_seconds(&self) -> f64 {
        self.accumulated_dead_ball_seconds
    }
}
