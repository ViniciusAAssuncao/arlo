use crate::error::{TacticsError, TacticsResult};
use arlo_domain::SlotRole;

pub fn parse_slot_role(code: &str) -> TacticsResult<SlotRole> {
    match code {
        "Standard" => Ok(SlotRole::Standard),
        "FalseArtrine" => Ok(SlotRole::FalseArtrine),
        "Launcher" => Ok(SlotRole::Launcher),
        "Safeguard" => Ok(SlotRole::Safeguard),
        "Blocker" => Ok(SlotRole::Blocker),
        "Kicker" => Ok(SlotRole::Kicker),
        _ => Err(TacticsError::InvalidEnum(format!(
            "Invalid slot role: {code}"
        ))),
    }
}

pub fn slot_role_to_code(role: SlotRole) -> &'static str {
    match role {
        SlotRole::Standard => "Standard",
        SlotRole::FalseArtrine => "FalseArtrine",
        SlotRole::Launcher => "Launcher",
        SlotRole::Safeguard => "Safeguard",
        SlotRole::Blocker => "Blocker",
        SlotRole::Kicker => "Kicker",
    }
}
