use crate::context::MatchAnalysisContext;
use crate::performance::observation::{ObservationCategory, PerformanceObservation, PossessionPhase};
use crate::performance::rating::PerformanceBreakdown;
use arlo_domain::PitchZone;
use arlo_events::{GoalguardRecoveryResolved, MatchClockInstant, Turnover};

pub(crate) fn translate_turnover(
    event: &Turnover,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let mut obs = Vec::new();

    if let Some(recoverer_id) = event.recovering_player() {
        let bd = PerformanceBreakdown::new_unchecked(0.40, 0.20, 0.65, 0.85, 0.0, 0.95);
        obs.push(PerformanceObservation::new_unchecked(
            recoverer_id,
            event.new_offense(),
            clock,
            PossessionPhase::Defense,
            ObservationCategory::Recovery,
            bd,
            1.0,
            1.75,
            "Turnover recovered".into(),
        ));
    }

    if let Some(lost_id) = event.lost_by_player_id() {
        let bd = PerformanceBreakdown::new_unchecked(-0.75, -0.30, 0.0, -1.60, 0.0, -1.30);
        obs.push(PerformanceObservation::new_unchecked(
            lost_id,
            event.previous_offense(),
            clock,
            PossessionPhase::Offense,
            ObservationCategory::Turnover,
            bd,
            1.0,
            1.85,
            "Turnover conceded".into(),
        ));
    }

    obs
}

pub(crate) fn translate_goalguard_recovery(
    event: &GoalguardRecoveryResolved,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let zone_bonus = match event.zone() {
        PitchZone::FirstZone => 0.40,
        PitchZone::SecondZone => 0.20,
        PitchZone::OpenField => 0.05,
    };

    let hands_bonus = if event.used_hands() { 0.20 } else { 0.0 };

    let def = 0.70 + zone_bonus;
    let exec = 0.50 + hands_bonus;
    let sec = 0.55;
    let hi = 0.25 + zone_bonus;

    let bd = PerformanceBreakdown::new_unchecked(exec, 0.0, def, sec, 0.0, hi);

    vec![PerformanceObservation::new_unchecked(
        event.goalguard_id(),
        event.team_id(),
        clock,
        PossessionPhase::Defense,
        ObservationCategory::Recovery,
        bd,
        1.0,
        1.30,
        format!(
            "Goalguard recovery at {:?} (hands: {})",
            event.zone(),
            event.used_hands()
        ),
    )]
}
