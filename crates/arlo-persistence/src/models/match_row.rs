use crate::models::match_seed_codec::encode_match_seed;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchRow {
    pub id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub venue_id: Option<String>,
    pub pitch_length_mirim: f64,
    pub pitch_width_mirim: f64,
    pub match_seed: i64,
    pub format_regulation_periods: i32,
    pub format_regulation_period_duration_seconds: i32,
    pub format_allows_overtime: bool,
    pub format_overtime_periods: i32,
    pub format_overtime_period_duration_seconds: i32,
    pub head_referee_id: String,
    pub peace_referee_id: String,
    pub final_period: i32,
    pub went_to_overtime: bool,
    pub completed_at_unix_seconds: i64,
    pub created_at_unix_seconds: i64,
}

impl MatchRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        home_team_id: Uuid,
        away_team_id: Uuid,
        venue_id: Option<Uuid>,
        pitch_length_mirim: f64,
        pitch_width_mirim: f64,
        match_seed: u64,
        format_regulation_periods: u32,
        format_regulation_period_duration_seconds: u32,
        format_allows_overtime: bool,
        format_overtime_periods: u32,
        format_overtime_period_duration_seconds: u32,
        head_referee_id: Uuid,
        peace_referee_id: Uuid,
        final_period: u32,
        went_to_overtime: bool,
        completed_at_unix_seconds: i64,
        created_at_unix_seconds: i64,
    ) -> Self {
        Self {
            id: id.to_string(),
            home_team_id: home_team_id.to_string(),
            away_team_id: away_team_id.to_string(),
            venue_id: venue_id.map(|v| v.to_string()),
            pitch_length_mirim,
            pitch_width_mirim,
            match_seed: encode_match_seed(match_seed),
            format_regulation_periods: format_regulation_periods as i32,
            format_regulation_period_duration_seconds:
                format_regulation_period_duration_seconds as i32,
            format_allows_overtime,
            format_overtime_periods: format_overtime_periods as i32,
            format_overtime_period_duration_seconds:
                format_overtime_period_duration_seconds as i32,
            head_referee_id: head_referee_id.to_string(),
            peace_referee_id: peace_referee_id.to_string(),
            final_period: final_period as i32,
            went_to_overtime,
            completed_at_unix_seconds,
            created_at_unix_seconds,
        }
    }
}
