use crate::rng::MatchSeed;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Formation, Manager, MatchFormatRules, Player};
use arlo_tactics::{PlayCall, TacticalLineup, TeamTacticalProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamSetupParams {
    pub team_id: Uuid,
    pub tactical_lineup: TacticalLineup,
    pub formation: Formation,
    pub roster: Vec<Player>,
    pub tactical_profile: TeamTacticalProfile,
    pub manager: Manager,
    pub available_profiles: Vec<TeamTacticalProfile>,
    pub playbook: Vec<PlayCall>,
}

impl TeamSetupParams {
    pub fn new(
        team_id: Uuid,
        tactical_lineup: TacticalLineup,
        formation: Formation,
        roster: Vec<Player>,
        tactical_profile: TeamTacticalProfile,
        manager: Manager,
        available_profiles: Vec<TeamTacticalProfile>,
        playbook: Vec<PlayCall>,
    ) -> Self {
        Self {
            team_id,
            tactical_lineup,
            formation,
            roster,
            tactical_profile,
            manager,
            available_profiles,
            playbook,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchSetupParams {
    pub home: TeamSetupParams,
    pub away: TeamSetupParams,
    pub pitch: Pitch,
    pub attribute_keys: HashMap<Uuid, AttributeKey>,
    pub format_rules: MatchFormatRules,
    pub seed: MatchSeed,
}

impl MatchSetupParams {
    pub fn new(
        home: TeamSetupParams,
        away: TeamSetupParams,
        pitch: Pitch,
        attribute_keys: HashMap<Uuid, AttributeKey>,
        format_rules: MatchFormatRules,
        seed: MatchSeed,
    ) -> Self {
        Self {
            home,
            away,
            pitch,
            attribute_keys,
            format_rules,
            seed,
        }
    }
}
