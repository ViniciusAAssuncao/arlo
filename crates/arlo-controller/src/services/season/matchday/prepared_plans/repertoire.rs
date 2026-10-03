use super::super::manager_match_context::ManagerMatchContext;
use arlo_domain::{
    sport_constants::effective_manager_flexibility, AttributeDefinition, AttributeKey,
};
use arlo_engine::TeamInput;
use arlo_tactics::{
    Compactness, Directness, Mentality, PressingIntensity, ScoringPatience, TeamInstructions,
    TeamTacticalProfile, Tempo, Width,
};
use std::collections::HashMap;
use uuid::Uuid;

pub(super) struct PreparationPolicy {
    pub flexibility: f64,
    pub knowledge: f64,
    pub offense: f64,
    pub defense: f64,
    pub artro: f64,
    pub load: f64,
    pub relative_strength: f64,
}

impl PreparationPolicy {
    pub fn new(
        team: &TeamInput,
        definitions: &HashMap<Uuid, AttributeDefinition>,
        context: &ManagerMatchContext,
    ) -> Self {
        let raw = |key| {
            team.manager()
                .attributes()
                .iter()
                .find(|value| {
                    definitions
                        .get(&value.attribute_definition_id())
                        .is_some_and(|definition| definition.key() == key)
                })
                .map_or(10.0, |value| f64::from(value.value()))
        };
        let skill = |key| ((raw(key) - 1.0) / 19.0).clamp(0.0, 1.0);
        let tendency = team
            .manager()
            .tactical_profile()
            .map_or(skill(AttributeKey::Adaptability), |profile| {
                profile.flexibility_tendency()
            });
        Self {
            flexibility: effective_manager_flexibility(raw(AttributeKey::Adaptability), tendency),
            knowledge: skill(AttributeKey::TacticalKnowledge),
            offense: skill(AttributeKey::OffensePlanning),
            defense: skill(AttributeKey::DefenseOrganization),
            artro: skill(AttributeKey::ArtroStrategy),
            load: skill(AttributeKey::LoadManagement),
            relative_strength: context.relative_strength,
        }
    }

    pub fn step(&self, skill: f64) -> f64 {
        self.flexibility * (0.08 + self.knowledge * 0.12 + skill * 0.14)
    }
}

pub(super) fn instruction_variants(
    team: &TeamInput,
    policy: &PreparationPolicy,
) -> Vec<TeamTacticalProfile> {
    let base = team.tactics().instructions();
    let offense = base.in_possession();
    let defense = base.out_of_possession();
    let mut variants = Vec::new();
    for (axis, skill) in [
        policy.offense,
        policy.offense,
        policy.artro,
        policy.defense,
        policy.load,
    ]
    .into_iter()
    .enumerate()
    {
        let step = policy.step(skill);
        if step < 0.08 {
            continue;
        }
        for direction in [-1.0, 1.0] {
            let change = direction * step;
            let builder = TeamInstructions::builder(offense.mentality())
                .with_in_possession(*offense)
                .with_out_of_possession(*defense)
                .with_transition(*base.transition());
            let (label, instructions) = match axis {
                0 => (
                    "Amplitude",
                    builder
                        .with_width(Width::new_clamped(offense.width().value() + change))
                        .build(),
                ),
                1 => (
                    "Ritmo",
                    builder
                        .with_tempo(Tempo::new_clamped(offense.tempo().value() + change))
                        .with_mentality(Mentality::new_clamped(
                            offense.mentality().value() + policy.relative_strength * step * 0.6,
                        ))
                        .build(),
                ),
                2 => (
                    "Circulação",
                    builder
                        .with_directness(Directness::new_clamped(
                            offense.directness().value() + change,
                        ))
                        .with_scoring_patience(ScoringPatience::new_clamped(
                            offense.scoring_patience().value() - change * 0.5,
                        ))
                        .build(),
                ),
                3 => (
                    "Pressão",
                    builder
                        .with_pressing_intensity(PressingIntensity::new_clamped(
                            defense.pressing_intensity().value() + change,
                        ))
                        .with_compactness(Compactness::new_clamped(
                            defense.compactness().value() - change * 0.3,
                        ))
                        .build(),
                ),
                _ => (
                    "Intensidade",
                    builder
                        .with_tempo(Tempo::new_clamped(offense.tempo().value() + change))
                        .with_pressing_intensity(PressingIntensity::new_clamped(
                            defense.pressing_intensity().value() + change,
                        ))
                        .build(),
                ),
            };
            if instructions == *base
                || variants
                    .iter()
                    .any(|profile: &TeamTacticalProfile| *profile.instructions() == instructions)
            {
                continue;
            }
            let name = format!(
                "{}: {} {}",
                team.tactics().name(),
                label,
                if direction > 0.0 { "+" } else { "-" }
            );
            variants.push(profile(
                team,
                name,
                instructions,
                axis as u128 * 2 + u128::from(direction > 0.0) + 1,
            ));
        }
    }
    variants
}

pub(super) fn profile(
    team: &TeamInput,
    name: String,
    instructions: TeamInstructions,
    key: u128,
) -> TeamTacticalProfile {
    let id = Uuid::from_u128(
        team.tactics().id().as_u128().rotate_left(29)
            ^ team.manager().id().as_u128().rotate_left(53)
            ^ key,
    );
    TeamTacticalProfile::new(id, team.team_id(), name, instructions, None, false)
}
