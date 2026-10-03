use arlo_events::DuelKind;

const DUEL_EXPECTATION_SCALE: f64 = 1.10;
const PERSONAL_SURPRISE_HIGH_IMPACT_SCALE: f64 = 0.45;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DuelPerformanceSignal {
    pub base: f64,
    pub high_impact: f64,
}

pub(crate) fn performance_signal(
    kind: DuelKind,
    attacker_won: bool,
    personal_win_probability: f64,
) -> DuelPerformanceSignal {
    let observed = if attacker_won { 1.0 } else { 0.0 };
    let structural = structural_attacker_win_probability(kind);
    let base = (observed - structural) * DUEL_EXPECTATION_SCALE;
    let personal_surprise =
        (observed - personal_win_probability.clamp(0.0, 1.0))
            * PERSONAL_SURPRISE_HIGH_IMPACT_SCALE;

    DuelPerformanceSignal {
        base,
        high_impact: base + personal_surprise,
    }
}

pub(crate) const fn structural_attacker_win_probability(kind: DuelKind) -> f64 {
    match kind {
        DuelKind::PassProtection => 0.685,
        DuelKind::RouteContest => 0.820,
        DuelKind::RunBreakthrough => 0.535,
        DuelKind::CentralBlock => 0.685,
        DuelKind::LateralBlock => 0.685,
        DuelKind::ArtroBreakthrough => 0.545,
        DuelKind::AerialDuel => 0.805,
        DuelKind::FinishingAttempt => 0.278,
        DuelKind::FieldGoalAttempt => 0.328,
        DuelKind::ShortDistribution => 0.825,
        DuelKind::LongDistribution => 0.720,
        DuelKind::CrossDistribution => 0.752,
        DuelKind::BallSecurityCarry => 0.550,
        DuelKind::BallSecurityDistribution => 0.550,
        DuelKind::KickBlockAttempt => 0.500,
    }
}