use crate::domain::calendar::CalendarDate;
use arlo_analytics::prediction::preseason::PreseasonProjection;
use arlo_persistence::repositories::season::preseason_projections::StoredPreseasonProjection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreseasonForecastDto {
    pub id: Uuid,
    pub calibration_id: Uuid,
    pub generated_as_of: CalendarDate,
    pub power_seed_model_version: u32,
    pub projection: PreseasonProjection,
}

impl From<StoredPreseasonProjection> for PreseasonForecastDto {
    fn from(stored: StoredPreseasonProjection) -> Self {
        Self {
            id: stored.id,
            calibration_id: stored.calibration_id,
            generated_as_of: CalendarDate::new(stored.generated_year, stored.generated_day_of_year),
            power_seed_model_version: stored.power_seed_model_version,
            projection: stored.projection,
        }
    }
}
