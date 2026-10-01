use super::MatchState;
use crate::error::EngineResult;
use crate::input::MatchInput;
use arlo_domain::AttributeKey;
use arlo_domain::InjurySeverityGrade;
use arlo_events::{ImpulseEventKind, ImpulseShiftRecorded, MatchEvent, MatchEventEnvelope};
use std::collections::HashMap;
use uuid::Uuid;

impl MatchState {
    pub fn player_morale(&self, player_id: Uuid) -> f64 {
        self.morale.get(&player_id).copied().unwrap_or(100.0)
    }

    pub(crate) fn record_segment_morale(
        &mut self,
        prior: &Self,
        events: &mut Vec<MatchEventEnvelope>,
    ) -> EngineResult<()> {
        let elapsed = (self.clock.total_elapsed_seconds() - prior.clock.total_elapsed_seconds()).max(0.0);
        let mut shifts: HashMap<Uuid, (f64, ImpulseEventKind)> = HashMap::new();
        for envelope in events.iter() {
            match envelope.event() {
                MatchEvent::DuelResolved(duel) => {
                    for id in duel.attacker_ids() {
                        add(&mut shifts, *id, if duel.attacker_won() { 0.08 } else { -0.08 },
                            if duel.attacker_won() { ImpulseEventKind::DuelWon } else { ImpulseEventKind::DuelLost });
                    }
                    for id in duel.defender_ids() {
                        add(&mut shifts, *id, if duel.attacker_won() { -0.08 } else { 0.08 },
                            if duel.attacker_won() { ImpulseEventKind::DuelLost } else { ImpulseEventKind::DuelWon });
                    }
                }
                MatchEvent::GoalPoint(score) => {
                    add(&mut shifts, score.scorer_id(), 2.4, ImpulseEventKind::ScoreFor);
                    add_team_score(&mut shifts, prior, score.team_id(), 0.25);
                }
                MatchEvent::FieldPoint(score) => {
                    add(&mut shifts, score.scorer_id(), 1.1, ImpulseEventKind::ScoreFor);
                    add_team_score(&mut shifts, prior, score.team_id(), 0.12);
                }
                MatchEvent::FieldGoal(score) => {
                    add(&mut shifts, score.scorer_id(), 0.5, ImpulseEventKind::ScoreFor);
                    add_team_score(&mut shifts, prior, score.team_id(), 0.06);
                }
                MatchEvent::ScoringAttemptMissed(shot) => add(&mut shifts, shot.scorer_id(), -0.25, ImpulseEventKind::BigPlayAllowed),
                MatchEvent::Turnover(turnover) => {
                    if let Some(id) = turnover.lost_by_player_id() {
                        add(&mut shifts, id, -0.65, ImpulseEventKind::TurnoverCommitted);
                    }
                    if let Some(id) = turnover.recovering_player_id() {
                        add(&mut shifts, id, 0.55, ImpulseEventKind::TurnoverWon);
                    }
                }
                _ => {}
            }
        }
        for &player_id in prior.home.active_player_ids().iter().chain(prior.away.active_player_ids()) {
            let energy_used = (prior.player_energy(player_id) - self.player_energy(player_id)).max(0.0);
            let mental_load = elapsed * 0.00008 + energy_used * 4.0;
            if mental_load > 0.0 {
                add(&mut shifts, player_id, -mental_load, ImpulseEventKind::MentalFatigue);
            }
        }
        let mut ordered: Vec<_> = shifts.into_iter().collect();
        ordered.sort_by_key(|(id, _)| *id);
        for (player_id, (delta, kind)) in ordered {
            let previous = self.player_morale(player_id);
            let resilience = self.morale_resilience.get(&player_id).copied().unwrap_or(20.0);
            let modifier = if delta < 0.0 { (1.35 - resilience * 0.0175).clamp(0.65, 1.32) }
                else { (0.8 + resilience * 0.01).clamp(0.8, 1.2) };
            let next = (previous + delta * modifier).clamp(0.0, 120.0);
            self.morale.insert(player_id, next);
            if previous.round() as u8 != next.round() as u8 {
                events.push(self.emit(MatchEvent::ImpulseShiftRecorded(ImpulseShiftRecorded::new(
                    player_id, previous.round() as u8, next.round() as u8, kind, delta.abs(),
                )))?);
            }
        }
        Ok(())
    }

    pub(crate) fn record_final_morale(&mut self, events: &mut Vec<MatchEventEnvelope>) -> EngineResult<()> {
        let mut players: Vec<_> = self.energy_participants.iter().copied().collect();
        players.sort_unstable();
        for player_id in players {
            let value = self.player_morale(player_id).round() as u8;
            events.push(self.emit(MatchEvent::ImpulseShiftRecorded(ImpulseShiftRecorded::new(
                player_id, value, value, ImpulseEventKind::MentalFatigue, 0.0,
            )))?);
        }
        Ok(())
    }

    pub(crate) fn record_injury_morale(
        &mut self,
        player_id: Uuid,
        grade: InjurySeverityGrade,
        events: &mut Vec<MatchEventEnvelope>,
    ) -> EngineResult<()> {
        let previous = self.player_morale(player_id);
        let setback = match grade {
            InjurySeverityGrade::Grade1 => 2.5,
            InjurySeverityGrade::Grade2 => 6.0,
            InjurySeverityGrade::Grade3 => 11.0,
        };
        let next = (previous - setback).max(0.0);
        self.morale.insert(player_id, next);
        events.push(self.emit(MatchEvent::ImpulseShiftRecorded(ImpulseShiftRecorded::new(
            player_id, previous.round() as u8, next.round() as u8,
            ImpulseEventKind::InjurySetback, setback,
        )))?);
        Ok(())
    }

}

fn add(shifts: &mut HashMap<Uuid, (f64, ImpulseEventKind)>, id: Uuid, delta: f64, kind: ImpulseEventKind) {
    let entry = shifts.entry(id).or_insert((0.0, kind));
    entry.0 += delta;
    if delta.abs() > 0.3 {
        entry.1 = kind;
    }
}

fn add_team_score(shifts: &mut HashMap<Uuid, (f64, ImpulseEventKind)>, prior: &MatchState, team_id: Uuid, amount: f64) {
    for player_id in prior.home.active_player_ids() {
        add(shifts, *player_id, if prior.home.team_id() == team_id { amount } else { -amount },
            if prior.home.team_id() == team_id { ImpulseEventKind::ScoreFor } else { ImpulseEventKind::ScoreAgainst });
    }
    for player_id in prior.away.active_player_ids() {
        add(shifts, *player_id, if prior.away.team_id() == team_id { amount } else { -amount },
            if prior.away.team_id() == team_id { ImpulseEventKind::ScoreFor } else { ImpulseEventKind::ScoreAgainst });
    }
}

pub(super) fn initial_resilience(input: &MatchInput) -> HashMap<Uuid, f64> {
    let attribute_ids: Vec<_> = [AttributeKey::Composure, AttributeKey::Determination].iter()
        .map(|key| input.player_attribute_definitions().iter()
            .find(|definition| definition.key() == *key).map(|definition| definition.id())).collect();
    input.home().roster().iter().chain(input.away().roster().iter())
        .map(|player| {
            let total = attribute_ids.iter().map(|id| player.attributes().iter()
                .find(|value| Some(value.attribute_definition_id()) == *id)
                .map(|value| f64::from(value.value())).unwrap_or(10.0)).sum();
            (player.id(), total)
        }).collect()
}
