use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::{AttributeKey, Position};

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn artrine_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Artrine,
        vec![
            w(AttributeKey::Passing, 4.5),
            w(AttributeKey::Vision, 4.5),
            w(AttributeKey::Technique, 4.0),
            w(AttributeKey::Decisions, 4.0),
            w(AttributeKey::Composure, 3.5),
            w(AttributeKey::Anticipation, 3.5),
            w(AttributeKey::Teamwork, 3.0),
            w(AttributeKey::Balance, 2.5),
            w(AttributeKey::Stamina, 2.5),
            w(AttributeKey::Flair, 2.0),
            w(AttributeKey::WorkRate, 2.0),
            w(AttributeKey::Concentration, 2.0),
        ],
    )
}

pub fn passer_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Passer,
        vec![
            w(AttributeKey::Passing, 5.0),
            w(AttributeKey::Vision, 4.5),
            w(AttributeKey::Decisions, 4.5),
            w(AttributeKey::Composure, 4.0),
            w(AttributeKey::Technique, 4.0),
            w(AttributeKey::Anticipation, 3.5),
            w(AttributeKey::Concentration, 3.0),
            w(AttributeKey::Teamwork, 3.0),
            w(AttributeKey::Strength, 2.5),
            w(AttributeKey::Balance, 2.5),
            w(AttributeKey::Stamina, 2.0),
            w(AttributeKey::Leadership, 2.0),
            w(AttributeKey::Positioning, 2.0),
            w(AttributeKey::Determination, 2.0),
        ],
    )
}

pub fn pass_rusher_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::PassRusher,
        vec![
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
        ],
    )
}

pub fn wide_end_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::WideEnd,
        vec![
            w(AttributeKey::Pace, 4.5),
            w(AttributeKey::Acceleration, 4.5),
            w(AttributeKey::Agility, 4.0),
            w(AttributeKey::Balance, 3.5),
            w(AttributeKey::Anticipation, 3.0),
            w(AttributeKey::Stamina, 3.0),
            w(AttributeKey::Flair, 2.5),
            w(AttributeKey::Positioning, 2.5),
            w(AttributeKey::Composure, 2.5),
            w(AttributeKey::Technique, 2.5),
            w(AttributeKey::Crossing, 2.0),
            w(AttributeKey::WorkRate, 2.0),
            w(AttributeKey::Bravery, 2.0),
            w(AttributeKey::Determination, 1.5),
        ],
    )
}

pub fn running_end_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::RunningEnd,
        vec![
            w(AttributeKey::Acceleration, 4.5),
            w(AttributeKey::Pace, 4.5),
            w(AttributeKey::Balance, 4.0),
            w(AttributeKey::Agility, 4.0),
            w(AttributeKey::Strength, 3.5),
            w(AttributeKey::Stamina, 3.5),
            w(AttributeKey::Bravery, 3.0),
            w(AttributeKey::WorkRate, 3.0),
            w(AttributeKey::Anticipation, 3.0),
            w(AttributeKey::Composure, 2.5),
            w(AttributeKey::Decisions, 2.5),
            w(AttributeKey::Determination, 2.5),
            w(AttributeKey::Dribbling, 2.0),
            w(AttributeKey::Positioning, 1.5),
            w(AttributeKey::Flair, 1.5),
        ],
    )
}

pub fn lineback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Lineback,
        vec![
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
        ],
    )
}

pub fn fullback_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Fullback,
        vec![
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
        ],
    )
}
