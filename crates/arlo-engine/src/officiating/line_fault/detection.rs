use crate::officiating::line_fault::context::LineFaultEvaluationContext;
use crate::officiating::line_fault::depth_gate::{
    evaluate_reception_depth_gate, scale_line_fault_probability,
};
use crate::resolution::{resolve_contest, ContestOrientation, ContestRequest, DuelKind};
use arlo_domain::{AttributeKey, PitchZone, Player, Position as DomainPosition};
use arlo_math::Probability;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn identify_last_defender<'a>(
    outfield_defenders: &[&'a Player],
    defense_pos_index: &HashMap<Uuid, DomainPosition>,
) -> Option<&'a Player> {
    if outfield_defenders.is_empty() {
        return None;
    }
    outfield_defenders
        .iter()
        .copied()
        .find(|p| defense_pos_index.get(&p.id()) == Some(&DomainPosition::Centerback))
        .or_else(|| {
            outfield_defenders
                .iter()
                .copied()
                .find(|p| defense_pos_index.get(&p.id()) == Some(&DomainPosition::MiddleZonerback))
        })
        .or_else(|| outfield_defenders.first().copied())
}

pub fn is_line_fault<R: Rng + ?Sized>(
    ctx: &LineFaultEvaluationContext<'_>,
    rng: &mut R,
) -> (bool, f64) {
    let gate = evaluate_reception_depth_gate(
        ctx.normalized_proximity,
        ctx.defensive_line_height,
        ctx.zone,
    );

    if !gate.is_plausibly_beyond {
        return (false, 0.0);
    }

    let def_pos = ctx.defender_table.get(AttributeKey::Positioning);
    let def_ant = ctx.defender_table.get(AttributeKey::Anticipation);
    let defense_trap_rating = def_pos * 0.55 + def_ant * 0.45;

    let rec_ant = ctx.receiver_table.get(AttributeKey::Anticipation);
    let rec_dec = ctx.receiver_table.get(AttributeKey::Decisions);
    let receiver_timing_rating = rec_ant * 0.55 + rec_dec * 0.45;

    let zone_defense_bonus = match ctx.zone {
        PitchZone::FirstZone => 1.5,
        PitchZone::SecondZone => 0.5,
        PitchZone::OpenField => 0.0,
    };

    let defense_effective = defense_trap_rating + zone_defense_bonus;
    let receiver_effective = receiver_timing_rating + 4.0;

    let req = ContestRequest::for_contest(
        DuelKind::RouteContest,
        defense_effective,
        receiver_effective,
        &ctx.duel_context.with_orientation(ContestOrientation::AttackerIsDefense),
    )
    .with_slope(0.35);

    let outcome = resolve_contest(req, rng);
    let base_p = outcome.win_probability().value();
    let scaled_p = scale_line_fault_probability(base_p, gate.depth_margin);
    let fault_occurred = Probability::new_clamped(scaled_p).sample(rng);

    let margin = if fault_occurred {
        (outcome.net_advantage() * 0.2 + gate.depth_margin * 5.0).max(0.0)
    } else {
        0.0
    };

    (fault_occurred, margin)
}