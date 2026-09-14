use crate::domain::season::{ BracketSeed, StandingsEntry };
use crate::error::{ ControllerError, ControllerResult };
use arlo_domain::{ QualificationPoolRule, StageEntryRule };

pub fn evaluate_stage_transition(
    standings: &[StandingsEntry],
    entry_rule: &StageEntryRule
) -> ControllerResult<Vec<BracketSeed>> {
    if standings.is_empty() {
        return Err(
            ControllerError::Validation(
                "Standings must not be empty to evaluate stage transition".to_string()
            )
        );
    }

    let mut qualified_teams: Vec<StandingsEntry> = Vec::new();

    for pool in entry_rule.pools() {
        match pool {
            QualificationPoolRule::AllTeams => {
                qualified_teams.extend_from_slice(standings);
            }
            QualificationPoolRule::TopN { count } => {
                let count = *count as usize;
                if standings.len() < count {
                    return Err(
                        ControllerError::Validation(
                            format!(
                                "Not enough teams in standings ({}) to qualify top {}",
                                standings.len(),
                                count
                            )
                        )
                    );
                }
                qualified_teams.extend_from_slice(&standings[..count]);
            }
            QualificationPoolRule::BottomN { count } => {
                let count = *count as usize;
                if standings.len() < count {
                    return Err(
                        ControllerError::Validation(
                            format!(
                                "Not enough teams in standings ({}) to qualify bottom {}",
                                standings.len(),
                                count
                            )
                        )
                    );
                }
                let start = standings.len() - count;
                qualified_teams.extend_from_slice(&standings[start..]);
            }
            | QualificationPoolRule::GroupWinners
            | QualificationPoolRule::GroupRunnersUp
            | QualificationPoolRule::BestAtGroupPosition { .. } => {}
        }
    }

    let seeds = qualified_teams
        .iter()
        .enumerate()
        .map(|(idx, entry)| BracketSeed::new((idx + 1) as u32, entry.team_id()))
        .collect();

    Ok(seeds)
}
