pub mod decision;
pub mod disciplinary_trigger;
pub mod execution;
pub mod fatigue_trigger;
pub mod injury_substitution;
pub mod replacement_selection;
pub mod tactical_trigger;

pub use decision::{SubstitutionDecisionEngine, SubstitutionPlan};
pub use disciplinary_trigger::disciplinary_urgency;
pub use execution::execute_substitutions;
pub use fatigue_trigger::{
    threshold_for_policy, urgency_for_player, urgency_for_player_with_load_management,
};
pub use injury_substitution::execute_forced_injury_substitutions;
pub use replacement_selection::best_replacement;
pub use tactical_trigger::tactical_urgency;