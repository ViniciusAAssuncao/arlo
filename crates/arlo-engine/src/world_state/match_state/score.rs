use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
};
use arlo_events::ScoringPost;
use arlo_formatter::ScoreBreakdown;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TeamScore {
    pub goal_points: u32,
    pub field_goals: u32,
    pub field_points: u32,
    pub total_points: u32,
}

impl TeamScore {
    pub fn new(goal_points: u32, field_goals: u32, field_points: u32, total_points: u32) -> Self {
        Self {
            goal_points,
            field_goals,
            field_points,
            total_points,
        }
    }

    pub fn to_breakdown(&self) -> ScoreBreakdown {
        ScoreBreakdown::new(
            self.goal_points,
            self.field_goals,
            self.field_points,
            self.total_points,
        )
    }

    pub fn record_goal_point(&mut self) {
        self.goal_points += 1;
        self.total_points += GOAL_POINT_VALUE as u32;
    }

    pub fn record_field_point(&mut self) {
        self.field_points += 1;
        self.total_points += FIELD_POINT_VALUE as u32;
    }

    pub fn record_field_goal(&mut self, post: ScoringPost) {
        let points = match post {
            ScoringPost::Goalpost => FIELD_GOAL_GOALPOST_VALUE as u32,
            ScoringPost::Fieldpost => FIELD_GOAL_FIELDPOST_VALUE as u32,
        };
        self.field_goals += 1;
        self.total_points += points;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchScoreboard {
    home_score: TeamScore,
    away_score: TeamScore,
    drives_in_current_series: u32,
    last_action_score_occurred: bool,
}

impl MatchScoreboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn home_score(&self) -> TeamScore {
        self.home_score
    }

    pub fn away_score(&self) -> TeamScore {
        self.away_score
    }

    pub fn drives_in_current_series(&self) -> u32 {
        self.drives_in_current_series
    }

    pub fn increment_drives(&mut self) {
        self.drives_in_current_series += 1;
    }

    pub fn reset_drives(&mut self) {
        self.drives_in_current_series = 0;
    }

    pub fn last_action_score_occurred(&self) -> bool {
        self.last_action_score_occurred
    }

    pub fn record_goal_point(&mut self, is_home: bool) {
        if is_home {
            self.home_score.record_goal_point();
        } else {
            self.away_score.record_goal_point();
        }
        self.last_action_score_occurred = true;
    }

    pub fn record_field_point(&mut self, is_home: bool) {
        if is_home {
            self.home_score.record_field_point();
        } else {
            self.away_score.record_field_point();
        }
        self.last_action_score_occurred = true;
    }

    pub fn record_field_goal(&mut self, is_home: bool, post: ScoringPost) {
        if is_home {
            self.home_score.record_field_goal(post);
        } else {
            self.away_score.record_field_goal(post);
        }
        self.last_action_score_occurred = true;
    }
}