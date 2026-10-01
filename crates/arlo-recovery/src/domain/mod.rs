pub mod conditioning_profile;
pub mod fatigue_condition;
pub mod morale_condition;
pub mod injury_record;
pub mod injury_status_kind;
pub mod player_condition;

pub use conditioning_profile::ConditioningProfile;
pub use fatigue_condition::FatigueCondition;
pub use morale_condition::MoraleCondition;
pub type ImpulseCondition = MoraleCondition;
pub use injury_record::InjuryRecord;
pub use injury_status_kind::InjuryStatusKind;
pub use player_condition::PlayerCondition;
