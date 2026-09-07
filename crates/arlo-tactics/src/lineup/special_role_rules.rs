use arlo_domain::{Position, SlotRole};

pub fn max_concurrent_count(role: SlotRole) -> Option<u32> {
    match role {
        SlotRole::FalseArtrine => Some(1),
        SlotRole::Launcher => Some(1),
        SlotRole::Safeguard => Some(1),
        SlotRole::Kicker => Some(1),
        SlotRole::Blocker => None,
        SlotRole::Standard => None,
    }
}

pub fn is_role_eligible_for_position(role: SlotRole, position: Position) -> bool {
    match role {
        SlotRole::FalseArtrine => {
            position != Position::Artrine && position != Position::Passer
        }
        _ => true,
    }
}
