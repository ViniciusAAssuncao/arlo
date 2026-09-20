use crate::attributes::profiles::profile::{w, AttributeProfile};
use arlo_domain::AttributeKey;

pub fn shot_stopping_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Reflexes, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Handling, 4.5),
        w(AttributeKey::Agility, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Composure, 3.5),
        w(AttributeKey::OneOnOne, 3.0),
    ])
}

pub fn goalguard_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}
