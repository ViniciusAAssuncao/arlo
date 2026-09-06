use crate::error::EngineResult;
use crate::fatigue::{compute_player_fatigue_multiplier, FatigueState};
use crate::possession::PossessionSnapshot;
use crate::rng::{MatchSeed, RngProvider};
use crate::spatial::DynamicSpatialMap;
use crate::tactics::Lineup;
use crate::time::RealTimeAccumulator;
use crate::world_state::clock::MatchClock;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
};
use arlo_domain::{AttributeKey, MatchFormatRules, Player, Position as DomainPosition};
use arlo_events::ScoringPost;
use arlo_formatter::ScoreBreakdown;
use arlo_math::units::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchState {
    home_team_id: Uuid,
    away_team_id: Uuid,
    home_lineup: Lineup,
    away_lineup: Lineup,
    home_offensive_position_index: HashMap<Uuid, DomainPosition>,
    home_defensive_position_index: HashMap<Uuid, DomainPosition>,
    away_offensive_position_index: HashMap<Uuid, DomainPosition>,
    away_defensive_position_index: HashMap<Uuid, DomainPosition>,
    pitch: Pitch,
    attribute_keys: HashMap<Uuid, AttributeKey>,
    format_rules: MatchFormatRules,
    possession: PossessionSnapshot,
    spatial_map: DynamicSpatialMap,
    clock: MatchClock,
    real_time: RealTimeAccumulator,
    rng_provider: RngProvider,
    event_sequence: u64,
    home_score: TeamScore,
    away_score: TeamScore,
    drives_in_current_series: u32,
    last_action_score_occurred: bool,
    home_fatigue: HashMap<Uuid, FatigueState>,
    away_fatigue: HashMap<Uuid, FatigueState>,
}

impl MatchState {
    pub fn new(
        home_team_id: Uuid,
        away_team_id: Uuid,
        home_lineup: Lineup,
        away_lineup: Lineup,
        pitch: Pitch,
        attribute_keys: HashMap<Uuid, AttributeKey>,
        format_rules: MatchFormatRules,
        seed: MatchSeed,
    ) -> EngineResult<Self> {
        let spatial_map = DynamicSpatialMap::from_pitch(&pitch, &home_lineup, &away_lineup)?;
        let initial_scrimmage = Position::from_components(
            pitch.length().value() / 2.0,
            pitch.width().value() / 2.0,
            0.0,
        );
        let possession = PossessionSnapshot::opening(home_team_id, away_team_id, initial_scrimmage);
        let rng_provider = RngProvider::new(seed);
        let clock = MatchClock::new(&format_rules);
        let real_time = RealTimeAccumulator::new();
        let home_offensive_position_index = home_lineup.offensive_position_index();
        let home_defensive_position_index = home_lineup.defensive_position_index();
        let away_offensive_position_index = away_lineup.offensive_position_index();
        let away_defensive_position_index = away_lineup.defensive_position_index();

        Ok(Self {
            home_team_id,
            away_team_id,
            home_lineup,
            away_lineup,
            home_offensive_position_index,
            home_defensive_position_index,
            away_offensive_position_index,
            away_defensive_position_index,
            pitch,
            attribute_keys,
            format_rules,
            possession,
            spatial_map,
            clock,
            real_time,
            rng_provider,
            event_sequence: 0,
            home_score: TeamScore::default(),
            away_score: TeamScore::default(),
            drives_in_current_series: 0,
            last_action_score_occurred: false,
            home_fatigue: HashMap::new(),
            away_fatigue: HashMap::new(),
        })
    }

    pub fn home_team_id(&self) -> Uuid {
        self.home_team_id
    }

    pub fn away_team_id(&self) -> Uuid {
        self.away_team_id
    }

    pub fn home_lineup(&self) -> &Lineup {
        &self.home_lineup
    }

    pub fn away_lineup(&self) -> &Lineup {
        &self.away_lineup
    }

