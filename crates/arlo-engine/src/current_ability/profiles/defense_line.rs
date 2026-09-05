use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::{AttributeKey, Position};

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn centerback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Centerback,
        vec![
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
        ],
    )
}

pub fn defensive_end_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::DefensiveEnd,
        vec![
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
        ],
    )
}

pub fn rougieback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Rougieback,
        vec![
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
        ],
    )
}

pub fn defensive_blocker_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::DefensiveBlocker,
        vec![
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
        ],
    )
}

pub fn wide_blocker_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::WideBlocker,
        vec![
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
        ],
    )
}

pub fn outside_zonerback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::OutsideZonerback,
        vec![
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
        ],
    )
}

pub fn middle_zonerback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::MiddleZonerback,
        vec![
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
        ],
    )
}
