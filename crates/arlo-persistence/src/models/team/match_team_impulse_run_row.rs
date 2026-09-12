use arlo_stats::TeamImpulseRun;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchTeamImpulseRunRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub run_index: i32,
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub duration_seconds: f64,
    pub peak_average_value: f64,
    pub integrated_intensity: f64,
    pub average_intensity: f64,
}

impl MatchTeamImpulseRunRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        run_index: usize,
        start_time_seconds: f64,
        end_time_seconds: f64,
        duration_seconds: f64,
        peak_average_value: f64,
        integrated_intensity: f64,
        average_intensity: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            run_index: run_index as i32,
            start_time_seconds,
            end_time_seconds,
            duration_seconds,
            peak_average_value,
            integrated_intensity,
            average_intensity,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        run_index: usize,
        run: &TeamImpulseRun,
    ) -> Self {
        Self::new(
            id,
            match_id,
            team_id,
            run_index,
            run.start_time_seconds,
            run.end_time_seconds,
            run.duration_seconds,
            run.peak_average_value,
            run.integrated_intensity,
            run.average_intensity(),
        )
    }
}