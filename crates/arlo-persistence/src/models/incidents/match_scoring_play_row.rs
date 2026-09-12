use arlo_events::{
    FieldGoalScored, FieldPointScored, GoalPointScored, MatchClockInstant, ScoringAttemptMissed,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchScoringPlayRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
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

impl MatchScoringPlayRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        team_id: Uuid,
        scorer_id: Uuid,
        artrine_id: Option<Uuid>,
        assister_id: Option<Uuid>,
        play_type: impl Into<String>,
        points: u32,
        scoring_post: impl Into<String>,
        drives_completed: Option<u32>,
        territory_advance_mirim: Option<f64>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            team_id: team_id.to_string(),
            scorer_id: scorer_id.to_string(),
            artrine_id: artrine_id.map(|id| id.to_string()),
            assister_id: assister_id.map(|id| id.to_string()),
            play_type: play_type.into(),
            points: points as i32,
            scoring_post: scoring_post.into(),
            drives_completed: drives_completed.map(|d| d as i32),
            territory_advance_mirim,
        }
    }

    pub fn from_goal_point(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &GoalPointScored,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.scorer_id(),
            Some(event.artrine_id()),
            event.assister_id(),
            "GoalPoint",
            event.points(),
            event.post().as_str(),
            Some(event.drives_completed()),
            None,
        )
    }

    pub fn from_field_point(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &FieldPointScored,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.scorer_id(),
            None,
            None,
            "FieldPoint",
            event.points(),
            event.post().as_str(),
            Some(event.drives_completed()),
            Some(event.territory_advance_mirim()),
        )
    }

    pub fn from_field_goal(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &FieldGoalScored,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.scorer_id(),
            None,
            None,
            "FieldGoal",
            event.points(),
            event.post().as_str(),
            None,
            None,
        )
    }

    pub fn from_missed_attempt(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &ScoringAttemptMissed,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.scorer_id(),
            None,
            None,
            "MissedAttempt",
            0,
            event.attempted_post().as_str(),
            None,
            None,
        )
    }
}
