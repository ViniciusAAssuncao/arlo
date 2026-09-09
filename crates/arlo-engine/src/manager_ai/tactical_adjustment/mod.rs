pub mod approach_alignment;
pub mod decision;
pub mod execution;
pub mod fit_scoring;

pub use approach_alignment::{defensive_approach_alignment, offensive_approach_alignment};
pub use decision::TacticalAdjustmentDecisionEngine;
pub use execution::{execute_tactical_adjustment, execute_tactical_adjustment_by_id};
pub use fit_scoring::score_candidate;