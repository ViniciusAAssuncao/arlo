use crate::error::{DbError, DbResult};
use arlo_domain::{ArtrineDependency, DefensiveApproach, OffensiveApproach, RotationPolicy};

pub fn parse_offensive_approach(code: &str) -> DbResult<OffensiveApproach> {
    match code {
        "Positional" | "positional" => Ok(OffensiveApproach::Positional),
        "Functional" | "functional" => Ok(OffensiveApproach::Functional),
        "Direct" | "direct" => Ok(OffensiveApproach::Direct),
        "Balanced" | "balanced" => Ok(OffensiveApproach::Balanced),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid offensive approach: {code}"
        ))),
    }
}

pub fn offensive_approach_to_code(approach: OffensiveApproach) -> &'static str {
    match approach {
        OffensiveApproach::Positional => "Positional",
        OffensiveApproach::Functional => "Functional",
        OffensiveApproach::Direct => "Direct",
        OffensiveApproach::Balanced => "Balanced",
    }
}

pub fn parse_defensive_approach(code: &str) -> DbResult<DefensiveApproach> {
    match code {
        "HighPress" | "high_press" => Ok(DefensiveApproach::HighPress),
        "MidBlock" | "mid_block" => Ok(DefensiveApproach::MidBlock),
        "DeepLowBlock" | "deep_low_block" => Ok(DefensiveApproach::DeepLowBlock),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid defensive approach: {code}"
        ))),
    }
}

pub fn defensive_approach_to_code(approach: DefensiveApproach) -> &'static str {
    match approach {
        DefensiveApproach::HighPress => "HighPress",
        DefensiveApproach::MidBlock => "MidBlock",
        DefensiveApproach::DeepLowBlock => "DeepLowBlock",
    }
}

pub fn parse_rotation_policy(code: &str) -> DbResult<RotationPolicy> {
    match code {
        "StrictCore" | "strict_core" => Ok(RotationPolicy::StrictCore),
        "Situational" | "situational" => Ok(RotationPolicy::Situational),
        "HighRotation" | "high_rotation" => Ok(RotationPolicy::HighRotation),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid rotation policy: {code}"
        ))),
    }
}

pub fn rotation_policy_to_code(policy: RotationPolicy) -> &'static str {
    match policy {
        RotationPolicy::StrictCore => "StrictCore",
        RotationPolicy::Situational => "Situational",
        RotationPolicy::HighRotation => "HighRotation",
    }
}

pub fn parse_artrine_dependency(code: &str) -> DbResult<ArtrineDependency> {
    match code {
        "SystemDriven" | "system_driven" => Ok(ArtrineDependency::SystemDriven),
        "ArtrineCentric" | "artrine_centric" => Ok(ArtrineDependency::ArtrineCentric),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid artrine dependency: {code}"
        ))),
    }
}

pub fn artrine_dependency_to_code(dependency: ArtrineDependency) -> &'static str {
    match dependency {
        ArtrineDependency::SystemDriven => "SystemDriven",
        ArtrineDependency::ArtrineCentric => "ArtrineCentric",
    }
}
