use arlo_stats::PlayerImpulseStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerImpulseRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub baseline: f64,
    pub current_value: i32,
    pub initial_value: i32,
    pub min_value: i32,
    pub max_value: i32,
    pub average_value: f64,
    pub shifts_count: i32,
    pub positive_shifts: i32,
    pub negative_shifts: i32,
    pub time_below_baseline_seconds: f64,
    pub critical_reached_count: i32,
    pub runs_count: i32,
    pub longest_run_duration_seconds: f64,
    pub peak_run_value: i32,
    pub total_integrated_run_intensity: f64,
    pub average_run_duration_seconds: f64,
    pub average_run_intensity: f64,
}

impl MatchPlayerImpulseRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        baseline: f64,
        current_value: u8,
        initial_value: u8,
        min_value: u8,
        max_value: u8,
        average_value: f64,
        shifts_count: u32,
        positive_shifts: u32,
        negative_shifts: u32,
        time_below_baseline_seconds: f64,
        critical_reached_count: u32,
        runs_count: u32,
        longest_run_duration_seconds: f64,
        peak_run_value: u8,
        total_integrated_run_intensity: f64,
        average_run_duration_seconds: f64,
        average_run_intensity: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            baseline,
            current_value: current_value as i32,
            initial_value: initial_value as i32,
            min_value: min_value as i32,
            max_value: max_value as i32,
            average_value,
            shifts_count: shifts_count as i32,
            positive_shifts: positive_shifts as i32,
            negative_shifts: negative_shifts as i32,
            time_below_baseline_seconds,
            critical_reached_count: critical_reached_count as i32,
            runs_count: runs_count as i32,
            longest_run_duration_seconds,
            peak_run_value: peak_run_value as i32,
            total_integrated_run_intensity,
            average_run_duration_seconds,
            average_run_intensity,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerImpulseStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.baseline,
            stats.current_value,
            stats.initial_value,
            stats.min_value,
            stats.max_value,
            stats.average_value(),
            stats.shifts_count,
            stats.positive_shifts,
            stats.negative_shifts,
            stats.time_below_baseline_seconds,
            stats.critical_reached_count,
            stats.runs_count(),
            stats.longest_run_duration_seconds(),
            stats.peak_run_value(),
            stats.total_integrated_run_intensity(),
            stats.average_run_duration_seconds(),
            stats.average_run_intensity(),
        )
    }
}
