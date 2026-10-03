use super::context::{ManagerDecisionContext, PlayerContext};
use super::random::sample;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) struct PlayerPerception {
    pub id: Uuid,
    pub reliability: f64,
    pub deviation: f64,
    pub execution: f64,
    pub production: f64,
    pub defense: f64,
    pub security: f64,
    pub discipline: f64,
    pub opportunity_rate: f64,
    pub fatigue: f64,
}

pub(super) fn perceive(context: &ManagerDecisionContext) -> Vec<PlayerPerception> {
    context
        .players
        .iter()
        .map(|player| perceive_player(context, player))
        .collect()
}

fn perceive_player(context: &ManagerDecisionContext, player: &PlayerContext) -> PlayerPerception {
    let evidence = player.evidence;
    let exposure = (evidence.opportunities / 18.0).clamp(0.0, 1.0)
        * (evidence.seconds_played / 600.0).clamp(0.0, 1.0);
    let reliability = evidence.confidence * exposure;
    let fidelity = 0.25 + 0.75 * context.skills.judging;
    let stable_error = (sample(context, player.id, 0x7065726365707469) - 0.5)
        * (1.0 - context.skills.judging)
        * 0.12;
    let pressure_error = (sample(context, player.id, context.sequence ^ 0x7072657373757265) - 0.5)
        * context.pressure()
        * (1.0 - context.skills.composure)
        * 0.06;
    let error = (stable_error + pressure_error) * reliability;
    let observed = |signal: f64| (signal * fidelity * reliability + error).clamp(-1.0, 1.0);
    let freshness = if player.active {
        1.0
    } else {
        player.exited_at.map_or(1.0, |exit| {
            (1.0 - (context.elapsed - exit).max(0.0) / 2400.0).clamp(0.25, 1.0)
        })
    };
    PlayerPerception {
        id: player.id,
        reliability,
        deviation: observed(evidence.deviation) * freshness,
        execution: observed(evidence.execution) * freshness,
        production: observed(evidence.production) * freshness,
        defense: observed(evidence.defense) * freshness,
        security: observed(evidence.security) * freshness,
        discipline: observed(evidence.discipline) * freshness,
        opportunity_rate: evidence.opportunities / (evidence.seconds_played / 60.0).max(1.0),
        fatigue: ((0.55 + context.skills.load * 0.12 - player.energy) / 0.50).clamp(0.0, 1.0),
    }
}
