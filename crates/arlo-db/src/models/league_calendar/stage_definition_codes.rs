use crate::error::{DbError, DbResult};
use arlo_domain::{KnockoutLegFormat, StageType};

pub fn parse_stage_type(code: &str) -> DbResult<StageType> {
    match code {
        "RoundRobinTable" | "round_robin_table" => Ok(StageType::RoundRobinTable),
        "KnockoutBracket" | "knockout_bracket" => Ok(StageType::KnockoutBracket),
        "GroupedCompetitionTable" | "grouped_competition_table" => {
            Ok(StageType::GroupedCompetitionTable)
        }
        _ => Err(DbError::InvalidEnum(format!("Invalid stage type: {code}"))),
    }
}

pub fn stage_type_to_code(stage_type: StageType) -> &'static str {
    match stage_type {
        StageType::RoundRobinTable => "RoundRobinTable",
        StageType::KnockoutBracket => "KnockoutBracket",
        StageType::GroupedCompetitionTable => "GroupedCompetitionTable",
    }
}

pub fn parse_knockout_leg_format(code: Option<&str>) -> DbResult<Option<KnockoutLegFormat>> {
    match code {
        Some("SingleLeg" | "single_leg") => Ok(Some(KnockoutLegFormat::SingleLeg)),
        Some("TwoLegAggregate" | "two_leg_aggregate") => {
            Ok(Some(KnockoutLegFormat::TwoLegAggregate))
        }
        Some(unknown) => Err(DbError::InvalidEnum(format!(
            "Invalid knockout leg format: {unknown}"
        ))),
        None => Ok(None),
    }
}

pub fn knockout_leg_format_to_code(format: Option<KnockoutLegFormat>) -> Option<&'static str> {
    match format {
        Some(KnockoutLegFormat::SingleLeg) => Some("SingleLeg"),
        Some(KnockoutLegFormat::TwoLegAggregate) => Some("TwoLegAggregate"),
        None => None,
    }
}
