pub mod approach_alignment;
pub mod decision;
pub mod execution;
pub mod fit_scoring;

pub use approach_alignment::{
    aeriality_alignment, defensive_approach_alignment, offensive_approach_alignment,
    passing_range_alignment, physicality_alignment, press_block_shape_alignment, scalar_alignment,
    structure_alignment, transition_pace_alignment,
};
pub use decision::TacticalAdjustmentDecisionEngine;
pub use execution::{execute_tactical_adjustment, execute_tactical_adjustment_by_id};
pub use fit_scoring::score_candidate;
