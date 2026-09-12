pub mod decision;
pub mod eligibility;
pub mod estimator;
pub mod event_stimulus;
pub mod event_translation;
pub mod period_weighting;
pub mod referee_recognition;
pub mod stoppage_event_kind;
pub mod stoppage_log;
pub mod tracker;

pub use decision::AddedTimeDecisionEngine;
pub use eligibility::{
    is_added_time_eligible_period, is_first_half_end, is_overtime_period, is_second_half_end,
};
pub use estimator::estimate_added_time_seconds;
pub use event_stimulus::{
    calculate_dead_ball_excess_seconds, calculate_event_stimulus_seconds,
    calculate_raw_stimulus_seconds,
};
pub use event_translation::translate_added_time_awarded;
pub use period_weighting::period_added_time_multiplier;
pub use referee_recognition::referee_recognition_scale;
pub use stoppage_event_kind::StoppageEventKind;
pub use stoppage_log::PeriodStoppageLog;
pub use tracker::AddedTimeTracker;
