use crate::attributes::RefereeAttributeTable;
use crate::officiating::added_time::estimator::estimate_added_time_seconds;
use crate::officiating::added_time::stoppage_log::PeriodStoppageLog;
use arlo_domain::sport_constants::{
    ADDED_TIME_CONSISTENCY_NOISE_SCALE_SECONDS, ATTRIBUTE_MAX, ATTRIBUTE_MIN,
    MAX_ADDED_TIME_SECONDS,
};
use arlo_domain::{AttributeKey, MatchFormatRules};
use arlo_math::stats::noise::sample_gaussian_noise;
use arlo_math::units::Duration;
use rand::Rng;

pub struct AddedTimeDecisionEngine;

impl AddedTimeDecisionEngine {
    pub fn evaluate<R: Rng + ?Sized>(
        log: &PeriodStoppageLog,
        referee_table: &RefereeAttributeTable,
        period: u32,
        format_rules: &MatchFormatRules,
        rng: &mut R,
    ) -> Duration {
        let base_estimate = estimate_added_time_seconds(log, referee_table, period, format_rules);
        let consistency_raw = referee_table
            .get(AttributeKey::Consistency)
            .clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX);
        let consistency_norm = (consistency_raw - ATTRIBUTE_MIN) / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);
        let noise_scale = ADDED_TIME_CONSISTENCY_NOISE_SCALE_SECONDS * (1.0 - consistency_norm);
        let noise = sample_gaussian_noise(noise_scale, rng);
        let total = (base_estimate + noise).clamp(0.0, MAX_ADDED_TIME_SECONDS);
        Duration::new(total)
    }
}
