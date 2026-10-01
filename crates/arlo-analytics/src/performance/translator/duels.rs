use crate::context::MatchAnalysisContext;
use crate::performance::observation::{ObservationCategory, PerformanceObservation, PossessionPhase};
use crate::performance::rating::PerformanceBreakdown;
use arlo_events::{DuelKind, DuelResolved, MatchClockInstant};
use uuid::Uuid;

pub(crate) fn translate_duel(
    event: &DuelResolved,
    clock: MatchClockInstant,
    offense_team_id: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
) -> Vec<PerformanceObservation> {
    let mut observations = Vec::new();
    let win_prob = event.win_probability().value();
    let net_adv = event.net_advantage();
    let attacker_won = event.attacker_won();

    let (att_perf, def_perf) = if attacker_won {
        let upset = (1.0 - win_prob) * 0.45;
        let margin = (net_adv.max(0.0) * 0.12).min(0.40);
        (0.50 + upset + margin, -(0.40 + win_prob * 0.30 + margin * 0.50))
    } else {
        let upset = win_prob * 0.45;
        let margin = ((-net_adv).max(0.0) * 0.12).min(0.40);
        (-(0.40 + (1.0 - win_prob) * 0.30 + margin * 0.50), 0.50 + upset + margin)
    };

    let att_count = event.attacker_ids().len().max(1) as f64;
    let def_count = event.defender_ids().len().max(1) as f64;
    let att_scale = 1.0 / att_count.sqrt();
    let def_scale = 1.0 / def_count.sqrt();

    let (att_bd, def_bd) = breakdown_for_duel(event.kind(), att_perf * att_scale, def_perf * def_scale);

    for &att_id in event.attacker_ids() {
        let team_id = resolve_team(att_id, offense_team_id, context, true);
        let phase = if Some(team_id) == offense_team_id {
            PossessionPhase::Offense
        } else {
            PossessionPhase::Defense
        };

        observations.push(PerformanceObservation::new_unchecked(
            att_id,
            team_id,
            clock,
            phase,
            ObservationCategory::Duel,
            att_bd,
            1.0 * att_scale,
            1.0,
            format!("{} attacker", event.kind().as_str()),
        ));
    }

    for &def_id in event.defender_ids() {
        let team_id = resolve_team(def_id, offense_team_id, context, false);
        let phase = if Some(team_id) == offense_team_id {
            PossessionPhase::Offense
        } else {
            PossessionPhase::Defense
        };

        observations.push(PerformanceObservation::new_unchecked(
            def_id,
            team_id,
            clock,
            phase,
            ObservationCategory::Duel,
            def_bd,
            1.0 * def_scale,
            1.0,
            format!("{} defender", event.kind().as_str()),
        ));
    }

    observations
}

fn breakdown_for_duel(kind: DuelKind, ap: f64, dp: f64) -> (PerformanceBreakdown, PerformanceBreakdown) {
    match kind {
        DuelKind::PassProtection | DuelKind::CentralBlock | DuelKind::LateralBlock => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.75, ap * 0.15, 0.0, 0.0, 0.0, ap * 0.10);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.35, 0.0, dp * 0.80, 0.0, 0.0, dp * 0.25);
            (att, def)
        }
        DuelKind::RunBreakthrough | DuelKind::ArtroBreakthrough => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.55, ap * 0.60, 0.0, ap * 0.20, 0.0, ap * 0.40);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.35, 0.0, dp * 0.85, 0.0, 0.0, dp * 0.35);
            (att, def)
        }
        DuelKind::RouteContest => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.55, ap * 0.45, 0.0, 0.0, 0.0, ap * 0.15);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.35, 0.0, dp * 0.80, 0.0, 0.0, dp * 0.20);
            (att, def)
        }
        DuelKind::AerialDuel => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.50, ap * 0.45, 0.0, ap * 0.15, 0.0, ap * 0.30);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.35, 0.0, dp * 0.80, 0.0, 0.0, dp * 0.30);
            (att, def)
        }
        DuelKind::FinishingAttempt => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.65, ap * 0.75, 0.0, 0.0, 0.0, ap * 0.60);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.45, 0.0, dp * 0.95, 0.0, 0.0, dp * 0.60);
            (att, def)
        }
        DuelKind::FieldGoalAttempt => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.60, ap * 0.65, 0.0, 0.0, 0.0, ap * 0.40);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.40, 0.0, dp * 0.80, 0.0, 0.0, dp * 0.45);
            (att, def)
        }
        DuelKind::ShortDistribution | DuelKind::LongDistribution | DuelKind::CrossDistribution => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.65, ap * 0.45, 0.0, ap * 0.30, 0.0, ap * 0.20);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.35, 0.0, dp * 0.75, 0.0, 0.0, dp * 0.15);
            (att, def)
        }
        DuelKind::BallSecurityCarry | DuelKind::BallSecurityDistribution => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.35, ap * 0.20, 0.0, ap * 0.90, 0.0, ap * 0.20);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.30, 0.0, dp * 0.70, 0.0, 0.0, dp * 0.35);
            (att, def)
        }
        DuelKind::KickBlockAttempt => {
            let att = PerformanceBreakdown::new_unchecked(ap * 0.50, ap * 0.45, 0.0, 0.0, 0.0, ap * 0.20);
            let def = PerformanceBreakdown::new_unchecked(dp * 0.40, 0.0, dp * 0.85, 0.0, 0.0, dp * 0.55);
            (att, def)
        }
    }
}

fn resolve_team(
    player_id: Uuid,
    offense_team: Option<Uuid>,
    context: Option<&MatchAnalysisContext>,
    is_attacker: bool,
) -> Uuid {
    if let Some(ctx) = context {
        if let Some(tid) = ctx.team_id(&player_id) {
            return tid;
        }
    }
    if is_attacker {
        offense_team.unwrap_or_default()
    } else {
        Uuid::default()
    }
}
