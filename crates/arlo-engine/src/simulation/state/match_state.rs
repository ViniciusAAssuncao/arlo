use super::availability::PlayerAvailabilityTracker;
use super::clock::MatchClock;
use super::possession_state::PossessionState;
use super::score_state::{ScoreState, TeamScore};
use super::series_state::SeriesState;
use arlo_domain::pitch::zone::PitchZone;
use arlo_domain::sport_constants::{
    AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM, FIRST_ZONE_DEPTH_MIRIM, GOAL_POINT_REQUIRED_DRIVES,
    PITCH_LENGTH_MIRIM_MIN,
};
use arlo_domain::MatchFormatRules;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchState {
    match_id: Uuid,
    home_team_id: Uuid,
    away_team_id: Uuid,
    pitch_length_mirim: f64,
    second_zone_depth_mirim: f64,
    field_progress_mirim: f64,
    clock: MatchClock,
    possession: PossessionState,
    series: SeriesState,
    score: ScoreState,
    availability: PlayerAvailabilityTracker,
}

impl MatchState {
    pub fn new(
        match_id: Uuid,
        home_team_id: Uuid,
        away_team_id: Uuid,
        pitch_length_mirim: f64,
        second_zone_depth_mirim: f64,
        field_progress_mirim: f64,
        clock: MatchClock,
        possession: PossessionState,
        series: SeriesState,
        score: ScoreState,
        availability: PlayerAvailabilityTracker,
    ) -> Self {
        Self {
            match_id,
            home_team_id,
            away_team_id,
            pitch_length_mirim,
            second_zone_depth_mirim,
            field_progress_mirim: field_progress_mirim.clamp(0.0, pitch_length_mirim),
            clock,
            possession,
            series,
            score,
            availability,
        }
    }

    pub fn initial(
        match_id: Uuid,
        home_team_id: Uuid,
        away_team_id: Uuid,
        format_rules: &MatchFormatRules,
        pitch_length_mirim: f64,
        second_zone_depth_mirim: f64,
    ) -> Self {
        let length = pitch_length_mirim.max(PITCH_LENGTH_MIRIM_MIN);
        let second_depth = if second_zone_depth_mirim > 0.0 {
            second_zone_depth_mirim
        } else {
            AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM
        };
        let initial_progress = length * 0.5;

        Self {
            match_id,
            home_team_id,
            away_team_id,
            pitch_length_mirim: length,
            second_zone_depth_mirim: second_depth,
            field_progress_mirim: initial_progress,
            clock: MatchClock::new(format_rules),
            possession: PossessionState::new(home_team_id, away_team_id),
            series: SeriesState::default_ruleset(),
            score: ScoreState::new(),
            availability: PlayerAvailabilityTracker::new(),
        }
    }

    pub fn match_id(&self) -> Uuid {
        self.match_id
    }

    pub fn home_team_id(&self) -> Uuid {
        self.home_team_id
    }

    pub fn away_team_id(&self) -> Uuid {
        self.away_team_id
    }

    pub fn pitch_length_mirim(&self) -> f64 {
        self.pitch_length_mirim
    }

    pub fn second_zone_depth_mirim(&self) -> f64 {
        self.second_zone_depth_mirim
    }

    pub fn field_progress_mirim(&self) -> f64 {
        self.field_progress_mirim
    }

    pub fn set_field_progress_mirim(&mut self, mirim: f64) {
        self.field_progress_mirim = mirim.clamp(0.0, self.pitch_length_mirim);
    }

    pub fn clock(&self) -> &MatchClock {
        &self.clock
    }

    pub fn clock_mut(&mut self) -> &mut MatchClock {
        &mut self.clock
    }

    pub fn possession(&self) -> &PossessionState {
        &self.possession
    }

    pub fn possession_mut(&mut self) -> &mut PossessionState {
        &mut self.possession
    }

    pub fn series(&self) -> &SeriesState {
        &self.series
    }

    pub fn series_mut(&mut self) -> &mut SeriesState {
        &mut self.series
    }

    pub fn score(&self) -> &ScoreState {
        &self.score
    }

    pub fn score_mut(&mut self) -> &mut ScoreState {
        &mut self.score
    }

    pub fn availability(&self) -> &PlayerAvailabilityTracker {
        &self.availability
    }

    pub fn availability_mut(&mut self) -> &mut PlayerAvailabilityTracker {
        &mut self.availability
    }

    pub fn is_offense_home(&self) -> bool {
        self.possession.offense_team_id() == self.home_team_id
    }

    pub fn distance_to_goal_mirim(&self) -> f64 {
        (self.pitch_length_mirim - self.field_progress_mirim).max(0.0)
    }

