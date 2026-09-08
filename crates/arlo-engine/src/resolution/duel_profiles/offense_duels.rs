use crate::resolution::duel_profiles::DuelProfile;
use crate::weighting::AttributeWeight;
use arlo_domain::AttributeKey;

fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}

pub fn pass_protection_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::OffensiveBlocking, 4.5),
        w(AttributeKey::Composure, 4.5),
        w(AttributeKey::Decisions, 4.0),
        w(AttributeKey::Strength, 3.5),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Balance, 3.0),
        w(AttributeKey::Positioning, 2.5),
        w(AttributeKey::Teamwork, 2.5),
    ])
}

pub fn route_contest_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Pace, 4.5),
        w(AttributeKey::Acceleration, 4.5),
        w(AttributeKey::Agility, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::HandsReception, 3.5),
        w(AttributeKey::Positioning, 3.0),
        w(AttributeKey::Flair, 2.5),
        w(AttributeKey::Balance, 2.5),
    ])
}

pub fn run_breakthrough_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Acceleration, 4.5),
        w(AttributeKey::Pace, 4.0),
        w(AttributeKey::Balance, 4.0),
        w(AttributeKey::DriveTechnique, 4.0),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Strength, 3.5),
        w(AttributeKey::Bravery, 3.0),
        w(AttributeKey::WorkRate, 2.5),
    ])
}

pub fn central_block_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::OffensiveBlocking, 5.0),
        w(AttributeKey::Strength, 4.5),
        w(AttributeKey::Balance, 4.0),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Bravery, 3.0),
        w(AttributeKey::Teamwork, 3.0),
    ])
}

pub fn lateral_block_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::OffensiveBlocking, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Pace, 3.5),
        w(AttributeKey::Acceleration, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn artro_breakthrough_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::DriveTechnique, 5.0),
        w(AttributeKey::ArloControl, 4.5),
        w(AttributeKey::Decisions, 4.0),
        w(AttributeKey::Anticipation, 4.0),
        w(AttributeKey::Agility, 3.5),
        w(AttributeKey::Composure, 3.5),
        w(AttributeKey::Acceleration, 3.0),
        w(AttributeKey::Vision, 3.0),
    ])
}

pub fn aerial_duel_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::JumpingReach, 5.0),
        w(AttributeKey::HandsReception, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn finishing_attempt_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Finishing, 5.0),
        w(AttributeKey::Technique, 4.5),
        w(AttributeKey::Composure, 4.0),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn short_distribution_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Passing, 5.0),
        w(AttributeKey::Decisions, 4.5),
        w(AttributeKey::Composure, 4.0),
        w(AttributeKey::Technique, 3.5),
        w(AttributeKey::Teamwork, 3.5),
    ])
}

pub fn long_distribution_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Passing, 5.0),
        w(AttributeKey::Vision, 5.0),
        w(AttributeKey::ArloControl, 4.0),
        w(AttributeKey::Flair, 4.0),
        w(AttributeKey::Technique, 3.5),
    ])
}

pub fn cross_distribution_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Crossing, 5.0),
        w(AttributeKey::Vision, 4.5),
        w(AttributeKey::Flair, 4.0),
        w(AttributeKey::Technique, 3.5),
    ])
}

pub fn ball_security_carry_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::ArloControl, 5.0),
        w(AttributeKey::Balance, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Determination, 3.5),
        w(AttributeKey::Bravery, 3.0),
    ])
}

pub fn ball_security_distribution_profile() -> DuelProfile {
    DuelProfile::new(vec![
        w(AttributeKey::Composure, 5.0),
        w(AttributeKey::Decisions, 4.5),
        w(AttributeKey::ArloControl, 4.0),
        w(AttributeKey::Technique, 3.5),
    ])
}
