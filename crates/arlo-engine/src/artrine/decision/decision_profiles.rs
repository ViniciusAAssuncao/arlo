use crate::resolution::duel_profiles::DuelProfile;
use crate::weighting::AttributeWeight;
use arlo_domain::AttributeKey;

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn self_carry_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::DriveTechnique, 5.0),
        w(AttributeKey::ArloControl, 4.5),
        w(AttributeKey::Balance, 4.0),
        w(AttributeKey::Acceleration, 4.0),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Determination, 3.0),
        w(AttributeKey::Bravery, 3.0),
    ])
}

pub fn short_pass_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Passing, 5.0),
        w(AttributeKey::Decisions, 4.5),
        w(AttributeKey::Composure, 4.0),
        w(AttributeKey::Teamwork, 3.5),
        w(AttributeKey::Technique, 3.5),
    ])
}

pub fn long_launch_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Passing, 5.0),
        w(AttributeKey::Vision, 5.0),
        w(AttributeKey::ArloControl, 4.0),
        w(AttributeKey::Flair, 4.0),
        w(AttributeKey::Technique, 3.5),
    ])
}

pub fn cross_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Crossing, 5.0),
        w(AttributeKey::Vision, 4.5),
        w(AttributeKey::Flair, 4.0),
        w(AttributeKey::Technique, 3.5),
        w(AttributeKey::Decisions, 3.0),
    ])
}

pub fn self_finish_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Finishing, 5.0),
        w(AttributeKey::Technique, 4.5),
        w(AttributeKey::Composure, 4.0),
        w(AttributeKey::Flair, 3.5),
        w(AttributeKey::Anticipation, 3.0),
    ])
}
