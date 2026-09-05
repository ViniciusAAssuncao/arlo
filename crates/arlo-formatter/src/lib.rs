pub mod domain_errors;
pub mod scoreboard_display;

pub use domain_errors::format_domain_error;
pub use scoreboard_display::{
    format_match_result, format_score, format_score_breakdown, ScoreBreakdown,
};
