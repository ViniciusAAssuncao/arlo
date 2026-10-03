use super::context::{ManagerDecisionContext, PlayerContext, SlotContext};
use super::diagnosis::{attacking_slot, PlayerDiagnosis, TeamDiagnosis};
use super::perception::PlayerPerception;
use arlo_domain::{Position, SlotRole};

pub(super) fn contribution(
    player: &PlayerContext,
    perceived: &PlayerPerception,
    slot_index: usize,
    incoming: bool,
) -> f64 {
    let physical =
        (0.58 + player.energy * 0.42) * (0.72 + player.morale.clamp(0.0, 100.0) * 0.0028);
    let injury = if player.injured { 0.72 } else { 1.0 };
    let settling = if incoming { 0.92 } else { player.settling };
    player.fits[slot_index].quality * physical * injury * settling
        + perceived.deviation * if incoming { 0.08 } else { 0.18 }
}

pub(super) fn change_cost(
    context: &ManagerDecisionContext,
    outgoing: &PlayerContext,
    incoming: &PlayerContext,
    slot_index: usize,
    diagnosis: &PlayerDiagnosis,
) -> f64 {
    let slot = &context.slots[slot_index];
    let strongest = context
        .players
        .iter()
        .filter(|player| player.active || player.available)
        .map(|player| player.fits[slot_index].quality)
        .fold(0.0, f64::max);
    let quality = outgoing.fits[slot_index].quality;
    let structural = if matches!(
        slot.offensive_position,
        Position::Artrine | Position::Passer | Position::Goalguard
    ) || slot.role == SlotRole::Kicker
    {
        0.045
    } else {
        0.0
    };
    let importance = if outgoing.starter { 0.015 } else { 0.0 }
        + if outgoing.captain { 0.015 } else { 0.0 }
        + if quality >= strongest - 0.04 {
            0.030
        } else {
            0.0
        };
    let patient = 0.015 + context.skills.patience * 0.015;
    let core_cost = (structural + importance)
        * (1.0 - diagnosis.fatigue * 0.80)
        * (1.0 - diagnosis.individual * 0.65);
    let unfamiliarity = (0.8 - incoming.fits[slot_index].proficiency).max(0.0) * 0.08;
    let return_cost = incoming.exited_at.map_or(0.0, |exit| {
        (1.0 - (context.elapsed - exit).max(0.0) / 1800.0).clamp(0.0, 1.0) * 0.10
    });
    let recent_plan = context.last_tactical_switch_at.map_or(0.0, |last| {
        (1.0 - (context.elapsed - last).max(0.0) / 900.0).clamp(0.0, 1.0) * 0.025
    });
    patient + core_cost + unfamiliarity + return_cost + recent_plan
}

pub(super) fn need_fit(
    context: &ManagerDecisionContext,
    player: &PlayerContext,
    slot: &SlotContext,
    team: &TeamDiagnosis,
) -> f64 {
    let skills = context.skills;
    let attack = if attacking_slot(slot) {
        team.attack * player.attack * (0.3 + skills.offense * 0.7)
    } else {
        0.0
    };
    let control = if matches!(
        slot.offensive_position,
        Position::Artrine | Position::Passer
    ) {
        team.control * player.control * (0.3 + skills.artro * 0.7)
    } else {
        0.0
    };
    let defense = team.defense * player.defense * (0.3 + skills.defense * 0.7);
    attack
        + control
        + defense
        + team.security * player.security
        + team.discipline * player.discipline
}
