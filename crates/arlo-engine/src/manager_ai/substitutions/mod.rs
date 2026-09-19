pub mod budget_policy;
pub mod decision;
pub mod disciplinary_trigger;
pub mod execution;
pub mod fatigue_trigger;
pub mod forced_departure_detection;
pub mod injury_substitution;
pub mod plan_builder;
pub mod replacement_selection;
pub mod tactical_trigger;
pub mod urgency_ranking;
pub mod value_of_substitution;

pub use budget_policy::SubstitutionBudgetPolicy;
pub use decision::{SubstitutionDecisionEngine, SubstitutionPlan};
pub use disciplinary_trigger::disciplinary_urgency;
pub use execution::execute_substitutions;
pub use fatigue_trigger::{
    threshold_for_policy, urgency_for_player, urgency_for_player_with_load_management,
};
pub use forced_departure_detection::{detect_forced_departures, forced_departures_for_team};
pub use injury_substitution::{
    apply_forced_substitution_intents, execute_forced_injury_substitutions,
    resolve_forced_substitutions_for_team,
};
pub use plan_builder::build_substitution_plans;
pub use replacement_selection::best_replacement;
pub use tactical_trigger::tactical_urgency;
pub use urgency_ranking::{rank_substitution_urgency, PlayerUrgency};
pub use value_of_substitution::{evaluate_best_substitution_value, SubstitutionValueAssessment};
