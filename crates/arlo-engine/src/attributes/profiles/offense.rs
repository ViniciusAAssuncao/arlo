use crate::attributes::profiles::profile::{w, AttributeProfile};
use arlo_domain::AttributeKey;

pub fn pass_protection_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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

pub fn route_contest_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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

pub fn run_breakthrough_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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

pub fn central_block_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::OffensiveBlocking, 5.0),
        w(AttributeKey::Strength, 4.5),
        w(AttributeKey::Balance, 4.0),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Bravery, 3.0),
        w(AttributeKey::Teamwork, 3.0),
    ])
}

pub fn lateral_block_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::OffensiveBlocking, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Pace, 3.5),
        w(AttributeKey::Acceleration, 3.5),
        w(AttributeKey::WorkRate, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn artro_breakthrough_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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

pub fn aerial_duel_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::JumpingReach, 5.0),
        w(AttributeKey::HandsReception, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Bravery, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn finishing_attempt_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Finishing, 5.0),
        w(AttributeKey::Technique, 4.5),
        w(AttributeKey::Composure, 4.0),
        w(AttributeKey::Decisions, 3.5),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Balance, 3.0),
    ])
}

pub fn field_goal_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::GoalKicking, 5.0),
        w(AttributeKey::Composure, 4.5),
        w(AttributeKey::Technique, 4.0),
        w(AttributeKey::Concentration, 3.5),
    ])
}

pub fn short_distribution_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Passing, 5.0),
        w(AttributeKey::Decisions, 4.5),
        w(AttributeKey::Composure, 4.0),
        w(AttributeKey::Technique, 3.5),
        w(AttributeKey::Teamwork, 3.5),
    ])
}

pub fn long_distribution_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Passing, 5.0),
        w(AttributeKey::Vision, 5.0),
        w(AttributeKey::ArloControl, 4.0),
        w(AttributeKey::Flair, 4.0),
        w(AttributeKey::Technique, 3.5),
    ])
}

pub fn cross_distribution_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Crossing, 5.0),
        w(AttributeKey::Vision, 4.5),
        w(AttributeKey::Flair, 4.0),
        w(AttributeKey::Technique, 3.5),
    ])
}

pub fn ball_security_carry_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::ArloControl, 5.0),
        w(AttributeKey::Balance, 4.5),
        w(AttributeKey::Strength, 4.0),
        w(AttributeKey::Determination, 3.5),
        w(AttributeKey::Bravery, 3.0),
    ])
}

pub fn ball_security_distribution_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Composure, 5.0),
        w(AttributeKey::Decisions, 4.5),
        w(AttributeKey::ArloControl, 4.0),
        w(AttributeKey::Technique, 3.5),
    ])
}

pub fn center_offense_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn wing_offense_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn midcenter_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn tight_wing_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn center_tight_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn corridor_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn artrine_position_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn passer_position_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn wide_end_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn running_end_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
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
    ])
}

pub fn gravity_finishing_threat_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Finishing, 5.0),
        w(AttributeKey::Composure, 4.0),
        w(AttributeKey::Anticipation, 3.5),
        w(AttributeKey::Technique, 3.5),
        w(AttributeKey::Positioning, 3.0),
        w(AttributeKey::Decisions, 2.5),
    ])
}

pub fn gravity_creation_threat_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Passing, 4.5),
        w(AttributeKey::Vision, 4.5),
        w(AttributeKey::Flair, 3.5),
        w(AttributeKey::Agility, 3.0),
        w(AttributeKey::Acceleration, 3.0),
        w(AttributeKey::ArloControl, 3.0),
    ])
}

pub use artrine_position_profile as artrine_profile;
pub use passer_position_profile as passer_profile;
