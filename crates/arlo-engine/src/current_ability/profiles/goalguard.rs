use crate::current_ability::weights::{AttributeWeight, PositionWeightProfile};
use arlo_domain::{AttributeKey, Position};

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn goalguard_profile() -> PositionWeightProfile {
    PositionWeightProfile::new(
        Position::Goalguard,
        vec![
            w(AttributeKey::Reflexes, 5.0),
            w(AttributeKey::Handling, 4.5),
            w(AttributeKey::Positioning, 4.5),
            w(AttributeKey::RushingOut, 3.5),
            w(AttributeKey::Communication, 3.5),
            w(AttributeKey::Concentration, 3.5),
            w(AttributeKey::Agility, 3.5),
            w(AttributeKey::Composure, 3.0),
            w(AttributeKey::Anticipation, 3.0),
            w(AttributeKey::Bravery, 3.0),
            w(AttributeKey::Distribution, 2.5),
            w(AttributeKey::Decisions, 2.0),
        ],
    )
}
