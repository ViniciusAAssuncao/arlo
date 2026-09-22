use arlo_domain::scoreboard::ScoreBreakdown;
use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
};
use crate::simulation::api::MatchScore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TeamScore {
    pub goal_points: u32,
    pub field_points: u32,
    pub field_goals_goalpost: u32,
    pub field_goals_fieldpost: u32,
    pub total_points: u32,
}

impl TeamScore {
    pub fn new(
        goal_points: u32,
        field_points: u32,
        field_goals_goalpost: u32,
        field_goals_fieldpost: u32,
    ) -> Self {
        let total_points = (goal_points as i32 * GOAL_POINT_VALUE
            + field_points as i32 * FIELD_POINT_VALUE
            + field_goals_goalpost as i32 * FIELD_GOAL_GOALPOST_VALUE
            + field_goals_fieldpost as i32 * FIELD_GOAL_FIELDPOST_VALUE)
            .max(0) as u32;

        Self {
            goal_points,
            field_points,
            field_goals_goalpost,
            field_goals_fieldpost,
            total_points,
        }
    }

    pub fn goal_points(&self) -> u32 {
        self.goal_points
    }

    pub fn field_points(&self) -> u32 {
        self.field_points
    }

    pub fn field_goals_goalpost(&self) -> u32 {
        self.field_goals_goalpost
    }

    pub fn field_goals_fieldpost(&self) -> u32 {
        self.field_goals_fieldpost
    }

    pub fn field_goals(&self) -> u32 {
        self.field_goals_goalpost + self.field_goals_fieldpost
    }

    pub fn total_points(&self) -> u32 {
        self.total_points
    }

    pub fn add_goal_point(&mut self) {
        self.goal_points += 1;
        self.total_points += GOAL_POINT_VALUE as u32;
    }

    pub fn add_field_point(&mut self) {
        self.field_points += 1;
        self.total_points += FIELD_POINT_VALUE as u32;
    }

    pub fn add_field_goal_goalpost(&mut self) {
        self.field_goals_goalpost += 1;
        self.total_points += FIELD_GOAL_GOALPOST_VALUE as u32;
    }

    pub fn add_field_goal_fieldpost(&mut self) {
        self.field_goals_fieldpost += 1;
        self.total_points += FIELD_GOAL_FIELDPOST_VALUE as u32;
    }

    pub fn to_score_breakdown(&self) -> ScoreBreakdown {
        ScoreBreakdown::new(
            self.goal_points,
            self.field_goals(),
            self.field_points,
            self.total_points,
        )
    }

    pub fn to_match_score(&self) -> MatchScore {
        MatchScore {
            goal_points: self.goal_points,
            field_points: self.field_points,
            field_goals_goalpost: self.field_goals_goalpost,
            field_goals_fieldpost: self.field_goals_fieldpost,
            total_points: self.total_points,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScoreState {
    home: TeamScore,
    away: TeamScore,
}

impl ScoreState {
    pub fn new() -> Self {
        Self {
            home: TeamScore::default(),
            away: TeamScore::default(),
        }
    }

    pub fn with_scores(home: TeamScore, away: TeamScore) -> Self {
        Self { home, away }
    }

    pub fn home(&self) -> &TeamScore {
        &self.home
    }

    pub fn home_mut(&mut self) -> &mut TeamScore {
        &mut self.home
    }

    pub fn away(&self) -> &TeamScore {
        &self.away
    }

    pub fn away_mut(&mut self) -> &mut TeamScore {
        &mut self.away
    }

    pub fn score_for_team(&self, team_id: Uuid, home_team_id: Uuid) -> &TeamScore {
        if team_id == home_team_id {
            &self.home
        } else {
            &self.away
        }
    }

    pub fn score_for_team_mut(&mut self, team_id: Uuid, home_team_id: Uuid) -> &mut TeamScore {
        if team_id == home_team_id {
            &mut self.home
        } else {
            &mut self.away
        }
    }

    pub fn add_goal_point(&mut self, is_home: bool) {
        if is_home {
            self.home.add_goal_point();
        } else {
            self.away.add_goal_point();
        }
    }

    pub fn add_field_point(&mut self, is_home: bool) {
        if is_home {
            self.home.add_field_point();
        } else {
            self.away.add_field_point();
        }
    }

    pub fn add_field_goal_goalpost(&mut self, is_home: bool) {
        if is_home {
            self.home.add_field_goal_goalpost();
        } else {
            self.away.add_field_goal_goalpost();
        }
    }

    pub fn add_field_goal_fieldpost(&mut self, is_home: bool) {
        if is_home {
            self.home.add_field_goal_fieldpost();
        } else {
            self.away.add_field_goal_fieldpost();
        }
    }

    pub fn total_difference(&self) -> i32 {
        self.home.total_points as i32 - self.away.total_points as i32
    }

    pub fn is_draw(&self) -> bool {
        self.home.total_points == self.away.total_points
    }

    pub fn leader(&self, home_team_id: Uuid, away_team_id: Uuid) -> Option<Uuid> {
        if self.home.total_points > self.away.total_points {
            Some(home_team_id)
        } else if self.away.total_points > self.home.total_points {
            Some(away_team_id)
        } else {
            None
        }
    }

    pub fn restore(&mut self, home: TeamScore, away: TeamScore) {
        self.home = home;
        self.away = away;
    }
}