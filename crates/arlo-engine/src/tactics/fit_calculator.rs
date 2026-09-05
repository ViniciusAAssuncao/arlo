use crate::tactics::lineup::Lineup;
use arlo_domain::{FormationSlot, Player, Position, PositionLine};
use serde::{Deserialize, Serialize};

pub const SAME_LINE_PENALTY_FACTOR: f64 = 0.70;
pub const ADJACENT_LINE_PENALTY_FACTOR: f64 = 0.45;
pub const DISTANT_LINE_PENALTY_FACTOR: f64 = 0.20;
pub const GOALGUARD_MISMATCH_FACTOR: f64 = 0.10;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PositionalFit {
    effective_proficiency: f64,
    efficiency_multiplier: f64,
    is_exact_position: bool,
    is_same_line: bool,
}

impl PositionalFit {
    pub fn new(
        effective_proficiency: f64,
        efficiency_multiplier: f64,
        is_exact_position: bool,
        is_same_line: bool,
    ) -> Self {
        Self {
            effective_proficiency,
            efficiency_multiplier,
            is_exact_position,
            is_same_line,
        }
    }

    pub fn effective_proficiency(&self) -> f64 {
        self.effective_proficiency
    }

    pub fn efficiency_multiplier(&self) -> f64 {
        self.efficiency_multiplier
    }

    pub fn is_exact_position(&self) -> bool {
        self.is_exact_position
    }

    pub fn is_same_line(&self) -> bool {
        self.is_same_line
    }

    pub fn is_natural(&self) -> bool {
        self.is_exact_position && self.effective_proficiency >= 8.0
    }
}

fn line_index(line: PositionLine) -> i32 {
    match line {
        PositionLine::OffensiveLine => 0,
        PositionLine::BackLine => 1,
        PositionLine::DefenseLine => 2,
        PositionLine::Goalguard => 3,
    }
}

fn evaluate_position_match(
    pos: Position,
    nominal_prof: i32,
    target_pos: Position,
) -> (f64, bool, bool) {
    let prof = nominal_prof as f64;
    if pos == target_pos {
        (prof, true, true)
    } else if pos.line() == target_pos.line() {
        (prof * SAME_LINE_PENALTY_FACTOR, false, true)
    } else if pos.line() == PositionLine::Goalguard || target_pos.line() == PositionLine::Goalguard {
        (prof * GOALGUARD_MISMATCH_FACTOR, false, false)
    } else {
        let dist = (line_index(pos.line()) - line_index(target_pos.line())).abs();
        let factor = if dist == 1 {
            ADJACENT_LINE_PENALTY_FACTOR
        } else {
            DISTANT_LINE_PENALTY_FACTOR
        };
        (prof * factor, false, false)
    }
}

pub fn calculate_fit_for_position(player: &Player, target_position: Position) -> PositionalFit {
    if player.positions().is_empty() {
        return PositionalFit::new(0.0, 0.0, false, false);
    }

    let mut best_score = -1.0;
    let mut best_exact = false;
    let mut best_same_line = false;

    for pp in player.positions() {
        let (score, exact, same_line) =
            evaluate_position_match(pp.position(), pp.proficiency(), target_position);
        if score > best_score
            || (score == best_score && exact)
            || (score == best_score && !best_exact && same_line)
        {
            best_score = score;
            best_exact = exact;
            best_same_line = same_line;
        }
    }

    let effective_proficiency = best_score.max(0.0);
    let efficiency_multiplier = (effective_proficiency / 10.0).clamp(0.0, 1.0);

    PositionalFit::new(
        effective_proficiency,
        efficiency_multiplier,
        best_exact,
        best_same_line,
    )
}

pub fn calculate_fit(player: &Player, slot: &FormationSlot) -> PositionalFit {
    calculate_fit_for_position(player, slot.position())
}

pub fn calculate_lineup_fit(lineup: &Lineup) -> Vec<PositionalFit> {
    lineup
        .assignments()
        .iter()
        .map(|a| calculate_fit(a.player(), a.slot()))
        .collect()
}

pub fn calculate_average_fit(lineup: &Lineup) -> f64 {
    let fits = calculate_lineup_fit(lineup);
    if fits.is_empty() {
        return 0.0;
    }
    let total: f64 = fits.iter().map(|f| f.efficiency_multiplier()).sum();
    total / fits.len() as f64
}

pub struct SlotFitCalculator;

impl SlotFitCalculator {
    pub fn calculate(player: &Player, slot: &FormationSlot) -> PositionalFit {
        calculate_fit(player, slot)
    }

    pub fn calculate_for_position(player: &Player, target_position: Position) -> PositionalFit {
        calculate_fit_for_position(player, target_position)
    }

    pub fn calculate_lineup_fit(lineup: &Lineup) -> Vec<PositionalFit> {
        calculate_lineup_fit(lineup)
    }

    pub fn calculate_average_fit(lineup: &Lineup) -> f64 {
        calculate_average_fit(lineup)
    }
}