use arlo_stats::TeamImpulseStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchTeamImpulseRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub average_baseline: f64,
    pub current_average_value: f64,
    pub min_average_value: f64,
    pub max_average_value: f64,
    pub average_value: f64,
    pub time_below_baseline_seconds: f64,
    pub runs_count: i32,
    pub longest_run_duration_seconds: f64,
    pub peak_run_average_value: f64,
    pub total_integrated_run_intensity: f64,
    pub average_run_duration_seconds: f64,
    pub average_run_intensity: f64,
}

impl MatchTeamImpulseRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        average_baseline: f64,
        current_average_value: f64,
        min_average_value: f64,
        max_average_value: f64,
        average_value: f64,
        time_below_baseline_seconds: f64,
        runs_count: u32,
        longest_run_duration_seconds: f64,
        peak_run_average_value: f64,
        total_integrated_run_intensity: f64,
        average_run_duration_seconds: f64,
        average_run_intensity: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            average_baseline,
            current_average_value,
            min_average_value,
            max_average_value,
            average_value,
            time_below_baseline_seconds,
            runs_count: runs_count as i32,
            longest_run_duration_seconds,
            peak_run_average_value,
            total_integrated_run_intensity,
            average_run_duration_seconds,
            average_run_intensity,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &TeamImpulseStats) -> Self {
        let runs_count = stats.runs_count();
        let total_intensity = stats.total_integrated_run_intensity();
        let average_run_intensity = if runs_count == 0 {
            0.0
        } else {
            total_intensity / (runs_count as f64)
        };

        Self::new(
            id,
            match_id,
            stats.team_id,
            stats.average_baseline,
            stats.current_average_value,
            stats.min_average_value,
            stats.max_average_value,
            stats.average_value(),
            stats.time_below_baseline_seconds,
            runs_count,
            stats.longest_run_duration_seconds(),
            stats.peak_run_average_value(),
            total_intensity,
            stats.average_run_duration_seconds(),
            average_run_intensity,
        )
    }
}