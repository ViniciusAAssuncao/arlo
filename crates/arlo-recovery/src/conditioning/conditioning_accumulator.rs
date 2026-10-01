use crate::tuning::RecoveryTuningProfile;

pub fn advance_conditioning(
    current_score: f64,
    was_active_today: bool,
    is_injured_today: bool,
    tuning: &RecoveryTuningProfile,
) -> f64 {
    let delta = if was_active_today {
        tuning.conditioning_daily_gain_active
    } else if is_injured_today {
        -tuning.conditioning_daily_loss_injured
    } else {
        -tuning.conditioning_daily_loss_inactive
    };

    (current_score + delta).clamp(0.0, 1.0)
}

pub fn advance_conditioning_days(
    current_score: f64,
    active_days: u32,
    inactive_days: u32,
    injured_days: u32,
    tuning: &RecoveryTuningProfile,
) -> f64 {
    let mut score = current_score;
    for _ in 0..active_days {
        score = advance_conditioning(score, true, false, tuning);
    }
    for _ in 0..inactive_days {
        score = advance_conditioning(score, false, false, tuning);
    }
    for _ in 0..injured_days {
        score = advance_conditioning(score, false, true, tuning);
    }
    score
}
