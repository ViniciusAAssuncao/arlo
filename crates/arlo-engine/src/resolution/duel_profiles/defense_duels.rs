use crate::resolution::duel_profiles::DuelProfile;
use crate::weighting::AttributeWeight;
use arlo_domain::AttributeKey;

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn pass_rush_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::PasserPressure, 5.0),
        w(AttributeKey::Acceleration, 4.5),
        w(AttributeKey::Strength, 4.5),
        w(AttributeKey::ControlledAggression, 4.0),
        w(AttributeKey::Pace, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Bravery, 3.0),
        w(AttributeKey::Anticipation, 3.0),
    ])
}

pub fn coverage_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Anticipation, 4.5),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Concentration, 3.0),
    ])
}

pub fn run_containment_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::DefensiveContainment, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::WorkRate, 3.0),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn central_resistance_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Strength, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Balance, 4.0),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::ControlledAggression, 3.5),
        w(AttributeKey::Decisions, 3.0),
    ])
}

pub fn lateral_resistance_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Strength, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn artro_defense_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::DefensiveContainment, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Anticipation, 4.0),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Teamwork, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Concentration, 3.0),
    ])
}

pub fn aerial_defense_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::JumpingReach, 5.0),
        w(AttributeKey::Strength, 4.5),
        w(AttributeKey::Positioning, 4.0),
        w(AttributeKey::Anticipation, 4.0),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn dispossession_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::ControlledAggression, 5.0),
        w(AttributeKey::DefensiveContainment, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 3.5),
    ])
}
