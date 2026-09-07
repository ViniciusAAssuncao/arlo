use crate::error::{TacticsError, TacticsResult};
use crate::instructions::player::MarkingAssignment;
use arlo_db::models::position_code::{parse_position, position_to_code};

pub fn parse_marking_assignment(
    scheme: Option<&str>,
    target_position: Option<&str>,
) -> TacticsResult<Option<MarkingAssignment>> {
    match scheme {
        None => Ok(None),
        Some("Zonal") => Ok(Some(MarkingAssignment::Zonal)),
        Some("Man") => {
            let target = target_position.ok_or_else(|| {
                TacticsError::InvalidLineup("Man marking requires target position".to_string())
            })?;
            let position = parse_position(target)?;
            Ok(Some(MarkingAssignment::Man(position)))
        }
        Some(other) => Err(TacticsError::InvalidEnum(format!(
            "Invalid marking scheme: {other}"
        ))),
    }
}

pub fn marking_assignment_to_columns(
    marking: Option<MarkingAssignment>,
) -> (Option<String>, Option<String>) {
    match marking {
        None => (None, None),
        Some(MarkingAssignment::Zonal) => (Some("Zonal".to_string()), None),
        Some(MarkingAssignment::Man(position)) => (
            Some("Man".to_string()),
            Some(position_to_code(position).to_string()),
        ),
    }
}