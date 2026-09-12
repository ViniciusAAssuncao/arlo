use crate::attributes::PlayerAttributeTable;
use crate::weighting::calculate_weighted_saturated_average;
use arlo_domain::sport_constants::{
    ATTRIBUTE_MAX, ATTRIBUTE_SATURATION_MULTIPLIER, ATTRIBUTE_SATURATION_THRESHOLD,
    PROTECTIVE_AGILITY_WEIGHT, PROTECTIVE_BALANCE_WEIGHT, PROTECTIVE_NATURAL_FITNESS_WEIGHT,
    PROTECTIVE_STAMINA_WEIGHT, PROTECTIVE_STRENGTH_WEIGHT,
};
use arlo_domain::{AttributeKey, PlayerInjuryProfile};

pub fn calculate_tissue_resilience(table: &PlayerAttributeTable) -> f64 {
    let strength = table.get(AttributeKey::Strength);
    let balance = table.get(AttributeKey::Balance);
    let agility = table.get(AttributeKey::Agility);
    let natural_fitness = table.get(AttributeKey::NaturalFitness);
    let stamina = table.get(AttributeKey::Stamina);

    let pairs = [
        (strength, PROTECTIVE_STRENGTH_WEIGHT),
        (balance, PROTECTIVE_BALANCE_WEIGHT),
        (agility, PROTECTIVE_AGILITY_WEIGHT),
        (natural_fitness, PROTECTIVE_NATURAL_FITNESS_WEIGHT),
        (stamina, PROTECTIVE_STAMINA_WEIGHT),
    ];

    calculate_weighted_saturated_average(
        &pairs,
        ATTRIBUTE_SATURATION_THRESHOLD,
        ATTRIBUTE_SATURATION_MULTIPLIER,
    )
    .unwrap_or(10.0)
}

pub fn derive_intrinsic_vulnerability_factor(table: &PlayerAttributeTable) -> f64 {
    let resilience = calculate_tissue_resilience(table);
    let normalized = (resilience.clamp(0.0, ATTRIBUTE_MAX)) / ATTRIBUTE_MAX;
    (1.5 - normalized).clamp(0.5, 1.5)
}

pub fn derive_effective_susceptibility(
    table: &PlayerAttributeTable,
    profile: &PlayerInjuryProfile,
) -> f64 {
    let intrinsic = derive_intrinsic_vulnerability_factor(table);
    (profile.injury_susceptibility_multiplier() * intrinsic).max(0.1)
}
