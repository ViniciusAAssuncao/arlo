use crate::error::{TacticsError, TacticsResult};
use crate::persistence::models::artro_placement_code::parse_artro_placement;
use crate::persistence::models::slot_role_code::parse_slot_role;
use crate::playcall::axes::ReadPriority;
use crate::playcall::route::RouteAssignment;
use arlo_domain::SlotRole;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PlayCallRouteAssignmentRow {
    pub id: String,
    pub play_call_id: String,
    pub slot_index: i32,
    pub has_route: i64,
    pub target_channel: Option<String>,
    pub depth_ratio: Option<f64>,
    pub break_ratio: Option<f64>,
    pub read_priority: Option<f64>,
    pub role_override: Option<String>,
}

impl PlayCallRouteAssignmentRow {
    pub fn to_domain(&self) -> TacticsResult<(Option<RouteAssignment>, Option<(usize, SlotRole)>)> {
        let slot_idx = self.slot_index as usize;
        let role_override = match &self.role_override {
            Some(r) => {
                let role = parse_slot_role(r)?;
                Some((slot_idx, role))
            }
            None => None,
        };

        if self.has_route == 1 {
            match (
                &self.target_channel,
                self.depth_ratio,
                self.break_ratio,
                self.read_priority,
            ) {
                (Some(tc), Some(dr), Some(br), Some(rp)) => {
                    let placement = parse_artro_placement(tc)?;
                    let read_priority = ReadPriority::new_clamped(rp);
                    let assignment = RouteAssignment::new(
                        slot_idx,
                        placement,
                        dr,
                        br,
                        read_priority,
                    )?;
                    Ok((Some(assignment), role_override))
                }
                _ => Err(TacticsError::InvalidPlayCall(format!(
                    "Route row with has_route=1 must have all route fields present for slot {slot_idx}"
                ))),
            }
        } else if self.has_route == 0 {
            if self.target_channel.is_some()
                || self.depth_ratio.is_some()
                || self.break_ratio.is_some()
                || self.read_priority.is_some()
            {
                return Err(TacticsError::InvalidPlayCall(format!(
                    "Route row with has_route=0 cannot have route fields populated for slot {slot_idx}"
                )));
            }
            if role_override.is_none() {
                return Err(TacticsError::InvalidPlayCall(format!(
                    "Route row with has_route=0 must have a role_override for slot {slot_idx}"
                )));
            }
            Ok((None, role_override))
        } else {
            Err(TacticsError::InvalidPlayCall(format!(
                "Invalid has_route flag value: {}",
                self.has_route
            )))
        }
    }
}
