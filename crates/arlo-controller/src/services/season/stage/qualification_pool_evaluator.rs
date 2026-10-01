use crate::domain::season::{GroupRankedStandingsEntry, StandingsEntry};
use crate::error::{ControllerError, ControllerResult};
use arlo_domain::QualificationPoolRule;
use std::collections::HashMap;
use uuid::Uuid;

pub fn evaluate_pool(
    pool: &QualificationPoolRule,
    annotated: &[GroupRankedStandingsEntry],
    global_sorted: &[StandingsEntry],
    external_winners: &HashMap<Uuid, Uuid>,
) -> ControllerResult<Vec<Uuid>> {
    match pool {
        QualificationPoolRule::AllTeams => Ok(global_sorted.iter().map(|e| e.team_id()).collect()),
        QualificationPoolRule::TopN { count } => {
            let count = *count as usize;
            if global_sorted.len() < count {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) to qualify top {}",
                    global_sorted.len(),
                    count
                )));
            }
            Ok(global_sorted[..count].iter().map(|e| e.team_id()).collect())
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
            Ok(global_sorted[start..].iter().map(|e| e.team_id()).collect())
        }
        QualificationPoolRule::GroupWinners => {
            let winners = annotated
                .iter()
                .filter(|entry| entry.rank_in_group() == 0)
                .map(|entry| entry.standings_entry().team_id())
                .collect();
            Ok(winners)
        }
        QualificationPoolRule::GroupRunnersUp => {
            let runners_up = annotated
                .iter()
                .filter(|entry| entry.rank_in_group() == 1)
                .map(|entry| entry.standings_entry().team_id())
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
                .map(|entry| entry.standings_entry().team_id())
                .collect();
            Ok(matches)
        }
        QualificationPoolRule::PositionRange {
            start_position,
            end_position,
        } => {
            let start = *start_position as usize;
            let end = *end_position as usize;
            if start == 0 || end < start {
                return Err(ControllerError::Validation(format!(
                    "Invalid position range: {}-{}",
                    start, end
                )));
            }
            if global_sorted.len() < end {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) to qualify position range {}-{}",
                    global_sorted.len(),
                    start,
                    end
                )));
            }
            Ok(global_sorted[(start - 1)..end]
                .iter()
                .map(|e| e.team_id())
                .collect())
        }
        QualificationPoolRule::ExternalCompetitionWinner { competition_id } => {
            let winner = external_winners
                .get(competition_id)
                .copied()
                .ok_or_else(|| {
                    ControllerError::Validation(format!(
                        "External winner for competition {} not found",
                        competition_id
                    ))
                })?;
            Ok(vec![winner])
        }
    }
}
