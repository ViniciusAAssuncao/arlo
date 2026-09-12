use crate::attributes::{
    ManagerAttributeTable, PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE,
};
use crate::kick_foul::KickFoulTracker;
use crate::possession::PossessionSnapshot;
use crate::rng::RngProvider;
use crate::spatial::DynamicSpatialMap;
use crate::time::RealTimeAccumulator;
use crate::world_state::clock::MatchClock;
use crate::world_state::match_state::availability::PlayerAvailabilityTracker;
use crate::world_state::match_state::decision_cooldown::DecisionCooldownTracker;
use crate::world_state::match_state::fatigue::FatigueTracker;
use crate::world_state::match_state::foul_review::FoulReviewTracker;
use crate::world_state::match_state::impulse::ImpulseTracker;
use crate::world_state::match_state::matchday_squad::MatchdaySquad;
use crate::world_state::match_state::officiating::OfficiatingTracker;
use crate::world_state::match_state::play_call_efficacy::PlayCallEfficacyTracker;
use crate::world_state::match_state::play_calling::PlayCallTracker;
use crate::world_state::match_state::referee_registry::RefereeRegistry;
use crate::world_state::match_state::score::MatchScoreboard;
use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::pitch::Pitch;
use arlo_domain::{
    AttributeKey, FaultCatalog, InjuryCatalog, MatchFormatRules, PlayerInjuryProfile,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchState {
    pub(crate) teams: TeamRegistry,
    pub(crate) referees: RefereeRegistry,
    pub(crate) home_squad: MatchdaySquad,
    pub(crate) away_squad: MatchdaySquad,
    pub(crate) pitch: Pitch,
    pub(crate) attribute_keys: HashMap<Uuid, AttributeKey>,
    pub(crate) format_rules: MatchFormatRules,
    pub(crate) fault_catalog: Arc<FaultCatalog>,
    pub(crate) injury_catalog: Arc<InjuryCatalog>,
    pub(crate) player_injury_profiles: HashMap<Uuid, PlayerInjuryProfile>,
    pub(crate) possession: PossessionSnapshot,
    pub(crate) spatial_map: DynamicSpatialMap,
    pub(crate) clock: MatchClock,
    pub(crate) real_time: RealTimeAccumulator,
    pub(crate) rng_provider: RngProvider,
    pub(crate) event_sequence: u64,
    pub(crate) scoreboard: MatchScoreboard,
    pub(crate) fatigue: FatigueTracker,
    pub(crate) impulse: ImpulseTracker,
    pub(crate) availability: PlayerAvailabilityTracker,
    pub(crate) play_calling: PlayCallTracker,
    pub(crate) officiating: OfficiatingTracker,
    pub(crate) foul_review: FoulReviewTracker,
    pub(crate) decision_cooldown: DecisionCooldownTracker,
    pub(crate) play_call_efficacy: PlayCallEfficacyTracker,
    pub(crate) last_play_outcome_summary: Option<(Uuid, bool)>,
    pub(crate) kick_foul: KickFoulTracker,
}

impl MatchState {
    pub fn attribute_table_for(&self, player_id: &Uuid) -> &PlayerAttributeTable {
        self.teams
            .player_attribute_table(player_id)
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE)
    }

    pub fn manager_attribute_table_for(&self, team_id: Uuid) -> &ManagerAttributeTable {
        self.teams.manager_attribute_table(team_id)
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

    pub fn fault_catalog(&self) -> &FaultCatalog {
        &self.fault_catalog
    }

    pub fn fault_catalog_arc(&self) -> Arc<FaultCatalog> {
        Arc::clone(&self.fault_catalog)
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

    pub fn is_match_finished(&self) -> bool {
        self.clock.is_finished()
    }

    pub fn availability(&self) -> &PlayerAvailabilityTracker {
        &self.availability
    }

    pub fn availability_mut(&mut self) -> &mut PlayerAvailabilityTracker {
        &mut self.availability
    }

    pub fn decision_cooldown(&self) -> &DecisionCooldownTracker {
        &self.decision_cooldown
    }

    pub fn decision_cooldown_mut(&mut self) -> &mut DecisionCooldownTracker {
        &mut self.decision_cooldown
    }

    pub fn play_call_efficacy(&self) -> &PlayCallEfficacyTracker {
        &self.play_call_efficacy
    }

    pub fn play_call_efficacy_mut(&mut self) -> &mut PlayCallEfficacyTracker {
        &mut self.play_call_efficacy
    }

    pub fn last_play_outcome_summary(&self) -> Option<(Uuid, bool)> {
        self.last_play_outcome_summary
    }

    pub fn set_last_play_outcome_summary(&mut self, summary: Option<(Uuid, bool)>) {
        self.last_play_outcome_summary = summary;
    }

    pub fn kick_foul(&self) -> &KickFoulTracker {
        &self.kick_foul
    }

    pub fn kick_foul_mut(&mut self) -> &mut KickFoulTracker {
        &mut self.kick_foul
    }
}