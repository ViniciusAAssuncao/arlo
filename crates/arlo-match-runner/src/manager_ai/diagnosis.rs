use super::context::{ManagerDecisionContext, PlayerContext, SlotContext};
use super::perception::PlayerPerception;
use arlo_domain::{Position, PositionLine};

#[derive(Debug, Clone, Copy, Default)]
pub struct TeamDiagnosis {
    pub attack: f64,
    pub defense: f64,
    pub control: f64,
    pub security: f64,
    pub energy: f64,
    pub discipline: f64,
    pub collective_attack: f64,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PlayerDiagnosis {
    pub individual: f64,
    pub fatigue: f64,
    pub discipline: f64,
}

pub(super) fn diagnose_team(
    context: &ManagerDecisionContext,
    perceptions: &[PlayerPerception],
) -> TeamDiagnosis {
    let mut needs = TeamDiagnosis::default();
    let mut attack_count: f64 = 0.0;
    let mut count: f64 = 0.0;
    for slot in &context.slots {
        let Some(observed) = perceptions
            .iter()
            .find(|player| player.id == slot.player_id)
        else {
            continue;
        };
        count += 1.0;
        needs.security += (-observed.security).max(0.0);
        needs.discipline += (-observed.discipline).max(0.0);
        needs.energy += observed.fatigue;
        needs.defense += (-observed.defense).max(0.0);
        if attacking_slot(slot) {
            attack_count += 1.0;
            needs.collective_attack += (-observed.production).max(0.0);
        }
        if slot.offensive_position == Position::Artrine
            || slot.offensive_position == Position::Passer
        {
            needs.control += (-observed.execution).max(0.0) * 0.5;
        }
    }
    let reading = 0.30 + 0.70 * context.skills.adjustments;
    needs.collective_attack = needs.collective_attack / attack_count.max(1.0) * reading;
    needs.security = needs.security / count.max(1.0) * reading;
    needs.discipline = needs.discipline / count.max(1.0) * reading;
    needs.energy /= count.max(1.0);
    needs.defense = needs.defense / count.max(1.0) * reading;
    let chase = (context.deficit / 15.0).clamp(0.0, 1.0) * context.progress.powi(2);
    let protect = (-context.deficit / 15.0).clamp(0.0, 1.0) * context.progress.powi(2);
    needs.attack = (chase + needs.collective_attack).clamp(0.0, 1.0);
    needs.defense = (protect + needs.defense).clamp(0.0, 1.0);
    needs.control = (needs.control * reading + needs.security).clamp(0.0, 1.0);
    needs
}

pub(super) fn diagnose_player(
    context: &ManagerDecisionContext,
    player: &PlayerContext,
    slot: &SlotContext,
    perceived: &PlayerPerception,
    team: &TeamDiagnosis,
) -> PlayerDiagnosis {
    let execution = (-perceived.execution).max(0.0);
    let security = (-perceived.security).max(0.0);
    let defense = (-perceived.defense).max(0.0);
    let discipline = (-perceived.discipline).max(0.0);
    let mut individual = execution * 0.55 + security * 0.35 + discipline * 0.20;
    if slot.defensive_position.line() == PositionLine::DefenseLine
        || slot.defensive_position == Position::Goalguard
    {
        individual += defense * 0.50;
    }
    if attacking_slot(slot) {
        let opportunity_support = (perceived.opportunity_rate / 0.8).clamp(0.0, 1.0);
        individual += (-perceived.production).max(0.0) * execution.min(0.5) * opportunity_support;
        if team.collective_attack > execution && security < 0.08 {
            individual *= 0.65;
        }
    }
    if individual > 0.03 {
        individual += (-perceived.deviation).max(0.0) * 0.25;
    }
    if player
        .entered_at
        .is_some_and(|entered| context.elapsed - entered < 600.0)
    {
        individual *= 0.20;
    }
    PlayerDiagnosis {
        individual: (individual * (0.30 + context.skills.adjustments * 0.70)).clamp(0.0, 1.0),
        fatigue: perceived.fatigue,
        discipline,
    }
}

pub(super) fn attacking_slot(slot: &SlotContext) -> bool {
    slot.offensive_position.line() == PositionLine::OffensiveLine
        || matches!(
            slot.offensive_position,
            Position::Artrine | Position::Passer
        )
}
