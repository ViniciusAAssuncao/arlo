use arlo_stats::ImpulseRun;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerImpulseRunRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub run_index: i32,
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub duration_seconds: f64,
    pub peak_value: i32,
    pub integrated_intensity: f64,
    pub shifts_count: i32,
    pub average_intensity: f64,
}

impl MatchPlayerImpulseRunRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        run_index: usize,
        start_time_seconds: f64,
        end_time_seconds: f64,
        duration_seconds: f64,
        peak_value: u8,
        integrated_intensity: f64,
        shifts_count: u32,
        average_intensity: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            run_index: run_index as i32,
            start_time_seconds,
            end_time_seconds,
            duration_seconds,
            peak_value: peak_value as i32,
            integrated_intensity,
            shifts_count: shifts_count as i32,
            average_intensity,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        run_index: usize,
        run: &ImpulseRun,
    ) -> Self {
        Self::new(
            id,
            match_id,
            player_id,
            run_index,
            run.start_time_seconds,
            run.end_time_seconds,
            run.duration_seconds,
            run.peak_value,
            run.integrated_intensity,
            run.shifts_count,
            run.average_intensity(),
        )
    }
}
