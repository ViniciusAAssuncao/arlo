use crate::context::MatchAnalysisContext;
use crate::performance::observation::{ObservationCategory, PerformanceObservation, PossessionPhase};
use crate::performance::rating::PerformanceBreakdown;
use arlo_events::{ArtrineDecisionMade, DriveRecorded, MatchClockInstant, PasserContactResolved};
use uuid::Uuid;

pub(crate) fn translate_artrine_decision(
    event: &ArtrineDecisionMade,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let artrine_team = resolve_team(event.artrine_id(), offense_team_id, context);
    let down_leverage = match event.down_number() {
        1 => 1.0,
        2 => 1.25,
        3 => 1.65,
        _ => 2.30,
    };

    let prob = event.decision_probability().value();
    let quality_factor = 0.40 + (prob * 0.35);

    let exec = quality_factor * 0.55 * down_leverage;
    let prod = quality_factor * 0.45 * down_leverage;
    let hi = quality_factor * 0.65 * down_leverage;
    let sec = 0.15;

    let bd = PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi);

    vec![PerformanceObservation::new_unchecked(
        event.artrine_id(),
        artrine_team,
        clock,
        PossessionPhase::Offense,
        ObservationCategory::ArtrineDecision,
        bd,
        1.0,
        down_leverage,
        format!(
            "Artrine {:?} (Down {})",
            event.decision_kind(),
            event.down_number()
        ),
    )]
}

pub(crate) fn translate_drive_recorded(
    event: &DriveRecorded,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let artrine_team = resolve_team(event.artrine_id(), offense_team_id, context);
    let drives = event.drives_in_series();

    let (hi_bonus, lev) = if drives >= 3 {
        (1.50, 2.50)
    } else if drives == 2 {
        (1.00, 1.70)
    } else {
        (0.80, 1.30)
    };

    let exec = 0.55;
    let prod = 0.75;
    let sec = 0.30;

    let bd = PerformanceBreakdown::new_unchecked(exec, prod, 0.0, sec, 0.0, hi_bonus);

    vec![PerformanceObservation::new_unchecked(
        event.artrine_id(),
        artrine_team,
        clock,
        PossessionPhase::Offense,
        ObservationCategory::Drive,
        bd,
        1.0,
        lev,
        format!(
            "Drive {} in series at {:?}",
            drives,
            event.placement()
        ),
    )]
}

pub(crate) fn translate_passer_contact(
    event: &PasserContactResolved,
    clock: MatchClockInstant,
    _offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let mut obs = Vec::new();
    let def_team = resolve_team(event.defender_id(), None, context);

    if event.violent() {
        let bd = PerformanceBreakdown::new_unchecked(-0.40, 0.0, 0.10, 0.0, -1.30, -0.50);
        obs.push(PerformanceObservation::new_unchecked(
            event.defender_id(),
            def_team,
            clock,
            PossessionPhase::Defense,
            ObservationCategory::PasserContact,
            bd,
            1.0,
            1.2,
            "Violent contact on passer".into(),
        ));
    } else if event.rough() || event.late() {
        let bd = PerformanceBreakdown::new_unchecked(-0.25, 0.0, 0.15, 0.0, -0.80, -0.30);
        obs.push(PerformanceObservation::new_unchecked(
            event.defender_id(),
            def_team,
            clock,
            PossessionPhase::Defense,
            ObservationCategory::PasserContact,
            bd,
            1.0,
            1.1,
            "Rough or late contact on passer".into(),
        ));
    } else {
        let bd = PerformanceBreakdown::new_unchecked(0.50, 0.0, 0.75, 0.0, 0.0, 0.40);
        obs.push(PerformanceObservation::new_unchecked(
            event.defender_id(),
            def_team,
            clock,
            PossessionPhase::Defense,
            ObservationCategory::PasserContact,
            bd,
            1.0,
            1.2,
            "Legal passer pressure applied".into(),
        ));
    }

    obs
}

fn resolve_team(
    player_id: Uuid,
    fallback_team: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Uuid {
    if let Some(ctx) = context {
        if let Some(tid) = ctx.team_id(&player_id) {
            return tid;
        }
    }
    fallback_team.unwrap_or_default()
}
