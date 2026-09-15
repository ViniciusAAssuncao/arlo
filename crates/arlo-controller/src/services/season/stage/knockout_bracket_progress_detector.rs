use crate::domain::season::{BracketSeed, Fixture, KnockoutTie};
use crate::services::season::stage::knockout_round_resolver::resolve_tie_winner;
use arlo_domain::TieBreakCriterion;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnockoutBracketProgress {
    RoundIncomplete {
        round_index: u32,
    },
    RoundCompleteNeedsNextRound {
        completed_round_index: u32,
        winner_seeds: Vec<BracketSeed>,
    },
    BracketComplete {
        champion_team_id: Uuid,
    },
}

pub fn detect_knockout_bracket_progress(
    ties: &[KnockoutTie],
    fixtures: &[Fixture],
    criteria: &[TieBreakCriterion],
) -> KnockoutBracketProgress {
    if ties.is_empty() {
        return KnockoutBracketProgress::RoundIncomplete { round_index: 0 };
    }

    let max_round_index = ties.iter().map(|t| t.round_index()).max().unwrap_or(0);
    let mut current_round_ties: Vec<&KnockoutTie> = ties
        .iter()
        .filter(|t| t.round_index() == max_round_index)
        .collect();

    current_round_ties.sort_by_key(|t| t.tie_index());

    let mut winners = Vec::with_capacity(current_round_ties.len());

    for tie in &current_round_ties {
        let winner_id = match resolve_tie_winner(tie, fixtures, criteria) {
            Some(w) => w,
            None => {
                return KnockoutBracketProgress::RoundIncomplete {
                    round_index: max_round_index,
                };
            }
        };

        let original_seed_number = if winner_id == tie.high_seed().team_id() {
            tie.high_seed().seed_number()
        } else if winner_id == tie.low_seed().team_id() {
            tie.low_seed().seed_number()
        } else {
            tie.high_seed().seed_number()
        };

        winners.push((original_seed_number, winner_id));
    }

    if winners.len() == 1 {
        KnockoutBracketProgress::BracketComplete {
            champion_team_id: winners[0].1,
        }
    } else {
        winners.sort_by_key(|(seed_num, _)| *seed_num);
        let winner_seeds: Vec<BracketSeed> = winners
            .into_iter()
            .enumerate()
            .map(|(idx, (_, team_id))| BracketSeed::new((idx + 1) as u32, team_id))
            .collect();

        KnockoutBracketProgress::RoundCompleteNeedsNextRound {
            completed_round_index: max_round_index,
            winner_seeds,
        }
    }
}
