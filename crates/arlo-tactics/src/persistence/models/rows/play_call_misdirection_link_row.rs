use crate::error::TacticsResult;
use crate::playcall::misdirection::MisdirectionLink;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PlayCallMisdirectionLinkRow {
    pub id: String,
    pub play_call_id: String,
    pub decoy_slot_index: i32,
    pub true_carrier_slot_index: i32,
    pub deception_intensity: f64,
}

impl PlayCallMisdirectionLinkRow {
    pub fn to_domain(&self) -> TacticsResult<MisdirectionLink> {
        Ok(MisdirectionLink::new_clamped(
            self.decoy_slot_index as usize,
            self.true_carrier_slot_index as usize,
            self.deception_intensity,
        ))
    }
}
