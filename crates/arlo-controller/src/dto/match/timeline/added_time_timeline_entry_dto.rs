use arlo_persistence::models::incidents::MatchAddedTimeRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddedTimeTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub added_time_seconds: f64,
    pub foul_count: i32,
    pub injury_count: i32,
    pub challenge_count: i32,
    pub time_call_count: i32,
    pub kick_foul_count: i32,
    pub scoring_count: i32,
    pub accumulated_dead_ball_seconds: f64,
}

impl AddedTimeTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchAddedTimeRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            added_time_seconds: row.added_time_seconds,
            foul_count: row.foul_count,
            injury_count: row.injury_count,
            challenge_count: row.challenge_count,
            time_call_count: row.time_call_count,
            kick_foul_count: row.kick_foul_count,
            scoring_count: row.scoring_count,
            accumulated_dead_ball_seconds: row.accumulated_dead_ball_seconds,
        }
    }
}