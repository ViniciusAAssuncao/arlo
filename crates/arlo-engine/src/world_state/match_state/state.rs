use crate::error::EngineResult;
use crate::physical::FatigueState;
use crate::possession::PossessionSnapshot;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::event_bus::ImpulseEventBus;
use crate::psychology::systems::events::{ImpulseEvent, ImpulseShift};
use crate::rng::{MatchSeed, RngProvider};
use crate::spatial::DynamicSpatialMap;
use crate::tactics::Lineup;
use crate::time::RealTimeAccumulator;
use crate::world_state::clock::MatchClock;
use crate::world_state::match_state::fatigue::FatigueTracker;
use crate::world_state::match_state::impulse::ImpulseTracker;
use crate::world_state::match_state::score::{MatchScoreboard, TeamScore};
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, MatchFormatRules, Player, Position as DomainPosition};
use arlo_events::ScoringPost;
use arlo_math::units::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchState {
    teams: TeamRegistry,
    pitch: Pitch,
    attribute_keys: HashMap<Uuid, AttributeKey>,
    format_rules: MatchFormatRules,
    possession: PossessionSnapshot,
    spatial_map: DynamicSpatialMap,
    clock: MatchClock,
    real_time: RealTimeAccumulator,
    rng_provider: RngProvider,
    event_sequence: u64,
    scoreboard: MatchScoreboard,
    fatigue: FatigueTracker,
    impulse: ImpulseTracker,
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

        let teams = TeamRegistry::new(
            home_team_id,
            away_team_id,
            home_lineup.clone(),
            away_lineup.clone(),
        );

        let impulse = ImpulseTracker::new(&home_lineup, &away_lineup, &attribute_keys);
        let fatigue = FatigueTracker::new();
        let scoreboard = MatchScoreboard::new();

        Ok(Self {
            teams,
            pitch,
            attribute_keys,
            format_rules,
            possession,
            spatial_map,
            clock,
            real_time,
            rng_provider,
            event_sequence: 0,
            scoreboard,
            fatigue,
            impulse,
        })
    }

    pub fn home_team_id(&self) -> Uuid {
        self.teams.home_team_id()
    }

    pub fn away_team_id(&self) -> Uuid {
        self.teams.away_team_id()
    }

    pub fn home_lineup(&self) -> &Lineup {
        self.teams.home_lineup()
    }

    pub fn away_lineup(&self) -> &Lineup {
        self.teams.away_lineup()
    }

    pub fn home_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.home_offensive_position_index()
    }

    pub fn home_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.home_defensive_position_index()
    }

    pub fn away_offensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.away_offensive_position_index()
    }

    pub fn away_defensive_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.teams.away_defensive_position_index()
    }

    pub fn home_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.position_index_for_team(self.teams.home_team_id())
    }

    pub fn away_position_index(&self) -> &HashMap<Uuid, DomainPosition> {
        self.position_index_for_team(self.teams.away_team_id())
    }

    pub fn offensive_position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        self.teams.offensive_position_index_for_team(team_id)
    }

    pub fn defensive_position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        self.teams.defensive_position_index_for_team(team_id)
    }

    pub fn position_index_for_team(&self, team_id: Uuid) -> &HashMap<Uuid, DomainPosition> {
        self.teams.position_index_for_team(
            team_id,
            self.possession.role().is_offense(team_id),
        )
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

    pub fn record_goal_point(&mut self, team_id: Uuid) {
        self.scoreboard
            .record_goal_point(team_id == self.teams.home_team_id());
    }

    pub fn record_field_point(&mut self, team_id: Uuid) {
        self.scoreboard
            .record_field_point(team_id == self.teams.home_team_id());
    }

    pub fn record_field_goal(&mut self, team_id: Uuid, post: ScoringPost) {
        self.scoreboard
            .record_field_goal(team_id == self.teams.home_team_id(), post);
    }

    pub fn is_match_finished(&self) -> bool {
        self.clock.is_finished()
    }

    pub fn home_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        self.fatigue.home_fatigue()
    }

    pub fn away_fatigue(&self) -> &HashMap<Uuid, FatigueState> {
        self.fatigue.away_fatigue()
    }

    pub fn fatigue_for(&self, player_id: &Uuid) -> FatigueState {
        self.fatigue.fatigue_for(player_id)
    }

    pub fn home_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        self.impulse.home_impulse()
    }

    pub fn away_impulse(&self) -> &HashMap<Uuid, ImpulseState> {
        self.impulse.away_impulse()
    }

    pub fn impulse_bus(&self) -> &ImpulseEventBus {
        self.impulse.impulse_bus()
    }

    pub fn impulse_bus_mut(&mut self) -> &mut ImpulseEventBus {
        self.impulse.impulse_bus_mut()
    }

    pub fn impulse_for(&self, player_id: &Uuid) -> ImpulseState {
        self.impulse.impulse_for(player_id)
    }

    pub fn apply_impulse_event(
        &mut self,
        player_id: Uuid,
        event: &ImpulseEvent,
        timestamp_seconds: f64,
    ) -> Option<ImpulseShift> {
        self.impulse.apply_impulse_event(
            player_id,
            event,
            timestamp_seconds,
            &self.teams,
            &self.fatigue,
            &self.attribute_keys,
        )
    }

    pub fn process_impulse_bus(
        &mut self,
        timestamp_seconds: f64,
    ) -> Vec<(Uuid, ImpulseShift, ImpulseEvent)> {
        self.impulse.process_impulse_bus(
            timestamp_seconds,
            &self.teams,
            &self.fatigue,
            &self.attribute_keys,
        )
    }

    pub fn record_distance(&mut self, player_id: Uuid, mirim: f64) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let player = self.teams.find_player(&player_id);
        self.fatigue
            .record_distance(player_id, mirim, is_home, player, &self.attribute_keys)
    }

    pub fn apply_duel_anaerobic_cost(
        &mut self,
        player_id: Uuid,
        duration_seconds: f64,
        intensity: f64,
    ) -> (f64, f64) {
        let is_home = self.teams.is_home_player(&player_id);
        let player = self.teams.find_player(&player_id);
        self.fatigue.apply_duel_anaerobic_cost(
            player_id,
            duration_seconds,
            intensity,
            is_home,
            player,
            &self.attribute_keys,
        )
    }

    pub fn apply_dead_ball_recovery(&mut self, dead_ball_seconds: f64) -> Vec<(Uuid, f64, f64)> {
        let home_players = self.teams.home_lineup().players();
        let away_players = self.teams.away_lineup().players();
        self.fatigue.apply_dead_ball_recovery(
            dead_ball_seconds,
            &home_players,
            &away_players,
            &self.attribute_keys,
        )
    }

    pub fn player_fatigue_multiplier(&self, player: &Player) -> f64 {
        self.fatigue
            .player_fatigue_multiplier(player, &self.attribute_keys)
    }
}