    pub fn home_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.home_offensive_position_index
    }

    pub fn home_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.home_defensive_position_index
    }

    pub fn away_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.away_offensive_position_index
    }

    pub fn away_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        &self.away_defensive_position_index
    }

    pub fn home_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.position_index_for_team(self.home_team_id)
    }

    pub fn away_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.position_index_for_team(self.away_team_id)
    }

    pub fn offensive_position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        if team_id == self.home_team_id {
            &self.home_offensive_position_index
        } else {
            &self.away_offensive_position_index
        }
    }

    pub fn defensive_position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        if team_id == self.home_team_id {
            &self.home_defensive_position_index
        } else {
            &self.away_defensive_position_index
        }
    }

    pub fn position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        if self.possession.role().is_offense(team_id) {
            self.offensive_position_index_for_team(team_id)
        } else {
            self.defensive_position_index_for_team(team_id)
        }
    }

    pub fn pitch(&self) -> &Pitch {
        &self.pitch
    }

    pub fn attribute_keys(&self) -> &HashMap<Uuid, AttributeKey> {
        &self.attribute_keys
    }

    pub fn format_rules(&self) -> &MatchFormatRules {
        &self.format_rules
    }

    pub fn possession(&self) -> &PossessionSnapshot {
        &self.possession
    }

    pub fn possession_mut(&mut self) -> &mut PossessionSnapshot {
        &mut self.possession
    }

    pub fn spatial_map(&self) -> &DynamicSpatialMap {
        &self.spatial_map
    }

    pub fn spatial_map_mut(&mut self) -> &mut DynamicSpatialMap {
        &mut self.spatial_map
    }

    pub fn clock(&self) -> &MatchClock {
        &self.clock
    }

    pub fn clock_mut(&mut self) -> &mut MatchClock {
        &mut self.clock
    }

    pub fn real_time(&self) -> &RealTimeAccumulator {
        &self.real_time
    }

    pub fn real_time_mut(&mut self) -> &mut RealTimeAccumulator {
        &mut self.real_time
    }

    pub fn rng_provider(&self) -> &RngProvider {
        &self.rng_provider
    }

    pub fn event_sequence(&self) -> u64 {
        self.event_sequence
    }

    pub fn next_sequence(&mut self) -> u64 {
        let seq = self.event_sequence;
        self.event_sequence += 1;
        seq
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

    pub fn record_goal_point(&mut self, team_id: Uuid) {
        let score = if team_id == self.home_team_id {
            &mut self.home_score
        } else {
            &mut self.away_score
        };
        score.goal_points += 1;
        score.total_points += GOAL_POINT_VALUE as u32;
        self.last_action_score_occurred = true;
    }

    pub fn record_field_point(&mut self, team_id: Uuid) {
        let score = if team_id == self.home_team_id {
            &mut self.home_score
        } else {
            &mut self.away_score
        };
        score.field_points += 1;
        score.total_points += FIELD_POINT_VALUE as u32;
        self.last_action_score_occurred = true;
    }

    pub fn record_field_goal(&mut self, team_id: Uuid, post: ScoringPost) {
        let points = match post {
            ScoringPost::Goalpost => FIELD_GOAL_GOALPOST_VALUE as u32,
            ScoringPost::Fieldpost => FIELD_GOAL_FIELDPOST_VALUE as u32,
        };
        let score = if team_id == self.home_team_id {
            &mut self.home_score
        } else {
            &mut self.away_score
        };
        score.field_goals += 1;
        score.total_points += points;
        self.last_action_score_occurred = true;
    }

    pub fn is_match_finished(&self) -> bool {
        self.clock.is_finished()
    }

    pub fn home_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        &self.home_fatigue
    }

    pub fn away_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        &self.away_fatigue
    }

    pub fn fatigue_for(&self, player_id: &Uuid) -> FatigueState {
        self.home_fatigue
            .get(player_id)
            .or_else(|| self.away_fatigue.get(player_id))
            .copied()
            .unwrap_or_default()
    }

    pub fn record_distance(&mut self, player_id: Uuid, mirim: f64) {
        if self.home_offensive_position_index.contains_key(&player_id) {
            self.home_fatigue
                .entry(player_id)
                .or_default()
                .add_distance(mirim);
        } else {
            self.away_fatigue
                .entry(player_id)
                .or_default()
                .add_distance(mirim);
        }
    }

    pub fn player_fatigue_multiplier(&self, player: &Player) -> f64 {
        let fatigue = self.fatigue_for(&player.id());
        compute_player_fatigue_multiplier(player, &fatigue, &self.attribute_keys)
    }
}