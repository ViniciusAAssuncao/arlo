use crate::manager_ai::context::manager_attribute_extraction::extract_manager_attribute_value;
use arlo_domain::sport_constants::managerial::effective_manager_flexibility;
use arlo_domain::{AttributeKey, Manager, ManagerTacticalProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManagerSnapshot {
    pub tactical_knowledge: f64,
    pub offense_planning: f64,
    pub defense_organization: f64,
    pub artro_strategy: f64,
    pub adaptability: f64,
    pub artrine_communication: f64,
    pub time_call_management: f64,
    pub challenge_judgment: f64,
    pub in_game_adjustments: f64,
    pub man_management: f64,
    pub load_management: f64,
    pub player_development: f64,
    pub judging_ability: f64,
    pub judging_potential: f64,
    pub discipline: f64,
    pub tactical_profile: Option<ManagerTacticalProfile>,
    pub effective_flexibility: f64,
}

impl ManagerSnapshot {
    pub fn new(
        tactical_knowledge: f64,
        offense_planning: f64,
        defense_organization: f64,
        artro_strategy: f64,
        adaptability: f64,
        artrine_communication: f64,
        time_call_management: f64,
        challenge_judgment: f64,
        in_game_adjustments: f64,
        man_management: f64,
        load_management: f64,
        player_development: f64,
        judging_ability: f64,
        judging_potential: f64,
        discipline: f64,
        tactical_profile: Option<ManagerTacticalProfile>,
        effective_flexibility: f64,
    ) -> Self {
        Self {
            tactical_knowledge,
            offense_planning,
            defense_organization,
            artro_strategy,
            adaptability,
            artrine_communication,
            time_call_management,
            challenge_judgment,
            in_game_adjustments,
            man_management,
            load_management,
            player_development,
            judging_ability,
            judging_potential,
            discipline,
            tactical_profile,
            effective_flexibility,
        }
    }

    pub fn from_manager(manager: &Manager, attribute_keys: &HashMap<Uuid, AttributeKey>) -> Self {
        let tactical_knowledge = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::TacticalKnowledge,
        );
        let offense_planning =
            extract_manager_attribute_value(manager, attribute_keys, AttributeKey::OffensePlanning);
        let defense_organization = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::DefenseOrganization,
        );
        let artro_strategy =
            extract_manager_attribute_value(manager, attribute_keys, AttributeKey::ArtroStrategy);
        let adaptability =
            extract_manager_attribute_value(manager, attribute_keys, AttributeKey::Adaptability);
        let artrine_communication = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::ArtrineCommunication,
        );
        let time_call_management = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::TimeCallManagement,
        );
        let challenge_judgment = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::ChallengeJudgment,
        );
        let in_game_adjustments = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::InGameAdjustments,
        );
        let man_management =
            extract_manager_attribute_value(manager, attribute_keys, AttributeKey::ManManagement);
        let load_management =
            extract_manager_attribute_value(manager, attribute_keys, AttributeKey::LoadManagement);
        let player_development = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::PlayerDevelopment,
        );
        let judging_ability =
            extract_manager_attribute_value(manager, attribute_keys, AttributeKey::JudgingAbility);
        let judging_potential = extract_manager_attribute_value(
            manager,
            attribute_keys,
            AttributeKey::JudgingPotential,
        );
        let discipline =
            extract_manager_attribute_value(manager, attribute_keys, AttributeKey::Discipline);
        let tactical_profile = manager.tactical_profile().cloned();
        let flexibility_tendency = tactical_profile
            .as_ref()
            .map(|p| p.flexibility_tendency())
            .unwrap_or(0.5);
        let effective_flexibility =
            effective_manager_flexibility(adaptability, flexibility_tendency);

        Self {
            tactical_knowledge,
            offense_planning,
            defense_organization,
            artro_strategy,
            adaptability,
            artrine_communication,
            time_call_management,
            challenge_judgment,
            in_game_adjustments,
            man_management,
            load_management,
            player_development,
            judging_ability,
            judging_potential,
            discipline,
            tactical_profile,
            effective_flexibility,
        }
    }
}
