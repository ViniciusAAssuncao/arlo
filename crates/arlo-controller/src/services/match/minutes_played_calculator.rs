use crate::services::r#match::match_clock_math::{
    calculate_total_elapsed_seconds, MatchClockDurationConfig,
};
use arlo_persistence::models::MatchSubstitutionRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMinutesPlayedResult {
    pub minutes_played: u32,
    pub seconds_played: f64,
    pub entry_instant_seconds: Option<f64>,
    pub exit_instant_seconds: Option<f64>,
    pub left_due_to_incident: bool,
}

pub fn calculate_minutes_played_with_player_subs(
    was_starter: bool,
    sub_in: Option<&MatchSubstitutionRow>,
    sub_out: Option<&MatchSubstitutionRow>,
    final_availability_status: &str,
    _final_suspended_remaining_seconds: Option<f64>,
    clock_config: &MatchClockDurationConfig,
    total_match_duration_seconds: f64,
) -> PlayerMinutesPlayedResult {
    let entry_instant = if was_starter {
        Some(0.0)
    } else {
        sub_in.map(|s| {
            calculate_total_elapsed_seconds(
                clock_config,
                s.period.max(0) as u32,
                s.seconds_in_period.max(0.0),
            )
        })
    };

    let Some(entry_sec) = entry_instant else {
        return PlayerMinutesPlayedResult {
            minutes_played: 0,
            seconds_played: 0.0,
            entry_instant_seconds: None,
            exit_instant_seconds: None,
            left_due_to_incident: false,
        };
    };

    let exit_instant = sub_out.map(|s| {
        calculate_total_elapsed_seconds(
            clock_config,
            s.period.max(0) as u32,
            s.seconds_in_period.max(0.0),
        )
    });

    let reason_is_incident = sub_out
        .map(|s| {
            let r = s.reason.to_lowercase();
            r.contains("injury")
                || r.contains("lesão")
                || r.contains("expulsion")
                || r.contains("expuls")
        })
        .unwrap_or(false);

    let status_is_incident = final_availability_status.eq_ignore_ascii_case("Expelled")
        || final_availability_status.eq_ignore_ascii_case("Injured");

    let left_due_to_incident = reason_is_incident || status_is_incident;

    let end_sec = match exit_instant {
        Some(exit_sec) => exit_sec,
        None => total_match_duration_seconds,
    };

    let seconds_played = (end_sec - entry_sec).max(0.0);
    let minutes_played = if seconds_played <= 0.0 {
        0
    } else {
        (seconds_played / 60.0).ceil() as u32
    };

    PlayerMinutesPlayedResult {
        minutes_played,
        seconds_played,
        entry_instant_seconds: Some(entry_sec),
        exit_instant_seconds: exit_instant,
        left_due_to_incident,
    }
}

pub fn calculate_player_minutes_played(
    was_starter: bool,
    player_id: &str,
    substitutions: &[MatchSubstitutionRow],
    final_availability_status: &str,
    final_suspended_remaining_seconds: Option<f64>,
    clock_config: &MatchClockDurationConfig,
    total_match_duration_seconds: f64,
) -> PlayerMinutesPlayedResult {
    let sub_in = substitutions.iter().find(|s| s.player_in_id == player_id);
    let sub_out = substitutions.iter().find(|s| s.player_out_id == player_id);

    calculate_minutes_played_with_player_subs(
        was_starter,
        sub_in,
        sub_out,
        final_availability_status,
        final_suspended_remaining_seconds,
        clock_config,
        total_match_duration_seconds,
    )
}
