use crate::resolution::duel_profiles::DuelProfile;
use crate::weighting::AttributeWeight;
use arlo_domain::AttributeKey;

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn shot_stopping_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Reflexes, 5.0),
        w(AttributeKey::Positioning, 4.5),
        w(AttributeKey::Handling, 4.5),
        w(AttributeKey::Agility, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Composure, 3.5),
        w(AttributeKey::OneOnOne, 3.0),
    ])
}