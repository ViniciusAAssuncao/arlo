use crate::error::{DbError, DbResult};
use arlo_domain::{KnockoutLegFormat, StageEntryRule, StageType};

pub fn parse_stage_type(code: &str) -> DbResult<StageType> {
    match code {
        "RoundRobinTable" | "round_robin_table" => Ok(StageType::RoundRobinTable),
        "KnockoutBracket" | "knockout_bracket" => Ok(StageType::KnockoutBracket),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid stage type: {code}"
        ))),
    }
}

pub fn stage_type_to_code(stage_type: StageType) -> &'static str {
    match stage_type {
        StageType::RoundRobinTable => "RoundRobinTable",
        StageType::KnockoutBracket => "KnockoutBracket",
    }
}

pub fn parse_stage_entry_rule(kind_code: &str, count: Option<i32>) -> DbResult<StageEntryRule> {
    match kind_code {
        "AllTeams" | "all_teams" => Ok(StageEntryRule::AllTeams),
        "TopN" | "top_n" => {
            let count = count.ok_or_else(|| {
                DbError::InvalidData("TopN stage entry rule requires count value".to_string())
            })?;
            Ok(StageEntryRule::TopN {
                count: count as u32,
            })
        }
        "BottomN" | "bottom_n" => {
            let count = count.ok_or_else(|| {
                DbError::InvalidData("BottomN stage entry rule requires count value".to_string())
            })?;
            Ok(StageEntryRule::BottomN {
                count: count as u32,
            })
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid stage entry rule kind: {kind_code}"
        ))),
    }
}

pub fn stage_entry_rule_to_codes(rule: StageEntryRule) -> (&'static str, Option<i32>) {
    match rule {
        StageEntryRule::AllTeams => ("AllTeams", None),
        StageEntryRule::TopN { count } => ("TopN", Some(count as i32)),
        StageEntryRule::BottomN { count } => ("BottomN", Some(count as i32)),
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
