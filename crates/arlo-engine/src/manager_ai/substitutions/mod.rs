pub mod decision;
pub mod execution;
pub mod fatigue_trigger;
pub mod replacement_selection;
pub mod tactical_trigger;

pub use decision::{SubstitutionDecisionEngine, SubstitutionPlan};
pub use execution::execute_substitutions;
pub use fatigue_trigger::{
    threshold_for_policy, urgency_for_player, urgency_for_player_with_load_management,
};
pub use replacement_selection::best_replacement;
pub use tactical_trigger::tactical_urgency;
