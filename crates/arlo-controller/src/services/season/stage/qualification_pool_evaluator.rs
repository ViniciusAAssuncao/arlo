use crate::domain::season::{GroupRankedStandingsEntry, StandingsEntry};
use crate::error::{ControllerError, ControllerResult};
use arlo_domain::QualificationPoolRule;

pub fn evaluate_pool(
    pool: &QualificationPoolRule,
    annotated: &[GroupRankedStandingsEntry],
    global_sorted: &[StandingsEntry],
) -> ControllerResult<Vec<StandingsEntry>> {
    match pool {
        QualificationPoolRule::AllTeams => Ok(global_sorted.to_vec()),
        QualificationPoolRule::TopN { count } => {
            let count = *count as usize;
            if global_sorted.len() < count {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) to qualify top {}",
                    global_sorted.len(),
                    count
                )));
            }
            Ok(global_sorted[..count].to_vec())
        }
        QualificationPoolRule::BottomN { count } => {
            let count = *count as usize;
            if global_sorted.len() < count {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) to qualify bottom {}",
                    global_sorted.len(),
                    count
                )));
            }
            let start = global_sorted.len() - count;
            Ok(global_sorted[start..].to_vec())
        }
        QualificationPoolRule::GroupWinners => {
            let winners = annotated
                .iter()
                .filter(|entry| entry.rank_in_group() == 0)
                .map(|entry| entry.standings_entry())
                .collect();
            Ok(winners)
        }
        QualificationPoolRule::GroupRunnersUp => {
            let runners_up = annotated
                .iter()
                .filter(|entry| entry.rank_in_group() == 1)
                .map(|entry| entry.standings_entry())
                .collect();
            Ok(runners_up)
        }
        QualificationPoolRule::BestAtGroupPosition {
            position_index,
            count,
        } => {
            let matches = annotated
                .iter()
                .filter(|entry| entry.rank_in_group() == *position_index)
                .take(*count as usize)
                .map(|entry| entry.standings_entry())
                .collect();
            Ok(matches)
        }
    }
}