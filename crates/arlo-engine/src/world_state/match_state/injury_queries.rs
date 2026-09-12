use crate::world_state::match_state::state::MatchState;
use arlo_domain::{InjuryCatalog, PlayerInjuryProfile};
use std::sync::Arc;
use uuid::Uuid;

impl MatchState {
    pub fn injury_catalog(&self) -> &InjuryCatalog {
        &self.injury_catalog
    }

    pub fn injury_catalog_arc(&self) -> Arc<InjuryCatalog> {
        Arc::clone(&self.injury_catalog)
    }

    pub fn player_injury_profile(&self, id: &Uuid) -> PlayerInjuryProfile {
        self.player_injury_profiles
            .get(id)
            .copied()
            .unwrap_or_default()
    }
}