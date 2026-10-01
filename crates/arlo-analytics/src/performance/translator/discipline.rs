use crate::context::MatchAnalysisContext;
use crate::performance::observation::{ObservationCategory, PerformanceObservation, PossessionPhase};
use crate::performance::rating::PerformanceBreakdown;
use arlo_domain::PunishmentKind;
use arlo_events::{
    FoulOrigin, FoulRaised, KickFoulAwarded, KickFoulDecisionMade, MatchClockInstant,
    PunishmentApplied, RefereeDecisionResolved,
};
use uuid::Uuid;

pub(crate) fn translate_foul_raised(
    event: &FoulRaised,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let mut obs = Vec::new();

    if event.final_call_correct() {
        let (disc_penalty, hi_penalty) = match event.origin() {
            FoulOrigin::ContactDuel(_) => (-0.85, -0.35),
            FoulOrigin::LineFault => (-0.45, -0.15),
            FoulOrigin::ShotAttempt => (-0.75, -0.30),
            FoulOrigin::CallToAction | FoulOrigin::Drive => (-0.60, -0.25),
            _ => (-0.50, -0.20),
        };

        let off_bd = PerformanceBreakdown::new_unchecked(
            -0.30,
            0.0,
            0.0,
            0.0,
            disc_penalty,
            hi_penalty,
        );

        obs.push(PerformanceObservation::new_unchecked(
            event.offending_player_id(),
            event.offending_team_id(),
            clock,
            PossessionPhase::Defense,
            ObservationCategory::Foul,
            off_bd,
            1.0,
            1.20,
            format!("Foul committed ({:?})", event.origin()),
        ));

        if event.origin() != FoulOrigin::Lineup {
            let drawn_bd = PerformanceBreakdown::new_unchecked(0.20, 0.0, 0.0, 0.0, 0.0, 0.25);
            obs.push(PerformanceObservation::new_unchecked(
                event.opposing_player_id(),
                event.opposing_team_id(),
                clock,
                PossessionPhase::Offense,
                ObservationCategory::Foul,
                drawn_bd,
                0.5,
                1.10,
                format!("Foul drawn ({:?})", event.origin()),
            ));
        }
    }

    obs
}

pub(crate) fn translate_punishment_applied(
    event: &PunishmentApplied,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let (disc, hi) = match event.kind() {
        PunishmentKind::YardageLoss => {
            let mirim = event.magnitude().unwrap_or(5).max(0) as f64;
            (-0.10 * mirim, -0.05 * mirim)
        }
        PunishmentKind::LossOfDown => (-0.65, -0.50),
        PunishmentKind::LossOfDrive => (-0.85, -0.85),
        PunishmentKind::TimePenalty => (-0.75, -0.55),
        PunishmentKind::Expulsion => (-2.00, -1.60),
        PunishmentKind::InvalidatePreviousPlay => (-0.55, -0.60),
        PunishmentKind::KickFoulAwarded => (-0.60, -0.50),
    };

    let bd = PerformanceBreakdown::new_unchecked(-0.25, 0.0, 0.0, 0.0, disc, hi);

    vec![PerformanceObservation::new_unchecked(
        event.offending_player_id(),
        event.offending_team_id(),
        clock,
        PossessionPhase::Defense,
        ObservationCategory::Punishment,
        bd,
        1.0,
        1.35,
        format!("Punishment applied: {:?}", event.kind()),
    )]
}

pub(crate) fn translate_referee_decision(
    event: &RefereeDecisionResolved,
    clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    if event.peace_referee_intervened() && !event.factual_foul() && event.original_call() {
        let exoneration_bd = PerformanceBreakdown::new_unchecked(0.15, 0.0, 0.0, 0.0, 0.30, 0.10);
        vec![PerformanceObservation::new_unchecked(
            event.offending_player_id(),
            event.offending_team_id(),
            clock,
            PossessionPhase::Neutral,
            ObservationCategory::Foul,
            exoneration_bd,
            0.5,
            1.0,
            "False foul overturned by peace referee".into(),
        )]
    } else {
        Vec::new()
    }
}

pub(crate) fn translate_kick_foul_awarded(
    _event: &KickFoulAwarded,
    _clock: MatchClockInstant,
    _context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    Vec::new()
}

pub(crate) fn translate_kick_foul_decision(
    event: &KickFoulDecisionMade,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let taker_team = resolve_team(event.taker_id(), offense_team_id, context);
    let bd = PerformanceBreakdown::new_unchecked(0.30, 0.20, 0.0, 0.10, 0.0, 0.20);

    vec![PerformanceObservation::new_unchecked(
        event.taker_id(),
        taker_team,
        clock,
        PossessionPhase::Offense,
        ObservationCategory::KickFoul,
        bd,
        0.5,
        1.10,
        format!("Kick foul decision: {:?}", event.decision()),
    )]
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
