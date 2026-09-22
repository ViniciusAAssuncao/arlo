use crate::attributes::{PlayerAttributeTable, RefereeAttributeTable};
use crate::resolution::DuelContext;
use arlo_domain::PitchZone;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct LineFaultEvaluationContext<'a> {
    pub receiver_id: Uuid,
    pub receiver_team_id: Uuid,
    pub defender_id: Uuid,
    pub defender_team_id: Uuid,
    pub receiver_table: &'a PlayerAttributeTable,
    pub defender_table: &'a PlayerAttributeTable,
    pub head_referee_table: &'a RefereeAttributeTable,
    pub peace_referee_table: &'a RefereeAttributeTable,
    pub duel_context: &'a DuelContext,
    pub zone: PitchZone,
    pub normalized_proximity: f64,
    pub defensive_line_height: f64,
}

impl<'a> LineFaultEvaluationContext<'a> {
    pub fn new(
        receiver_id: Uuid,
        receiver_team_id: Uuid,
        defender_id: Uuid,
        defender_team_id: Uuid,
        receiver_table: &'a PlayerAttributeTable,
        defender_table: &'a PlayerAttributeTable,
        head_referee_table: &'a RefereeAttributeTable,
        peace_referee_table: &'a RefereeAttributeTable,
        duel_context: &'a DuelContext,
        zone: PitchZone,
        normalized_proximity: f64,
        defensive_line_height: f64,
    ) -> Self {
        Self {
            receiver_id,
            receiver_team_id,
            defender_id,
            defender_team_id,
            receiver_table,
            defender_table,
            head_referee_table,
            peace_referee_table,
            duel_context,
            zone,
            normalized_proximity,
            defensive_line_height,
        }
    }
}