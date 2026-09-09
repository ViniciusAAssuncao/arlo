use crate::world_state::match_state::score::TeamScore;
use crate::world_state::match_state::state::MatchState;
use arlo_events::ScoringPost;
use uuid::Uuid;

impl MatchState {
    pub fn home_score(&self) -> TeamScore {
        self.scoreboard.home_score()
    }

    pub fn away_score(&self) -> TeamScore {
        self.scoreboard.away_score()
    }

    pub fn drives_in_current_series(&self) -> u32 {
        self.scoreboard.drives_in_current_series()
    }

    pub fn increment_drives(&mut self) {
        self.scoreboard.increment_drives();
    }

    pub fn reset_drives(&mut self) {
        self.scoreboard.reset_drives();
    }

    pub fn last_action_score_occurred(&self) -> bool {
        self.scoreboard.last_action_score_occurred()
    }

    pub fn last_scoring_team(&self) -> Option<Uuid> {
        self.scoreboard.last_scoring_team()
    }

    pub fn record_goal_point(&mut self, team_id: Uuid) {
        self.scoreboard
            .record_goal_point(team_id == self.teams.home_team_id(), team_id);
    }

    pub fn record_field_point(&mut self, team_id: Uuid) {
        self.scoreboard
            .record_field_point(team_id == self.teams.home_team_id(), team_id);
    }

    pub fn record_field_goal(&mut self, team_id: Uuid, post: ScoringPost) {
        self.scoreboard
            .record_field_goal(team_id == self.teams.home_team_id(), post, team_id);
    }
}
