use crate::domain::calendar::IntercalationPlacement;
use crate::error::{ControllerError, ControllerResult};

pub fn parse_intercalation_placement(
    kind: &str,
    month_order_index: Option<i32>,
) -> ControllerResult<IntercalationPlacement> {
    match kind {
        "BeforeFirstMonth" | "before_first_month" => Ok(IntercalationPlacement::BeforeFirstMonth),
        "AfterLastMonth" | "after_last_month" => Ok(IntercalationPlacement::AfterLastMonth),
        "AppendToMonth" | "append_to_month" => {
            let month_order_index = month_order_index.ok_or_else(|| {
                ControllerError::InvalidData(
                    "AppendToMonth requires month_order_index to be set".to_string(),
                )
            })?;
            if month_order_index < 0 {
                return Err(ControllerError::InvalidData(
                    "month_order_index must be non-negative".to_string(),
                ));
            }
            Ok(IntercalationPlacement::AppendToMonth {
                month_order_index: month_order_index as u32,
            })
        }
        _ => Err(ControllerError::InvalidEnum(format!(
            "Invalid intercalation placement kind: {kind}"
        ))),
    }
}

pub fn intercalation_placement_to_code(
    placement: &IntercalationPlacement,
) -> (&'static str, Option<i32>) {
    match placement {
        IntercalationPlacement::BeforeFirstMonth => ("BeforeFirstMonth", None),
        IntercalationPlacement::AfterLastMonth => ("AfterLastMonth", None),
        IntercalationPlacement::AppendToMonth { month_order_index } => {
            ("AppendToMonth", Some(*month_order_index as i32))
        }
    }
}
