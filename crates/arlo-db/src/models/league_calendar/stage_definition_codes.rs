
use crate::error::{DbError, DbResult};
use arlo_domain::{KnockoutLegFormat, QualificationPoolRule, StageEntryRule, StageType};

pub fn parse_stage_type(code: &str) -> DbResult<StageType> {
    match code {
        "RoundRobinTable" | "round_robin_table" => Ok(StageType::RoundRobinTable),
        "KnockoutBracket" | "knockout_bracket" => Ok(StageType::KnockoutBracket),
        "GroupedCompetitionTable" | "grouped_competition_table" => {
            Ok(StageType::GroupedCompetitionTable)
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid stage type: {code}"
        ))),
    }
}

pub fn stage_type_to_code(stage_type: StageType) -> &'static str {
    match stage_type {
        StageType::RoundRobinTable => "RoundRobinTable",
        StageType::KnockoutBracket => "KnockoutBracket",
        StageType::GroupedCompetitionTable => "GroupedCompetitionTable",
    }
}

pub fn parse_stage_entry_rule(kind_code: &str, count: Option<i32>) -> DbResult<StageEntryRule> {
    match kind_code {
        "AllTeams" | "all_teams" => Ok(StageEntryRule::new(vec![QualificationPoolRule::AllTeams])?),
        "TopN" | "top_n" => {
            let count = count.ok_or_else(|| {
                DbError::InvalidData("TopN stage entry rule requires count value".to_string())
            })?;
            Ok(StageEntryRule::new(vec![QualificationPoolRule::TopN {
                count: count as u32,
            }])?)
        }
        "BottomN" | "bottom_n" => {
            let count = count.ok_or_else(|| {
                DbError::InvalidData("BottomN stage entry rule requires count value".to_string())
            })?;
            Ok(StageEntryRule::new(vec![QualificationPoolRule::BottomN {
                count: count as u32,
            }])?)
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid stage entry rule kind: {kind_code}"
        ))),
    }
}

pub fn stage_entry_rule_to_codes(rule: &StageEntryRule) -> (&'static str, Option<i32>) {
    match rule.pools().first() {
        Some(QualificationPoolRule::AllTeams) => ("AllTeams", None),
        Some(QualificationPoolRule::TopN { count }) => ("TopN", Some(*count as i32)),
        Some(QualificationPoolRule::BottomN { count }) => ("BottomN", Some(*count as i32)),
        _ => ("AllTeams", None),
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