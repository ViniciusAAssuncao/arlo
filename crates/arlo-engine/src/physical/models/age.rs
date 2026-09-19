use arlo_domain::Player;

pub const SECONDS_PER_YEAR: f64 = 31557600.0;
pub const AGE_DEGRADATION_THRESHOLD: f64 = 30.0;

pub fn calculate_player_age(player: &Player, current_time_unix_seconds: i64) -> f64 {
    if current_time_unix_seconds <= player.birthdate_unix_seconds() {
        if current_time_unix_seconds <= 0 {
            25.0
        } else {
            18.0
        }
    } else {
        ((current_time_unix_seconds - player.birthdate_unix_seconds()) as f64) / SECONDS_PER_YEAR
    }
}

pub fn calculate_age_degradation(age_years: f64) -> f64 {
    if age_years <= AGE_DEGRADATION_THRESHOLD {
        1.0
    } else {
        let excess = age_years - AGE_DEGRADATION_THRESHOLD;
        let factor = 2.0 / (1.0 + (0.075 * excess).exp());
        factor.clamp(0.20, 1.0)
    }
}