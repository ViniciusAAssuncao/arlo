use crate::domain::league_calendar::qualification_pool_rule::QualificationPoolRule;
use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;

pub fn validate_qualification_pool_rule(rule: &QualificationPoolRule) -> DomainResult<()> {
    match rule {
        QualificationPoolRule::TopN { count } => {
            validate_integer_range(*count as i32, 1, i32::MAX, "entry_rule.count")?;
        }
        QualificationPoolRule::BottomN { count } => {
            validate_integer_range(*count as i32, 1, i32::MAX, "entry_rule.count")?;
        }
        QualificationPoolRule::BestAtGroupPosition { count, .. } => {
            validate_integer_range(*count as i32, 1, i32::MAX, "entry_rule.count")?;
        }
        QualificationPoolRule::PositionRange {
            start_position,
            end_position,
        } => {
            validate_integer_range(
                *start_position as i32,
                1,
                i32::MAX,
                "entry_rule.start_position",
            )?;
            validate_integer_range(
                *end_position as i32,
                *start_position as i32,
                i32::MAX,
                "entry_rule.end_position",
            )?;
        }
        QualificationPoolRule::AllTeams
        | QualificationPoolRule::GroupWinners
        | QualificationPoolRule::GroupRunnersUp => {}
    }
    Ok(())
}
