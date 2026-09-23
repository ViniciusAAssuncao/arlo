use arlo_persistence::models::incidents::MatchScoringPlayRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoringTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub team_id: String,
    pub scorer_id: String,
    pub artrine_id: Option<String>,
    pub assister_id: Option<String>,
    pub play_type: String,
    pub points: i32,
    pub scoring_post: String,
    pub drives_completed: Option<i32>,
    pub territory_advance_mirim: Option<f64>,
}

impl ScoringTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchScoringPlayRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            team_id: row.team_id.clone(),
            scorer_id: row.scorer_id.clone(),
            artrine_id: row.artrine_id.clone(),
            assister_id: row.assister_id.clone(),
            play_type: row.play_type.clone(),
            points: row.points,
            scoring_post: row.scoring_post.clone(),
            drives_completed: row.drives_completed,
            territory_advance_mirim: row.territory_advance_mirim,
        }
    }
}
