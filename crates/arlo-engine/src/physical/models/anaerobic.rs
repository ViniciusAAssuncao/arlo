use crate::physical::state::PhysicalState;
use crate::resolution::DuelKind;

pub fn calculate_duel_intensity_multiplier(duel_kind: DuelKind) -> f64 {
    match duel_kind {
        DuelKind::FinishingAttempt => 3.2,
        DuelKind::ArtroBreakthrough => 2.6,
        DuelKind::RunBreakthrough => 2.2,
        DuelKind::CentralBlock | DuelKind::LateralBlock => 2.0,
        DuelKind::PassProtection | DuelKind::KickBlockAttempt => 1.8,
        DuelKind::RouteContest | DuelKind::AerialDuel => 1.7,
        DuelKind::BallSecurityCarry | DuelKind::BallSecurityDistribution => 1.6,
        DuelKind::CrossDistribution => 1.4,
        DuelKind::ShortDistribution
        | DuelKind::LongDistribution
        | DuelKind::FieldGoalAttempt => 1.2,
    }
}

pub fn calculate_contest_reserve_cost(
    intensity_multiplier: f64,
    strength: f64,
    acceleration: f64,
) -> f64 {
    let norm_str = (strength.clamp(0.0, 20.0)) / 20.0;
    let norm_acc = (acceleration.clamp(0.0, 20.0)) / 20.0;
    let power_buffer = 0.50 * norm_str + 0.50 * norm_acc;
    let base_cost = 0.08 * intensity_multiplier;
    (base_cost * (1.35 - 0.55 * power_buffer)).clamp(0.02, 0.45)
}

pub fn apply_contest_reserve_cost(
    state: &mut PhysicalState,
    intensity_multiplier: f64,
    strength: f64,
    acceleration: f64,
) {
    let cost = calculate_contest_reserve_cost(intensity_multiplier, strength, acceleration);
    let new_reserve = (state.w_prime_balance() - cost).clamp(0.0, 1.0);
    state.set_w_prime_balance(new_reserve);
}