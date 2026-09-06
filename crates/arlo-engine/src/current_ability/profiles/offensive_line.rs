use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::{AttributeKey, Position};

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn center_offense_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::CenterOffense,
        vec![
            w(AttributeKey::Finishing, 4.5),
            w(AttributeKey::Composure, 4.0),
            w(AttributeKey::Anticipation, 3.5),
            w(AttributeKey::Strength, 3.0),
            w(AttributeKey::Balance, 3.0),
            w(AttributeKey::Acceleration, 3.0),
            w(AttributeKey::Decisions, 2.5),
            w(AttributeKey::Positioning, 2.5),
            w(AttributeKey::Technique, 2.5),
            w(AttributeKey::Pace, 2.5),
            w(AttributeKey::Stamina, 2.0),
            w(AttributeKey::WorkRate, 2.0),
            w(AttributeKey::Determination, 2.0),
        ],
    )
}

pub fn wing_offense_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::WingOffense,
        vec![
            w(AttributeKey::Pace, 4.5),
            w(AttributeKey::Acceleration, 4.5),
            w(AttributeKey::Dribbling, 4.0),
            w(AttributeKey::Crossing, 4.0),
            w(AttributeKey::Agility, 4.0),
            w(AttributeKey::Technique, 3.5),
            w(AttributeKey::Flair, 3.0),
            w(AttributeKey::Finishing, 3.0),
            w(AttributeKey::Stamina, 3.0),
            w(AttributeKey::Composure, 2.5),
            w(AttributeKey::Passing, 2.5),
            w(AttributeKey::Vision, 2.5),
            w(AttributeKey::Anticipation, 2.5),
            w(AttributeKey::Balance, 2.5),
            w(AttributeKey::WorkRate, 2.0),
            w(AttributeKey::Decisions, 2.0),
        ],
    )
}

pub fn midcenter_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Midcenter,
        vec![
            w(AttributeKey::Passing, 4.5),
            w(AttributeKey::Vision, 4.5),
            w(AttributeKey::Technique, 4.0),
            w(AttributeKey::Decisions, 4.0),
            w(AttributeKey::Composure, 3.5),
            w(AttributeKey::Teamwork, 3.5),
            w(AttributeKey::Anticipation, 3.5),
            w(AttributeKey::Stamina, 3.0),
            w(AttributeKey::Positioning, 3.0),
            w(AttributeKey::WorkRate, 3.0),
            w(AttributeKey::Balance, 2.5),
            w(AttributeKey::Agility, 2.5),
        ],
    )
}

pub fn tight_wing_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::TightWing,
        vec![
            w(AttributeKey::Dribbling, 4.5),
            w(AttributeKey::Agility, 4.0),
            w(AttributeKey::Acceleration, 4.0),
            w(AttributeKey::Crossing, 3.5),
            w(AttributeKey::Technique, 3.5),
            w(AttributeKey::Pace, 3.5),
            w(AttributeKey::Balance, 3.0),
            w(AttributeKey::Passing, 3.0),
            w(AttributeKey::Stamina, 3.0),
            w(AttributeKey::WorkRate, 3.0),
            w(AttributeKey::Anticipation, 2.5),
            w(AttributeKey::Composure, 2.5),
            w(AttributeKey::Decisions, 2.5),
            w(AttributeKey::Finishing, 2.0),
        ],
    )
}

pub fn center_tight_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::CenterTight,
        vec![
            w(AttributeKey::Passing, 4.0),
            w(AttributeKey::Strength, 4.0),
            w(AttributeKey::Balance, 3.5),
            w(AttributeKey::Decisions, 3.5),
            w(AttributeKey::Composure, 3.5),
            w(AttributeKey::Technique, 3.5),
            w(AttributeKey::Positioning, 3.0),
            w(AttributeKey::Teamwork, 3.0),
            w(AttributeKey::WorkRate, 3.0),
            w(AttributeKey::Stamina, 3.0),
            w(AttributeKey::Anticipation, 2.5),
            w(AttributeKey::Vision, 2.0),
        ],
    )
}

pub fn corridor_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Corridor,
        vec![
            w(AttributeKey::Stamina, 4.5),
            w(AttributeKey::WorkRate, 4.5),
            w(AttributeKey::Pace, 4.0),
            w(AttributeKey::Acceleration, 4.0),
            w(AttributeKey::Teamwork, 3.5),
            w(AttributeKey::Crossing, 3.5),
            w(AttributeKey::Passing, 3.5),
            w(AttributeKey::Positioning, 3.0),
            w(AttributeKey::Anticipation, 3.0),
            w(AttributeKey::Balance, 2.5),
            w(AttributeKey::Agility, 2.5),
            w(AttributeKey::Decisions, 2.5),
            w(AttributeKey::Determination, 2.0),
        ],
    )
}
