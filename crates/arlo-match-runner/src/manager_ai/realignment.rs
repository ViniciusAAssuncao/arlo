use super::context::{ManagerDecisionContext, PlayerContext, SlotContext};
use super::diagnosis::TeamDiagnosis;
use super::perception::PlayerPerception;
use super::valuation::{contribution, need_fit};
use arlo_domain::{Position, SlotRole};
use arlo_manager_control::TacticalRealignmentIntent;

#[derive(Debug, Clone)]
pub struct RealignmentCandidate {
    pub intent: TacticalRealignmentIntent,
    pub utility: f64,
    pub contribution_gain: f64,
    pub tactical_gain: f64,
    pub change_cost: f64,
}

pub(super) fn generate(
    context: &ManagerDecisionContext,
    perceptions: &[PlayerPerception],
    diagnosis: &TeamDiagnosis,
) -> Vec<RealignmentCandidate> {
    if context.skills.adaptability < 0.3
        || context
            .last_plan_activation_at
            .is_some_and(|last| context.elapsed - last < 900.0)
        || context.skills.knowledge < 0.3
        || context
            .last_realignment_at
            .is_some_and(|last| context.elapsed - last < 1200.0)
        || context
            .last_substitution_at
            .is_some_and(|last| context.elapsed - last < 600.0)
        || context
            .last_tactical_switch_at
            .is_some_and(|last| context.elapsed - last < 600.0)
    {
        return Vec::new();
    }
    let mut candidates = Vec::new();
    for first in 0..context.slots.len() {
        for second in first + 1..context.slots.len() {
            let left = &context.slots[first];
            let right = &context.slots[second];
            if left.offensive_position == right.offensive_position
                && left.defensive_position == right.defensive_position
                && left.role == right.role
            {
                continue;
            }
            let Some(a) = context.player(left.player_id) else {
                continue;
            };
            let Some(b) = context.player(right.player_id) else {
                continue;
            };
            if !eligible(context, a, second) || !eligible(context, b, first) {
                continue;
            }
            let static_gain = a.fits[second].quality + b.fits[first].quality
                - a.fits[first].quality
                - b.fits[second].quality;
            let tactical_gain = need_fit(context, a, right, diagnosis)
                + need_fit(context, b, left, diagnosis)
                - need_fit(context, a, left, diagnosis)
                - need_fit(context, b, right, diagnosis);
            if static_gain < 0.02 && tactical_gain < 0.06 {
                continue;
            }
            if a.fits[second].quality - a.fits[first].quality < -0.08
                || b.fits[first].quality - b.fits[second].quality < -0.08
            {
                continue;
            }
            let Some(pa) = perceptions.iter().find(|p| p.id == a.id) else {
                continue;
            };
            let Some(pb) = perceptions.iter().find(|p| p.id == b.id) else {
                continue;
            };
            let gain = contribution(a, pa, second, false) + contribution(b, pb, first, false)
                - contribution(a, pa, first, false)
                - contribution(b, pb, second, false);
            let change_cost = 0.015
                + (1.0 - context.skills.adaptability) * 0.015
                + (1.0 - context.skills.knowledge) * 0.01
                + continuity_cost(left, a)
                + continuity_cost(right, b)
                + (0.8 - a.fits[second].proficiency).max(0.0) * 0.05
                + (0.8 - b.fits[first].proficiency).max(0.0) * 0.05;
            candidates.push(RealignmentCandidate {
                intent: TacticalRealignmentIntent::new(a.id, b.id),
                utility: gain + tactical_gain * 0.12 - change_cost,
                contribution_gain: gain,
                tactical_gain,
                change_cost,
            });
        }
    }
    candidates.sort_by(|a, b| {
        b.utility
            .total_cmp(&a.utility)
            .then_with(|| a.intent.first_player_id().cmp(&b.intent.first_player_id()))
            .then_with(|| {
                a.intent
                    .second_player_id()
                    .cmp(&b.intent.second_player_id())
            })
    });
    candidates.truncate(2 + (context.skills.adaptability * 4.0).round() as usize);
    candidates
}

fn eligible(context: &ManagerDecisionContext, player: &PlayerContext, destination: usize) -> bool {
    let position = context.slots[destination].offensive_position;
    let minimum = if matches!(
        position,
        Position::Artrine | Position::Passer | Position::Goalguard
    ) {
        0.6
    } else {
        0.4
    };
    player.active
        && !player.injured
        && player.fits[destination].proficiency >= minimum
        && !player
            .entered_at
            .is_some_and(|last| context.elapsed - last < 600.0)
}

fn continuity_cost(slot: &SlotContext, player: &PlayerContext) -> f64 {
    (if matches!(
        slot.offensive_position,
        Position::Artrine | Position::Passer | Position::Goalguard
    ) || slot.role == SlotRole::Kicker
    {
        0.025
    } else {
        0.0
    }) + if player.captain { 0.01 } else { 0.0 }
}
