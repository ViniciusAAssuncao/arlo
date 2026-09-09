use crate::error::EngineResult;
use crate::lineup_runtime::hydrate;
use crate::possession::PossessionSnapshot;
use crate::rng::RngProvider;
use crate::spatial::DynamicSpatialMap;
use crate::time::RealTimeAccumulator;
use crate::world_state::clock::MatchClock;
use crate::world_state::match_state::fatigue::FatigueTracker;
use crate::world_state::match_state::impulse::ImpulseTracker;
use crate::world_state::match_state::matchday_squad::MatchdaySquad;
use crate::world_state::match_state::officiating::OfficiatingTracker;
use crate::world_state::match_state::play_calling::PlayCallTracker;
use crate::world_state::match_state::score::MatchScoreboard;
use crate::world_state::match_state::setup_params::MatchSetupParams;
use crate::world_state::match_state::state::MatchState;
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_math::units::Position;

impl MatchState {
    pub fn new(params: MatchSetupParams) -> EngineResult<Self> {
        let home_lineup = hydrate(
            &params.home.tactical_lineup,
            &params.home.formation,
            &params.home.roster,
        )?;
        let away_lineup = hydrate(
            &params.away.tactical_lineup,
            &params.away.formation,
            &params.away.roster,
        )?;

        let spatial_map = DynamicSpatialMap::from_pitch(&params.pitch, &home_lineup, &away_lineup)?;
        let initial_scrimmage = Position::from_components(
            params.pitch.length().value() / 2.0,
            params.pitch.width().value() / 2.0,
            0.0,
        );
        let possession = PossessionSnapshot::opening(
            params.home.team_id,
            params.away.team_id,
            initial_scrimmage,
        );
        let rng_provider = RngProvider::new(params.seed);
        let clock = MatchClock::new(&params.format_rules);
        let real_time = RealTimeAccumulator::new();

        let teams = TeamRegistry::new(
            params.home.team_id,
            params.away.team_id,
            home_lineup.clone(),
            away_lineup.clone(),
            params.home.tactical_profile,
            params.away.tactical_profile,
            params.home.manager,
            params.away.manager,
        );

        let home_squad = MatchdaySquad::from_roster_and_lineup(&params.home.roster, &home_lineup);
        let away_squad = MatchdaySquad::from_roster_and_lineup(&params.away.roster, &away_lineup);

        let impulse = ImpulseTracker::new(&home_lineup, &away_lineup, &params.attribute_keys);
        let fatigue = FatigueTracker::new();
        let scoreboard = MatchScoreboard::new();
        let play_calling = PlayCallTracker::new();
        let officiating = OfficiatingTracker::new();

        Ok(Self {
            teams,
            home_squad,
            away_squad,
            pitch: params.pitch,
            attribute_keys: params.attribute_keys,
            format_rules: params.format_rules,
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
            officiating,
        })
    }
}