    pub fn normalized_progress(&self) -> f64 {
        (self.field_progress_mirim / self.pitch_length_mirim.max(1.0)).clamp(0.0, 1.0)
    }

    pub fn current_zone(&self) -> PitchZone {
        let dist = self.distance_to_goal_mirim();
        if dist <= FIRST_ZONE_DEPTH_MIRIM {
            PitchZone::FirstZone
        } else if dist <= FIRST_ZONE_DEPTH_MIRIM + self.second_zone_depth_mirim {
            PitchZone::SecondZone
        } else {
            PitchZone::OpenField
        }
    }

    pub fn record_play_advance(&mut self, mirins: f64) {
        let new_progress = (self.field_progress_mirim + mirins).clamp(0.0, self.pitch_length_mirim);
        self.field_progress_mirim = new_progress;
        self.series.record_advance(mirins);
        if self.series.has_achieved_target() && !self.series.is_bonus_phase() {
            self.series.reset_downs();
        }
    }

    pub fn record_drive(&mut self) {
        self.series.record_drive();
    }

    pub fn advance_down(&mut self) -> bool {
        self.series.advance_down()
    }

    pub fn turnover(&mut self, new_carrier_id: Option<Uuid>) {
        self.field_progress_mirim =
            (self.pitch_length_mirim - self.field_progress_mirim).clamp(0.0, self.pitch_length_mirim);
        self.possession.swap();
        self.possession.set_carrier_id(new_carrier_id);
        self.series.reset();
    }

    pub fn turnover_to_team(&mut self, new_offense_team_id: Uuid, new_carrier_id: Option<Uuid>) {
        if self.possession.offense_team_id() != new_offense_team_id {
            self.field_progress_mirim = (self.pitch_length_mirim - self.field_progress_mirim)
                .clamp(0.0, self.pitch_length_mirim);
            self.possession.set_offense(new_offense_team_id);
            self.possession.set_carrier_id(new_carrier_id);
            self.series.reset();
        } else {
            self.possession.set_carrier_id(new_carrier_id);
        }
    }

    pub fn turnover_on_downs(&mut self) {
        self.turnover(None);
    }

    pub fn record_goal_point(&mut self) -> bool {
        if !self.series.can_attempt_goal_point(GOAL_POINT_REQUIRED_DRIVES) {
            return false;
        }
        let is_home = self.is_offense_home();
        self.score.add_goal_point(is_home);
        self.series.set_bonus_phase(true);
        let bonus_spot = (self.pitch_length_mirim
            - (FIRST_ZONE_DEPTH_MIRIM + self.second_zone_depth_mirim))
            .max(0.0);
        self.field_progress_mirim = bonus_spot;
        true
    }

    pub fn record_field_point(&mut self) -> bool {
        if !self.series.can_attempt_field_point(1) {
            return false;
        }
        let is_home = self.is_offense_home();
        self.score.add_field_point(is_home);
        self.turnover(None);
        self.field_progress_mirim = self.pitch_length_mirim * 0.5;
        true
    }

    pub fn record_field_goal_goalpost(&mut self) {
        let is_home = self.is_offense_home();
        self.score.add_field_goal_goalpost(is_home);
        self.turnover(None);
        self.field_progress_mirim = self.pitch_length_mirim * 0.5;
    }

    pub fn record_field_goal_fieldpost(&mut self) {
        let is_home = self.is_offense_home();
        self.score.add_field_goal_fieldpost(is_home);
        self.turnover(None);
        self.field_progress_mirim = self.pitch_length_mirim * 0.5;
    }

    pub fn advance_time(&mut self, delta_seconds: f64) -> bool {
        let period_ended = self.clock.advance_seconds(delta_seconds);
        self.availability.advance_time(delta_seconds);
        period_ended
    }

    pub fn is_match_finished(&self) -> bool {
        self.clock.is_finished()
    }

    pub fn use_time_call(&mut self, team_id: Uuid) -> bool {
        let is_home = team_id == self.home_team_id;
        self.clock.use_time_call(is_home)
    }

    pub fn use_challenge(&mut self, team_id: Uuid, success: bool) -> bool {
        let is_home = team_id == self.home_team_id;
        self.clock.use_challenge(is_home, success)
    }

    pub fn winner(&self) -> Option<Uuid> {
        self.score.leader(self.home_team_id, self.away_team_id)
    }

    pub fn home_score(&self) -> TeamScore {
        *self.score.home()
    }

    pub fn away_score(&self) -> TeamScore {
        *self.score.away()
    }

    pub fn restore_scoreboard(
        &mut self,
        home: TeamScore,
        away: TeamScore,
        drives_in_current_series: u32,
    ) {
        self.score.restore(home, away);
        self.series.set_drives(drives_in_current_series);
    }
}