use crate::error::{DbError, DbResult};
use arlo_domain::SlotRole;

pub fn parse_slot_role(code: &str) -> DbResult<SlotRole> {
    match code {
        "Standard" | "standard" => Ok(SlotRole::Standard),
        "FalseArtrine" | "false_artrine" => Ok(SlotRole::FalseArtrine),
        "Launcher" | "launcher" => Ok(SlotRole::Launcher),
        "Safeguard" | "safeguard" => Ok(SlotRole::Safeguard),
        "Blocker" | "blocker" => Ok(SlotRole::Blocker),
        "Kicker" | "kicker" => Ok(SlotRole::Kicker),
        _ => Err(DbError::InvalidEnum(format!(
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