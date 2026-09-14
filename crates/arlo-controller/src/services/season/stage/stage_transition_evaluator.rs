use crate::domain::season::{BracketSeed, StandingsEntry};
use crate::error::{ControllerError, ControllerResult};
use arlo_domain::StageEntryRule;

pub fn evaluate_stage_transition(
    standings: &[StandingsEntry],
    entry_rule: &StageEntryRule,
) -> ControllerResult<Vec<BracketSeed>> {
    if standings.is_empty() {
        return Err(ControllerError::Validation(
            "Standings must not be empty to evaluate stage transition".to_string(),
        ));
    }

    let qualified_teams: Vec<StandingsEntry> = match entry_rule {
        StageEntryRule::AllTeams => standings.to_vec(),
        StageEntryRule::TopN { count } => {
            let count = *count as usize;
            if standings.len() < count {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) to qualify top {}",
                    standings.len(),
                    count
                )));
            }
            standings[..count].to_vec()
        }
        StageEntryRule::BottomN { count } => {
            let count = *count as usize;
            if standings.len() < count {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) to qualify bottom {}",
                    standings.len(),
                    count
                )));
            }
            let start = standings.len() - count;
            standings[start..].to_vec()
        }
    };

    let seeds = qualified_teams
        .iter()
        .enumerate()
        .map(|(idx, entry)| BracketSeed::new((idx + 1) as u32, entry.team_id()))
        .collect();

    Ok(seeds)
}
