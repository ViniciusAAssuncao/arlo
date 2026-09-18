use crate::attributes::profiles::profile::{w, AttributeProfile};
use arlo_domain::AttributeKey;

pub fn pass_rush_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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

pub fn coverage_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Anticipation, 4.5),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Concentration, 3.0),
    ])
}

pub fn run_containment_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::DefensiveContainment, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::WorkRate, 3.0),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn central_resistance_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Strength, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Balance, 4.0),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::ControlledAggression, 3.5),
        w(AttributeKey::Decisions, 3.0),
    ])
}

pub fn lateral_resistance_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Strength, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn artro_defense_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::DefensiveContainment, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Anticipation, 4.0),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Teamwork, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Concentration, 3.0),
    ])
}

pub fn aerial_defense_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::JumpingReach, 5.0),
        w(AttributeKey::Strength, 4.5),
        w(AttributeKey::Positioning, 4.0),
        w(AttributeKey::Anticipation, 4.0),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn dispossession_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::ControlledAggression, 5.0),
        w(AttributeKey::DefensiveContainment, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 3.5),
    ])
}

pub fn pass_rusher_position_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Acceleration, 4.5),
        w(AttributeKey::Pace, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Anticipation, 3.0),
        w(AttributeKey::Stamina, 3.0),
        w(AttributeKey::Determination, 3.0),
        w(AttributeKey::Balance, 2.5),
        w(AttributeKey::Decisions, 2.0),
        w(AttributeKey::Positioning, 2.0),
        w(AttributeKey::Concentration, 1.5),
        w(AttributeKey::Composure, 1.5),
    ])
}

pub fn lineback_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Positioning, 4.0),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 4.0),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Stamina, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Concentration, 3.0),
        w(AttributeKey::Balance, 2.5),
        w(AttributeKey::Acceleration, 2.5),
        w(AttributeKey::Teamwork, 2.0),
        w(AttributeKey::Determination, 2.0),
        w(AttributeKey::Composure, 1.5),
    ])
}

pub fn fullback_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Stamina, 4.5),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Positioning, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Crossing, 3.0),
        w(AttributeKey::Passing, 3.0),
        w(AttributeKey::Teamwork, 3.0),
        w(AttributeKey::Anticipation, 3.0),
        w(AttributeKey::Balance, 2.5),
        w(AttributeKey::Decisions, 2.5),
        w(AttributeKey::Agility, 2.0),
        w(AttributeKey::Concentration, 2.0),
        w(AttributeKey::Bravery, 2.0),
        w(AttributeKey::Determination, 1.5),
    ])
}

pub fn centerback_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Composure, 3.0),
        w(AttributeKey::Decisions, 3.0),
        w(AttributeKey::Concentration, 3.0),
        w(AttributeKey::Balance, 2.5),
        w(AttributeKey::Pace, 2.0),
        w(AttributeKey::Acceleration, 2.0),
        w(AttributeKey::Stamina, 2.0),
        w(AttributeKey::Teamwork, 2.0),
    ])
}

pub fn defensive_end_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Strength, 4.5),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Stamina, 3.5),
        w(AttributeKey::Anticipation, 3.0),
        w(AttributeKey::Agility, 3.0),
        w(AttributeKey::Positioning, 2.5),
        w(AttributeKey::Balance, 2.5),
        w(AttributeKey::Decisions, 2.0),
        w(AttributeKey::Determination, 2.0),
        w(AttributeKey::Concentration, 2.0),
    ])
}

pub fn rougieback_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Positioning, 4.0),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Stamina, 3.5),
        w(AttributeKey::Bravery, 3.0),
        w(AttributeKey::Concentration, 3.0),
        w(AttributeKey::WorkRate, 3.0),
        w(AttributeKey::Balance, 2.5),
        w(AttributeKey::Decisions, 2.5),
        w(AttributeKey::Strength, 2.0),
        w(AttributeKey::Teamwork, 2.0),
        w(AttributeKey::Determination, 1.5),
    ])
}

pub fn defensive_blocker_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Strength, 5.0),
        w(AttributeKey::Positioning, 4.0),
        w(AttributeKey::Bravery, 4.0),
        w(AttributeKey::Balance, 3.5),
        w(AttributeKey::Concentration, 3.0),
        w(AttributeKey::Composure, 3.0),
        w(AttributeKey::Decisions, 3.0),
        w(AttributeKey::Anticipation, 2.5),
        w(AttributeKey::Stamina, 2.0),
        w(AttributeKey::WorkRate, 2.0),
        w(AttributeKey::Teamwork, 1.5),
        w(AttributeKey::Determination, 1.5),
    ])
}

pub fn wide_blocker_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Pace, 4.5),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Stamina, 3.5),
        w(AttributeKey::Positioning, 3.5),
        w(AttributeKey::WorkRate, 3.0),
        w(AttributeKey::Balance, 3.0),
        w(AttributeKey::Anticipation, 3.0),
        w(AttributeKey::Bravery, 2.5),
        w(AttributeKey::Strength, 2.5),
        w(AttributeKey::Decisions, 2.0),
        w(AttributeKey::Concentration, 2.0),
        w(AttributeKey::Teamwork, 1.5),
        w(AttributeKey::Determination, 1.5),
    ])
}

pub fn outside_zonerback_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Anticipation, 4.0),
        w(AttributeKey::Agility, 3.0),
        w(AttributeKey::Stamina, 3.0),
        w(AttributeKey::Decisions, 3.0),
        w(AttributeKey::Concentration, 2.5),
        w(AttributeKey::Composure, 2.5),
        w(AttributeKey::Balance, 2.0),
        w(AttributeKey::WorkRate, 2.0),
        w(AttributeKey::Teamwork, 2.0),
        w(AttributeKey::Determination, 1.5),
    ])
}

pub fn middle_zonerback_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Anticipation, 4.5),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Composure, 3.5),
        w(AttributeKey::Teamwork, 3.0),
        w(AttributeKey::Strength, 3.0),
        w(AttributeKey::Stamina, 3.0),
        w(AttributeKey::Concentration, 3.0),
        w(AttributeKey::Balance, 2.0),
        w(AttributeKey::Bravery, 2.0),
        w(AttributeKey::Acceleration, 2.0),
        w(AttributeKey::Pace, 2.0),
        w(AttributeKey::Leadership, 1.5),
    ])
}

pub use pass_rusher_position_profile as pass_rusher_profile;
