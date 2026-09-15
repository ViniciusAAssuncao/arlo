use crate::officiating::added_time::stoppage_log::PeriodStoppageLog;
use arlo_domain::sport_constants::{
    ADDED_TIME_BASELINE_DEAD_BALL_SECONDS, ADDED_TIME_CHALLENGE_WEIGHT_SECONDS,
    ADDED_TIME_DEAD_BALL_EXCESS_WEIGHT, ADDED_TIME_FOUL_WEIGHT_SECONDS,
    ADDED_TIME_INJURY_WEIGHT_SECONDS, ADDED_TIME_KICK_FOUL_WEIGHT_SECONDS,
    ADDED_TIME_SCORING_WEIGHT_SECONDS, ADDED_TIME_TIME_CALL_WEIGHT_SECONDS,
};

pub fn calculate_event_stimulus_seconds(log: &PeriodStoppageLog) -> f64 {
    (log.foul_count() as f64) * ADDED_TIME_FOUL_WEIGHT_SECONDS
        + (log.injury_count() as f64) * ADDED_TIME_INJURY_WEIGHT_SECONDS
        + (log.challenge_count() as f64) * ADDED_TIME_CHALLENGE_WEIGHT_SECONDS
        + (log.time_call_count() as f64) * ADDED_TIME_TIME_CALL_WEIGHT_SECONDS
        + (log.kick_foul_count() as f64) * ADDED_TIME_KICK_FOUL_WEIGHT_SECONDS
        + (log.scoring_count() as f64) * ADDED_TIME_SCORING_WEIGHT_SECONDS
}

pub fn calculate_dead_ball_excess_seconds(log: &PeriodStoppageLog) -> f64 {
    let excess = (log.dead_ball_seconds() - ADDED_TIME_BASELINE_DEAD_BALL_SECONDS).max(0.0);
    excess * ADDED_TIME_DEAD_BALL_EXCESS_WEIGHT
}

pub fn calculate_raw_stimulus_seconds(log: &PeriodStoppageLog) -> f64 {
    calculate_event_stimulus_seconds(log) + calculate_dead_ball_excess_seconds(log)
}
