use crate::context::MatchAnalysisContext;
use crate::performance::observation::{ObservationCategory, PerformanceObservation, PossessionPhase};
use crate::performance::rating::PerformanceBreakdown;
use arlo_events::{
    FieldGoalScored, FieldPointScored, GoalPointScored, MatchClockInstant, ScoringAttemptMissed,
    ScoringPost,
};

pub(crate) fn translate_goal_point(
    event: &GoalPointScored,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let mut obs = Vec::new();

    let scorer_bd = PerformanceBreakdown::new_unchecked(0.90, 1.60, 0.0, 0.30, 0.0, 1.80);
    obs.push(PerformanceObservation::new_unchecked(
        event.scorer_id(),
        event.team_id(),
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Scoring,
        scorer_bd,
        1.0,
        1.80,
        "Goal Point scored (5 pts)".into(),
    ));

    if event.scorer_id() != event.artrine_id() {
        let artrine_bd = PerformanceBreakdown::new_unchecked(0.55, 0.85, 0.0, 0.20, 0.0, 1.20);
        obs.push(PerformanceObservation::new_unchecked(
            event.artrine_id(),
            event.team_id(),
            clock,
            PossessionPhase::Offense,
            ObservationCategory::Scoring,
            artrine_bd,
            1.0,
            1.50,
            "Orchestrated Goal Point drive sequence".into(),
        ));
    }

    if let Some(assister_id) = event.assister_id() {
        let assist_bd = PerformanceBreakdown::new_unchecked(0.60, 0.90, 0.0, 0.20, 0.0, 0.90);
        obs.push(PerformanceObservation::new_unchecked(
            assister_id,
            event.team_id(),
            clock,
            PossessionPhase::Offense,
            ObservationCategory::Assist,
            assist_bd,
            1.0,
            1.40,
            "Assist on Goal Point".into(),
        ));
    }

    obs
}

pub(crate) fn translate_field_point(
    event: &FieldPointScored,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let adv_bonus = (event.territory_advance_mirim() * 0.02).clamp(0.0, 0.40);
    let scorer_bd = PerformanceBreakdown::new_unchecked(0.75, 1.15 + adv_bonus, 0.0, 0.20, 0.0, 1.25);

    vec![PerformanceObservation::new_unchecked(
        event.scorer_id(),
        event.team_id(),
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Scoring,
        scorer_bd,
        1.0,
        1.45,
        format!(
            "Field Point scored (3 pts, advance {:.1}m)",
            event.territory_advance_mirim()
        ),
    )]
}

pub(crate) fn translate_field_goal(
    event: &FieldGoalScored,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let (pts, hi, lev) = match event.post() {
        ScoringPost::Goalpost => (2, 0.90, 1.30),
        ScoringPost::Fieldpost => (1, 0.60, 1.15),
    };

    let bd = PerformanceBreakdown::new_unchecked(0.65, 0.70, 0.0, 0.10, 0.0, hi);

    vec![PerformanceObservation::new_unchecked(
        event.scorer_id(),
        event.team_id(),
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Scoring,
        bd,
        1.0,
        lev,
        format!("Field Goal scored ({} pts, {:?})", pts, event.post()),
    )]
}

pub(crate) fn translate_scoring_missed(
    event: &ScoringAttemptMissed,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let penalty = match event.attempted_post() {
        ScoringPost::Goalpost => (-0.60, -0.55, -0.75),
        ScoringPost::Fieldpost => (-0.45, -0.40, -0.50),
    };

    let bd = PerformanceBreakdown::new_unchecked(penalty.0, penalty.1, 0.0, 0.0, 0.0, penalty.2);

    vec![PerformanceObservation::new_unchecked(
        event.scorer_id(),
        event.team_id(),
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Scoring,
        bd,
        1.0,
        1.20,
        format!("Scoring attempt missed ({:?})", event.attempted_post()),
    )]
}
