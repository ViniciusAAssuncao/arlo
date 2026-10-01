pub fn calculate_player_age_years(birthdate_unix_seconds: i64, current_unix_seconds: i64) -> f64 {
    if current_unix_seconds > birthdate_unix_seconds {
        (current_unix_seconds - birthdate_unix_seconds) as f64 / 31_557_600.0
    } else {
        0.0
    }
}

pub fn calculate_player_age(birthdate_unix_seconds: i64, current_unix_seconds: i64) -> u32 {
    calculate_player_age_years(birthdate_unix_seconds, current_unix_seconds).floor() as u32
}
