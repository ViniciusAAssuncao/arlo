use arlo_events::{AddedTimeAwarded, MatchClockInstant};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct MatchAddedTimeRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub added_time_seconds: f64,
    pub foul_count: i32,
    pub injury_count: i32,
    pub challenge_count: i32,
    pub time_call_count: i32,
    pub kick_foul_count: i32,
    pub scoring_count: i32,
    pub accumulated_dead_ball_seconds: f64,
}

impl MatchAddedTimeRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
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
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            added_time_seconds,
            foul_count: foul_count as i32,
            injury_count: injury_count as i32,
            challenge_count: challenge_count as i32,
            time_call_count: time_call_count as i32,
            kick_foul_count: kick_foul_count as i32,
            scoring_count: scoring_count as i32,
            accumulated_dead_ball_seconds,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &AddedTimeAwarded,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            event.period(),
            clock.seconds_in_period(),
            event.added_time_seconds(),
            event.foul_count(),
            event.injury_count(),
            event.challenge_count(),
            event.time_call_count(),
            event.kick_foul_count(),
            event.scoring_count(),
            event.accumulated_dead_ball_seconds(),
        )
    }
}
