use crate::context::MatchAnalysisContext;
use crate::performance::observation::{ObservationCategory, PerformanceObservation, PossessionPhase};
use crate::performance::rating::PerformanceBreakdown;
use arlo_events::{
    CarryResolved, DistributionCompleted, MatchClockInstant, PassCompleted, ReceptionResolved,
};
use uuid::Uuid;

pub(crate) fn translate_pass_completed(
    event: &PassCompleted,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let passer_team = resolve_team(event.passer_id(), offense_team_id, context);
    let dist = event.distance_mirim().max(0.0);
    let dist_norm = (dist / 40.0).min(1.0);
    let aerial_bonus = if event.is_aerial() { 0.08 } else { 0.0 };

    let exec = 0.05 + dist_norm * 0.45 + aerial_bonus;
    let prod = (dist * 0.035).min(0.90);
    let sec = 0.05;
    let hi = if dist >= 24.0 {
        0.22
    } else if dist >= 16.0 {
        0.08
    } else {
        0.0
    };

    let bd = PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi);
    let lev = 1.0 + (dist / 35.0).min(0.50);

    vec![PerformanceObservation::new_unchecked(
        event.passer_id(),
        passer_team,
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Pass,
        bd,
        1.0,
        lev,
        format!("Pass completed ({:.1}m)", dist),
    )]
}

pub(crate) fn translate_reception_resolved(
    event: &ReceptionResolved,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let receiver_team = resolve_team(event.receiver_id(), offense_team_id, context);
    let aerial_adj = if event.is_aerial() { 0.12 } else { 0.0 };

    let (bd, desc) = if event.caught() {
        let exec = 0.12 + aerial_adj;
        let prod = 0.08;
        let sec = 0.06;
        let hi = if event.is_aerial() { 0.15 } else { 0.0 };
        (
            PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi),
            "Reception caught",
        )
    } else {
        let exec = -0.65;
        let prod = -0.35;
        let sec = -0.45;
        let hi = -0.25;
        (
            PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi),
            "Reception dropped",
        )
    };

    let mut observations = vec![PerformanceObservation::new_unchecked(
        event.receiver_id(),
        receiver_team,
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Reception,
        bd,
        1.0,
        1.0,
        desc.into(),
    )];

    if !event.caught() {
        let passer_team = resolve_team(event.passer_id(), offense_team_id, context);
        let passer_bd =
            PerformanceBreakdown::new_unchecked(-0.18, -0.10, 0.0, -0.12, 0.0, -0.05);
        observations.push(PerformanceObservation::new_unchecked(
            event.passer_id(),
            passer_team,
            clock,
            PossessionPhase::Offense,
            ObservationCategory::Pass,
            passer_bd,
            0.75,
            1.0,
            "Pass attempt not completed".into(),
        ));
    }

    observations
}

pub(crate) fn translate_distribution_completed(
    event: &DistributionCompleted,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let passer_team = resolve_team(event.passer_id(), offense_team_id, context);
    let dist = event.distance_mirim().max(0.0);
    let dist_norm = (dist / 45.0).min(1.0);
    let aerial_bonus = if event.is_aerial() { 0.06 } else { 0.0 };

    let passer_bd = if event.caught() {
        let exec = 0.06 + dist_norm * 0.40 + aerial_bonus;
        let prod = (dist * 0.03).min(0.75);
        let sec = 0.05;
        let hi = if dist >= 20.0 { 0.15 } else { 0.0 };
        PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi)
    } else {
        PerformanceBreakdown::new_unchecked(-0.45, -0.25, 0.0, -0.30, 0.0, -0.20)
    };

    vec![PerformanceObservation::new_unchecked(
        event.passer_id(),
        passer_team,
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Pass,
        passer_bd,
        1.0,
        1.0,
        format!("Distribution {:?}", event.decision_kind()),
    )]
}

pub(crate) fn translate_carry_resolved(
    event: &CarryResolved,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let carrier_team = resolve_team(event.carrier_id(), offense_team_id, context);
    let gain = event.gain_mirim();

    let (bd, lev) = if gain > 0.0 {
        let exec = 0.05 + (gain * 0.03).min(0.40);
        let prod = (gain * 0.05).min(0.95);
        let sec = 0.05;
        let hi = if gain >= 10.0 {
            0.40
        } else if gain >= 5.0 {
            0.15
        } else {
            0.0
        };
        let lev = 1.0 + (gain / 20.0).min(0.50);
        (
            PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi),
            lev,
        )
    } else {
        let exec = (-0.25 + gain * 0.03).max(-0.60);
        let prod = (gain * 0.04).max(-0.50);
        let sec = -0.05;
        let hi = -0.15;
        (
            PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi),
            1.0,
        )
    };

    vec![PerformanceObservation::new_unchecked(
        event.carrier_id(),
        carrier_team,
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Carry,
        bd,
        1.0,
        lev,
        format!("Carry gain {:.1}m", gain),
    )]
}

fn resolve_team(
    player_id: Uuid,
    offense_team: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Uuid {
    if let Some(ctx) = context {
        if let Some(tid) = ctx.team_id(&player_id) {
            return tid;
        }
    }
    offense_team.unwrap_or_default()
}