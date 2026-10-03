use crate::context::MatchAnalysisContext;
use crate::performance::observation::{ObservationCategory, PerformanceObservation, PossessionPhase};
use crate::performance::rating::PerformanceBreakdown;
use arlo_events::{GoalguardRecoveryResolved, MatchClockInstant, Turnover};

pub(crate) fn translate_turnover(
    event: &Turnover,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
    follows_missed_shot: bool,
) -> Vec<PerformanceObservation> {
    let mut obs = Vec::new();

    if let Some(recoverer_id) = event.recovering_player() {
        let (bd, opportunity, leverage, description) = if follows_missed_shot {
            (
                PerformanceBreakdown::new_unchecked(0.08, 0.0, 0.12, 0.18, 0.0, 0.05),
                0.35,
                1.0,
                "Missed shot recovery secured",
            )
        } else {
            (
                PerformanceBreakdown::new_unchecked(0.20, 0.08, 0.35, 0.45, 0.0, 0.30),
                0.75,
                1.25,
                "Turnover recovered",
            )
        };

        obs.push(PerformanceObservation::new_unchecked(
            recoverer_id,
            event.new_offense(),
            clock,
            PossessionPhase::Defense,
            ObservationCategory::Recovery,
            bd,
            opportunity,
            leverage,
            description.into(),
        ));
    }

    if !follows_missed_shot {
        if let Some(lost_id) = event.lost_by_player_id() {
            let bd =
                PerformanceBreakdown::new_unchecked(-0.75, -0.30, 0.0, -1.60, 0.0, 0.0);
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
    }

    obs
}

pub(crate) fn translate_goalguard_recovery(
    _event: &GoalguardRecoveryResolved,
    _clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    Vec::new()
}
