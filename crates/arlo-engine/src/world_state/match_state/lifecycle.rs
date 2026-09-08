use crate::error::EngineResult;
use crate::lineup_runtime::hydrate;
use crate::possession::PossessionSnapshot;
use crate::rng::{MatchSeed, RngProvider};
use crate::spatial::DynamicSpatialMap;
use crate::time::RealTimeAccumulator;
use crate::world_state::clock::MatchClock;
use crate::world_state::match_state::fatigue::FatigueTracker;
use crate::world_state::match_state::impulse::ImpulseTracker;
use crate::world_state::match_state::play_calling::PlayCallTracker;
use crate::world_state::match_state::score::MatchScoreboard;
use crate::world_state::match_state::state::MatchState;
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Formation, MatchFormatRules, Player};
use arlo_math::units::Position;
use arlo_tactics::{TacticalLineup, TeamTacticalProfile};
use std::collections::HashMap;
use uuid::Uuid;

impl MatchState {
    pub fn new(
        home_team_id: Uuid,
        away_team_id: Uuid,
        home_tactical_lineup: &TacticalLineup,
        away_tactical_lineup: &TacticalLineup,
        home_formation: &Formation,
        away_formation: &Formation,
        home_roster: &[Player],
        away_roster: &[Player],
        home_tactical_profile: TeamTacticalProfile,
        away_tactical_profile: TeamTacticalProfile,
        pitch: Pitch,
        attribute_keys: HashMap<Uuid, AttributeKey>,
        format_rules: MatchFormatRules,
        seed: MatchSeed,
    ) -> EngineResult<Self> {
        let home_lineup = hydrate(home_tactical_lineup, home_formation, home_roster)?;
        let away_lineup = hydrate(away_tactical_lineup, away_formation, away_roster)?;

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
            home_tactical_profile,
            away_tactical_profile,
        );

        let impulse = ImpulseTracker::new(&home_lineup, &away_lineup, &attribute_keys);
        let fatigue = FatigueTracker::new();
        let scoreboard = MatchScoreboard::new();
        let play_calling = PlayCallTracker::new();

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
            play_calling,
        })
    }
}