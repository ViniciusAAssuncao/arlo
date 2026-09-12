use crate::error::{DbError, DbResult};
use arlo_domain::BodyRegion;

pub fn parse_body_region(code: &str) -> DbResult<BodyRegion> {
    match code {
        "Head" | "head" => Ok(BodyRegion::Head),
        "Neck" | "neck" => Ok(BodyRegion::Neck),
        "Shoulder" | "shoulder" => Ok(BodyRegion::Shoulder),
        "Arm" | "arm" => Ok(BodyRegion::Arm),
        "Hand" | "hand" => Ok(BodyRegion::Hand),
        "Trunk" | "trunk" => Ok(BodyRegion::Trunk),
        "Hip" | "hip" => Ok(BodyRegion::Hip),
        "Groin" | "groin" => Ok(BodyRegion::Groin),
        "Thigh" | "thigh" => Ok(BodyRegion::Thigh),
        "Knee" | "knee" => Ok(BodyRegion::Knee),
        "Calf" | "calf" => Ok(BodyRegion::Calf),
        "Ankle" | "ankle" => Ok(BodyRegion::Ankle),
        "Foot" | "foot" => Ok(BodyRegion::Foot),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid body region: {code}"
        ))),
    }
}

pub fn body_region_to_code(region: BodyRegion) -> &'static str {
    match region {
        BodyRegion::Head => "Head",
        BodyRegion::Neck => "Neck",
        BodyRegion::Shoulder => "Shoulder",
        BodyRegion::Arm => "Arm",
        BodyRegion::Hand => "Hand",
        BodyRegion::Trunk => "Trunk",
        BodyRegion::Hip => "Hip",
        BodyRegion::Groin => "Groin",
        BodyRegion::Thigh => "Thigh",
        BodyRegion::Knee => "Knee",
        BodyRegion::Calf => "Calf",
        BodyRegion::Ankle => "Ankle",
        BodyRegion::Foot => "Foot",
    }
}