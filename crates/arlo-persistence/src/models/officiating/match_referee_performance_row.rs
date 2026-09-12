use arlo_stats::RefereeMatchStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchRefereePerformanceRow {
    pub id: String,
    pub match_id: String,
    pub referee_id: String,
    pub role: String,
    pub calls_made: i32,
    pub calls_correct: i32,
    pub calls_incorrect: i32,
    pub peace_referee_interventions: i32,
}

impl MatchRefereePerformanceRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        referee_id: Uuid,
        role: impl Into<String>,
        calls_made: u32,
        calls_correct: u32,
        calls_incorrect: u32,
        peace_referee_interventions: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            referee_id: referee_id.to_string(),
            role: role.into(),
            calls_made: calls_made as i32,
            calls_correct: calls_correct as i32,
            calls_incorrect: calls_incorrect as i32,
            peace_referee_interventions: peace_referee_interventions as i32,
        }
    }

    pub fn for_head_referee(
        id: Uuid,
        match_id: Uuid,
        referee_id: Uuid,
        stats: &RefereeMatchStats,
    ) -> Self {
        Self::new(
            id,
            match_id,
            referee_id,
            "Head",
            stats.calls_made,
            stats.calls_correct,
            stats.calls_incorrect,
            0,
        )
    }

    pub fn for_peace_referee(
        id: Uuid,
        match_id: Uuid,
        referee_id: Uuid,
        stats: &RefereeMatchStats,
    ) -> Self {
        Self::new(
            id,
            match_id,
            referee_id,
            "Peace",
            stats.calls_made,
            0,
            0,
            stats.peace_referee_interventions,
        )
    }
}
