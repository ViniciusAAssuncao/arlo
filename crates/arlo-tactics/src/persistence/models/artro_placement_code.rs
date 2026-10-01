use crate::error::{TacticsError, TacticsResult};
use arlo_domain::pitch::ArtroPlacement;

pub fn parse_artro_placement(code: &str) -> TacticsResult<ArtroPlacement> {
    match code {
        "LeftLateral" | "left_lateral" => Ok(ArtroPlacement::LeftLateral),
        "Central" | "central" => Ok(ArtroPlacement::Central),
        "RightLateral" | "right_lateral" => Ok(ArtroPlacement::RightLateral),
        _ => Err(TacticsError::InvalidEnum(format!(
            "Invalid artro placement: {code}"
        ))),
    }
}

pub fn artro_placement_to_code(placement: ArtroPlacement) -> &'static str {
    match placement {
        ArtroPlacement::LeftLateral => "LeftLateral",
        ArtroPlacement::Central => "Central",
        ArtroPlacement::RightLateral => "RightLateral",
    }
}
