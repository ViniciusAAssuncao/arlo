use crate::error::DbResult;
use crate::models::position_code::parse_position;
use crate::models::slot_role_code::parse_slot_role;
use arlo_domain::FormationSlot;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct FormationSlotRow {
    pub id: String,
    pub formation_id: String,
    pub slot_index: i32,
    pub position: String,
    pub pitch_length_ratio: Option<f64>,
    pub pitch_width_ratio: Option<f64>,
    pub slot_role: String,
}

impl FormationSlotRow {
    pub fn to_domain(&self) -> DbResult<FormationSlot> {
        let position = parse_position(&self.position)?;
        let role = parse_slot_role(&self.slot_role)?;
        Ok(FormationSlot::new(position, role))
    }
}