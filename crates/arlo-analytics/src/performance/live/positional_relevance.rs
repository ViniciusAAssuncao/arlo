use crate::performance::observation::{ObservationCategory, PossessionPhase};
use arlo_domain::{Position, PositionLine};

pub(crate) fn relevance_for(
    offensive_position: Position,
    defensive_position: Position,
    category: ObservationCategory,
    phase: PossessionPhase,
) -> f64 {
    match phase {
        PossessionPhase::Offense => {
            relevance_for_position(offensive_position, category, PossessionPhase::Offense)
        }
        PossessionPhase::Defense => {
            relevance_for_position(defensive_position, category, PossessionPhase::Defense)
        }
        PossessionPhase::Neutral => {
            let offense =
                relevance_for_position(offensive_position, category, PossessionPhase::Offense);
            let defense =
                relevance_for_position(defensive_position, category, PossessionPhase::Defense);
            (offense + defense) * 0.5
        }
    }
}

fn relevance_for_position(
    position: Position,
    category: ObservationCategory,
    phase: PossessionPhase,
) -> f64 {
    let base = match (position.line(), phase) {
        (PositionLine::OffensiveLine, PossessionPhase::Offense) => {
            offensive_line_offense(category)
        }
        (PositionLine::OffensiveLine, _) => offensive_line_defense(category),
        (PositionLine::BackLine, PossessionPhase::Offense) => back_line_offense(category),
        (PositionLine::BackLine, _) => back_line_defense(category),
        (PositionLine::DefenseLine, PossessionPhase::Offense) => defense_line_offense(category),
        (PositionLine::DefenseLine, _) => defense_line_defense(category),
        (PositionLine::Goalguard, PossessionPhase::Offense) => goalguard_offense(category),
        (PositionLine::Goalguard, _) => goalguard_defense(category),
    };
    let overlay = position_overlay(position, category, phase);
    (base * overlay).clamp(0.35, 1.65)
}

fn offensive_line_offense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 1.00,
        ObservationCategory::Pass => 0.78,
        ObservationCategory::Reception => 1.05,
        ObservationCategory::Carry => 1.05,
        ObservationCategory::Drive => 1.00,
        ObservationCategory::ArtrineDecision => 0.55,
        ObservationCategory::Recovery => 0.70,
        ObservationCategory::Turnover => 0.82,
        ObservationCategory::Scoring => 1.15,
        ObservationCategory::Assist => 0.90,
        ObservationCategory::Foul => 0.95,
        ObservationCategory::Punishment => 1.00,
        ObservationCategory::PasserContact => 0.70,
        ObservationCategory::KickFoul => 0.85,
    }
}

fn offensive_line_defense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 0.85,
        ObservationCategory::Pass => 0.55,
        ObservationCategory::Reception => 0.65,
        ObservationCategory::Carry => 0.70,
        ObservationCategory::Drive => 0.60,
        ObservationCategory::ArtrineDecision => 0.40,
        ObservationCategory::Recovery => 0.90,
        ObservationCategory::Turnover => 0.80,
        ObservationCategory::Scoring => 0.65,
        ObservationCategory::Assist => 0.60,
        ObservationCategory::Foul => 1.00,
        ObservationCategory::Punishment => 1.00,
        ObservationCategory::PasserContact => 0.85,
        ObservationCategory::KickFoul => 0.80,
    }
}

fn back_line_offense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 1.00,
        ObservationCategory::Pass => 1.00,
        ObservationCategory::Reception => 0.92,
        ObservationCategory::Carry => 1.00,
        ObservationCategory::Drive => 1.05,
        ObservationCategory::ArtrineDecision => 1.00,
        ObservationCategory::Recovery => 0.85,
        ObservationCategory::Turnover => 1.00,
        ObservationCategory::Scoring => 0.95,
        ObservationCategory::Assist => 1.00,
        ObservationCategory::Foul => 1.00,
        ObservationCategory::Punishment => 1.00,
        ObservationCategory::PasserContact => 0.85,
        ObservationCategory::KickFoul => 1.00,
    }
}

fn back_line_defense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 1.10,
        ObservationCategory::Pass => 0.70,
        ObservationCategory::Reception => 0.80,
        ObservationCategory::Carry => 0.85,
        ObservationCategory::Drive => 0.75,
        ObservationCategory::ArtrineDecision => 0.70,
        ObservationCategory::Recovery => 1.05,
        ObservationCategory::Turnover => 0.90,
        ObservationCategory::Scoring => 0.75,
        ObservationCategory::Assist => 0.80,
        ObservationCategory::Foul => 1.00,
        ObservationCategory::Punishment => 1.05,
        ObservationCategory::PasserContact => 1.15,
        ObservationCategory::KickFoul => 0.90,
    }
}

fn defense_line_offense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 0.65,
        ObservationCategory::Pass => 0.65,
        ObservationCategory::Reception => 0.70,
        ObservationCategory::Carry => 0.70,
        ObservationCategory::Drive => 0.65,
        ObservationCategory::ArtrineDecision => 0.40,
        ObservationCategory::Recovery => 0.75,
        ObservationCategory::Turnover => 0.75,
        ObservationCategory::Scoring => 0.65,
        ObservationCategory::Assist => 0.70,
        ObservationCategory::Foul => 0.95,
        ObservationCategory::Punishment => 1.00,
        ObservationCategory::PasserContact => 0.80,
        ObservationCategory::KickFoul => 0.80,
    }
}

