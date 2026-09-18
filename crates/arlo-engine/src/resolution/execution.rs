use crate::resolution::duel_kind::DuelKind;
use crate::resolution::evaluation::EvaluatedDuel;
use crate::resolution::outcome::DuelOutcome;
use rand::Rng;

pub fn calculate_velocity_mitigation(
    kind: DuelKind,
    attacker_won: bool,
    net_advantage: f64,
) -> f64 {
    let base = match kind {
        DuelKind::ArtroBreakthrough | DuelKind::RunBreakthrough => {
            if attacker_won {
                0.80 + (net_advantage * 0.04)
            } else {
                0.25 + (net_advantage * 0.03)
            }
        }
        DuelKind::CentralBlock | DuelKind::LateralBlock => {
            if attacker_won {
                0.70 + (net_advantage * 0.03)
            } else {
                0.20 + (net_advantage * 0.02)
            }
        }
        DuelKind::BallSecurityCarry | DuelKind::BallSecurityDistribution => {
            if attacker_won {
                0.60 + (net_advantage * 0.04)
            } else {
                0.00
            }
        }
        _ => {
            if attacker_won {
                0.85 + (net_advantage * 0.02)
            } else {
                0.35 + (net_advantage * 0.02)
            }
        }
    };

    if attacker_won {
        base.clamp(0.40, 1.00)
    } else {
        base.clamp(0.00, 0.40)
    }
}

pub fn execute_duel<R: Rng + ?Sized>(
    evaluated: &EvaluatedDuel,
    rng: &mut R,
) -> DuelOutcome {
    let attacker_won = evaluated.win_probability.sample(rng);
    let velocity_mitigation =
        calculate_velocity_mitigation(evaluated.kind, attacker_won, evaluated.net_advantage);

    DuelOutcome::with_mitigation(
        evaluated.kind,
        attacker_won,
        evaluated.effective_attacker,
        evaluated.effective_defender,
        evaluated.win_probability,
        evaluated.net_advantage,
        velocity_mitigation,
    )
}