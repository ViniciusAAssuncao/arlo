use super::context::{ManagerDecisionContext, PlayerContext};
use super::diagnosis::{diagnose_player, PlayerDiagnosis, TeamDiagnosis};
use super::perception::PlayerPerception;
use super::valuation::{change_cost, contribution, need_fit};
use arlo_domain::Position;
use arlo_events::SubstitutionReason;
use arlo_manager_control::SubstitutionIntent;

#[derive(Debug, Clone)]
pub struct SubstitutionCandidate {
    pub intent: SubstitutionIntent,
    pub reason: SubstitutionReason,
    pub utility: f64,
    pub contribution_gain: f64,
    pub individual_need: f64,
    pub fatigue_need: f64,
    pub change_cost: f64,
}

pub(super) fn generate(
    context: &ManagerDecisionContext,
    perceptions: &[PlayerPerception],
    team: &TeamDiagnosis,
) -> Vec<SubstitutionCandidate> {
    let mut candidates = Vec::new();
    if context
        .last_plan_activation_at
        .is_some_and(|last| context.elapsed - last < 900.0)
    {
        return candidates;
    }
    if context
        .last_substitution_at
        .is_some_and(|last| context.elapsed - last < 900.0)
        || context
            .last_realignment_at
            .is_some_and(|last| context.elapsed - last < 600.0)
    {
        return candidates;
    }
    let breadth = 2 + (context.skills.adaptability * 2.0).round() as usize;
    for (slot_index, slot) in context.slots.iter().enumerate() {
        let Some(outgoing) = context.player(slot.player_id) else {
            continue;
        };
        if outgoing.injured {
            continue;
        }
        let Some(observed) = perceptions.iter().find(|player| player.id == outgoing.id) else {
            continue;
        };
        let diagnosis = diagnose_player(context, outgoing, slot, observed, team);
        if diagnosis.individual < 0.06 && diagnosis.fatigue < 0.12 {
            continue;
        }
        let current = contribution(outgoing, observed, slot_index, false);
        let mut reserves: Vec<_> = context
            .players
            .iter()
            .filter(|player| {
                if !player.available {
                    return false;
                }
                let minimum = if matches!(
                    slot.offensive_position,
                    Position::Artrine | Position::Passer | Position::Goalguard
                ) {
                    0.6
                } else {
                    0.4
                };
                player.fits[slot_index].proficiency >= minimum
            })
            .collect();
        let quick_value = |player: &PlayerContext| {
            player.fits[slot_index].quality * (0.58 + player.energy * 0.42)
                + need_fit(context, player, slot, team) * 0.10
        };
        reserves.sort_by(|left, right| {
            quick_value(right)
                .total_cmp(&quick_value(left))
                .then_with(|| left.id.cmp(&right.id))
        });
        for incoming in reserves.into_iter().take(breadth) {
            let Some(incoming_perception) =
                perceptions.iter().find(|player| player.id == incoming.id)
            else {
                continue;
            };
            let gain = contribution(incoming, incoming_perception, slot_index, true) - current;
            if gain < -0.10 {
                continue;
            }
            let physical_gain = (incoming.energy - outgoing.energy).max(0.0);
            let relief = diagnosis.individual * 0.16
                + diagnosis.fatigue * physical_gain * (0.06 + context.skills.load * 0.08);
            let tactical = (need_fit(context, incoming, slot, team)
                - need_fit(context, outgoing, slot, team))
                * (0.06 + context.skills.knowledge * 0.10);
            let cost = change_cost(context, outgoing, incoming, slot_index, &diagnosis);
            let uncertainty = diagnosis.individual * (1.0 - observed.reliability) * 0.025;
            let restoration = if incoming.starter && incoming.exited_at.is_some() {
                0.015 * (team.attack + team.control).min(1.0)
            } else {
                0.0
            };
            let utility = gain + relief + tactical + restoration - cost - uncertainty;
            if !utility.is_finite() {
                continue;
            }
            candidates.push(SubstitutionCandidate {
                intent: SubstitutionIntent::new(outgoing.id, incoming.id),
                reason: reason(&diagnosis),
                utility,
                contribution_gain: gain,
                individual_need: diagnosis.individual,
                fatigue_need: diagnosis.fatigue,
                change_cost: cost,
            });
        }
    }
    candidates.sort_by(|left, right| {
        right
            .utility
            .total_cmp(&left.utility)
            .then_with(|| {
                left.intent
                    .outgoing_player_id()
                    .cmp(&right.intent.outgoing_player_id())
            })
            .then_with(|| {
                left.intent
                    .incoming_player_id()
                    .cmp(&right.intent.incoming_player_id())
            })
    });
    candidates
}

fn reason(diagnosis: &PlayerDiagnosis) -> SubstitutionReason {
    if diagnosis.fatigue >= diagnosis.individual {
        SubstitutionReason::Fatigue
    } else if diagnosis.discipline > diagnosis.individual * 0.8 {
        SubstitutionReason::Disciplinary
    } else {
        SubstitutionReason::Tactical
    }
}
