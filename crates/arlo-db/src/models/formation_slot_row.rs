use crate::error::DbResult;
use crate::models::position_code::parse_position;
use arlo_domain::FormationSlot;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct FormationSlotRow {
    pub id: String,
    pub formation_id: String,
    pub slot_index: i32,
    pub position: String,
    pub pitch_length_ratio: f64,
    pub pitch_width_ratio: f64,
    pub slot_role: String,
}

impl FormationSlotRow {
    pub fn to_domain(&self) -> DbResult<FormationSlot> {
        let position = parse_position(&self.position)?;
        FormationSlot::new(position, self.pitch_length_ratio, self.pitch_width_ratio)
            .map_err(Into::into)
    }
}
