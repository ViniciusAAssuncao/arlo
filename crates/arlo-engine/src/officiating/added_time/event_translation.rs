use crate::officiating::added_time::stoppage_log::PeriodStoppageLog;
use arlo_events::AddedTimeAwarded;

pub fn translate_added_time_awarded(
    period: u32,
    awarded_seconds: f64,
    log: &PeriodStoppageLog,
) -> AddedTimeAwarded {
    AddedTimeAwarded::new(
        period,
        awarded_seconds,
        log.foul_count(),
        log.injury_count(),
        log.challenge_count(),
        log.time_call_count(),
        log.kick_foul_count(),
        log.scoring_count(),
        log.dead_ball_seconds(),
    )
}
