use crate::lineup_runtime::lineup::Lineup;
use crate::lineup_runtime::position_similarity::position_similarity;
use arlo_domain::{FormationSlot, Player, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PositionalFit {
    effective_proficiency: f64,
    efficiency_multiplier: f64,
    is_exact_position: bool,
}

impl PositionalFit {
    pub fn new(
        effective_proficiency: f64,
        efficiency_multiplier: f64,
        is_exact_position: bool,
    ) -> Self {
        Self {
            effective_proficiency,
            efficiency_multiplier,
            is_exact_position,
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

    pub fn is_natural(&self) -> bool {
        self.is_exact_position && self.effective_proficiency >= 8.0
    }
}

fn evaluate_position_match(pos: Position, nominal_prof: i32, target_pos: Position) -> (f64, bool) {
    let prof = nominal_prof as f64;
    if pos == target_pos {
        (prof, true)
    } else {
        let sim = position_similarity(pos, target_pos);
        (prof * sim, false)
    }
}

pub fn calculate_fit_for_position(player: &Player, target_position: Position) -> PositionalFit {
    if player.positions().is_empty() {
        return PositionalFit::new(0.0, 0.0, false);
    }

    let mut best_score = -1.0;
    let mut best_exact = false;

    for pp in player.positions() {
        let (score, exact) =
            evaluate_position_match(pp.position(), pp.proficiency(), target_position);
        if score > best_score || (score == best_score && exact) {
            best_score = score;
            best_exact = exact;
        }
    }

    let effective_proficiency = best_score.max(0.0);
    let efficiency_multiplier = (effective_proficiency / 10.0).clamp(0.0, 1.0);

    PositionalFit::new(effective_proficiency, efficiency_multiplier, best_exact)
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