fn defense_line_defense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 1.35,
        ObservationCategory::Pass => 0.55,
        ObservationCategory::Reception => 0.65,
        ObservationCategory::Carry => 0.70,
        ObservationCategory::Drive => 0.55,
        ObservationCategory::ArtrineDecision => 0.35,
        ObservationCategory::Recovery => 1.35,
        ObservationCategory::Turnover => 0.85,
        ObservationCategory::Scoring => 0.55,
        ObservationCategory::Assist => 0.65,
        ObservationCategory::Foul => 1.00,
        ObservationCategory::Punishment => 1.10,
        ObservationCategory::PasserContact => 1.30,
        ObservationCategory::KickFoul => 0.80,
    }
}

fn goalguard_offense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 0.60,
        ObservationCategory::Pass => 0.55,
        ObservationCategory::Reception => 0.50,
        ObservationCategory::Carry => 0.50,
        ObservationCategory::Drive => 0.40,
        ObservationCategory::ArtrineDecision => 0.30,
        ObservationCategory::Recovery => 0.65,
        ObservationCategory::Turnover => 1.10,
        ObservationCategory::Scoring => 0.45,
        ObservationCategory::Assist => 0.55,
        ObservationCategory::Foul => 0.90,
        ObservationCategory::Punishment => 1.10,
        ObservationCategory::PasserContact => 0.50,
        ObservationCategory::KickFoul => 0.75,
    }
}

fn goalguard_defense(category: ObservationCategory) -> f64 {
    match category {
        ObservationCategory::Duel => 1.50,
        ObservationCategory::Pass => 0.40,
        ObservationCategory::Reception => 0.45,
        ObservationCategory::Carry => 0.45,
        ObservationCategory::Drive => 0.30,
        ObservationCategory::ArtrineDecision => 0.35,
        ObservationCategory::Recovery => 0.95,
        ObservationCategory::Turnover => 0.95,
        ObservationCategory::Scoring => 0.40,
        ObservationCategory::Assist => 0.50,
        ObservationCategory::Foul => 0.90,
        ObservationCategory::Punishment => 1.10,
        ObservationCategory::PasserContact => 0.65,
        ObservationCategory::KickFoul => 0.70,
    }
}

fn position_overlay(
    position: Position,
    category: ObservationCategory,
    phase: PossessionPhase,
) -> f64 {
    match (position, phase, category) {
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Duel) => 1.08,
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Pass) => 0.65,
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Reception) => 1.10,
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Carry) => 0.82,
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Recovery) => 0.85,
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Turnover) => 0.68,
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Scoring) => 1.28,
        (Position::CenterOffense, PossessionPhase::Offense, ObservationCategory::Assist) => 0.82,
        (Position::WingOffense, PossessionPhase::Offense, ObservationCategory::Scoring) => 1.10,
        (Position::WingOffense, PossessionPhase::Offense, ObservationCategory::Reception) => 1.08,
        (Position::WingOffense, PossessionPhase::Offense, ObservationCategory::Carry) => 1.08,
        (Position::WingOffense, PossessionPhase::Offense, ObservationCategory::Turnover) => 0.90,
        (Position::Passer, PossessionPhase::Offense, ObservationCategory::Pass) => 1.30,
        (Position::Passer, PossessionPhase::Offense, ObservationCategory::Turnover) => 1.28,
        (Position::Passer, PossessionPhase::Offense, ObservationCategory::Assist) => 1.15,
        (Position::Passer, PossessionPhase::Offense, ObservationCategory::Scoring) => 0.78,
        (Position::Passer, PossessionPhase::Offense, ObservationCategory::Reception) => 0.78,
        (Position::Passer, PossessionPhase::Offense, ObservationCategory::Carry) => 0.78,
        (Position::Artrine, PossessionPhase::Offense, ObservationCategory::ArtrineDecision) => 1.25,
        (Position::Artrine, PossessionPhase::Offense, ObservationCategory::Pass) => 1.10,
        (Position::Artrine, PossessionPhase::Offense, ObservationCategory::Carry) => 1.15,
        (Position::Artrine, PossessionPhase::Offense, ObservationCategory::Drive) => 1.15,
        (Position::Artrine, PossessionPhase::Offense, ObservationCategory::Turnover) => 1.10,
        (Position::Artrine, PossessionPhase::Offense, ObservationCategory::Assist) => 1.10,
        (Position::PassRusher, PossessionPhase::Defense, ObservationCategory::Duel) => 1.20,
        (Position::PassRusher, PossessionPhase::Defense, ObservationCategory::PasserContact) => 1.30,
        (Position::PassRusher, PossessionPhase::Defense, ObservationCategory::Recovery) => 1.08,
        (Position::Centerback, PossessionPhase::Defense, ObservationCategory::Duel) => 1.15,
        (Position::Centerback, PossessionPhase::Defense, ObservationCategory::Recovery) => 1.15,
        (Position::DefensiveBlocker, PossessionPhase::Defense, ObservationCategory::Duel) => 1.20,
        (Position::DefensiveBlocker, PossessionPhase::Defense, ObservationCategory::Recovery) => 1.10,
        (Position::DefensiveBlocker, PossessionPhase::Defense, ObservationCategory::PasserContact) => 1.15,
        (Position::WideBlocker, PossessionPhase::Defense, ObservationCategory::Duel) => 1.20,
        (Position::WideBlocker, PossessionPhase::Defense, ObservationCategory::Recovery) => 1.10,
        (Position::WideBlocker, PossessionPhase::Defense, ObservationCategory::PasserContact) => 1.15,
        (Position::OutsideZonerback, PossessionPhase::Defense, ObservationCategory::Duel) => 1.10,
        (Position::OutsideZonerback, PossessionPhase::Defense, ObservationCategory::Recovery) => 1.10,
        (Position::MiddleZonerback, PossessionPhase::Defense, ObservationCategory::Duel) => 1.10,
        (Position::MiddleZonerback, PossessionPhase::Defense, ObservationCategory::Recovery) => 1.10,
        _ => 1.0,
    }
}
