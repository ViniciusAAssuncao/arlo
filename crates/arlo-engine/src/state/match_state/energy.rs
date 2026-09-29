use super::MatchState;
use crate::error::EngineResult;
use crate::input::{MatchInput, TeamInput};
use arlo_domain::{AttributeKey, Position};
use arlo_events::{MatchEvent, MatchEventEnvelope, PhysicalStrainRecorded};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub(super) struct EnergyProfile {
    capacity: f64,
    work: f64,
}

impl MatchState {
    pub fn player_energy(&self, player_id: Uuid) -> f64 {
        self.energy.get(&player_id).copied().unwrap_or(1.0)
    }

    pub(crate) fn reduce_energy(&mut self, player_id: Uuid, amount: f64) {
        if let Some(energy) = self.energy.get_mut(&player_id) {
            *energy = (*energy - amount).clamp(0.0, 1.0);
        }
    }

    pub(crate) fn record_segment_energy(
        &mut self,
        input: &MatchInput,
        prior: &Self,
        events: &mut Vec<MatchEventEnvelope>,
    ) -> EngineResult<()> {
        let elapsed =
            (self.clock.total_elapsed_seconds() - prior.clock.total_elapsed_seconds()).max(0.0);
        if elapsed == 0.0 {
            return Ok(());
        }
        let mut action_load: HashMap<Uuid, f64> = HashMap::new();
        for envelope in events.iter() {
            match envelope.event() {
                MatchEvent::CarryResolved(event) => {
                    *action_load.entry(event.carrier_id()).or_default() += 0.00065;
                }
                MatchEvent::DuelResolved(event) => {
                    for id in event.attacker_ids().iter().chain(event.defender_ids()) {
                        *action_load.entry(*id).or_default() += 0.0009;
                    }
                }
                MatchEvent::PassCompleted(event) => {
                    *action_load.entry(event.passer_id()).or_default() += 0.00022;
                    *action_load.entry(event.receiver_id()).or_default() += 0.00022;
                }
                _ => {}
            }
        }
        for (team, active) in [
            (input.home(), prior.home.active_player_ids()),
            (input.away(), prior.away.active_player_ids()),
        ] {
            for &player_id in active {
                self.energy_participants.insert(player_id);
                let position = assigned_position(team, prior, player_id);
                let profile = prior.energy_profiles.get(&player_id);
                let capacity = profile.map_or(1.0, |profile| profile.capacity);
                let work = profile.map_or(1.0, |profile| profile.work);
                let team_state = if team.team_id() == prior.home.team_id() {
                    &prior.home
                } else {
                    &prior.away
                };
                let activity = team
                    .lineup()
                    .assignments()
                    .iter()
                    .find(|assignment| {
                        team_state.slot_player_id(assignment.player_id()) == player_id
                    })
                    .map(|assignment| {
                        let attack = assignment
                            .player_instructions()
                            .in_possession()
                            .involvement_priority()
                            .value();
                        let defense = assignment
                            .player_instructions()
                            .out_of_possession()
                            .engagement_bias()
                            .value();
                        0.78 + 0.24 * attack + 0.20 * defense
                    })
                    .unwrap_or(1.0);
                let tempo = team
                    .tactics()
                    .instructions()
                    .in_possession()
                    .tempo()
                    .value();
                let pressing = team
                    .tactics()
                    .instructions()
                    .out_of_possession()
                    .pressing_intensity()
                    .value();
                let tactical_load = 0.78 + 0.24 * tempo + 0.20 * pressing;
                let role = match position {
                    Position::Goalguard => 0.30,
                    Position::CenterOffense => 0.85,
                    Position::Midcenter => 1.30,
                    Position::WingOffense | Position::TightWing | Position::Corridor => 1.15,
                    Position::Artrine | Position::Passer => 1.05,
                    Position::PassRusher | Position::RunningEnd | Position::Lineback => 1.20,
                    Position::WideEnd | Position::Fullback | Position::CenterTight => 1.05,
                    _ => 0.90,
                };
                let load = (elapsed * 0.000055 * role * work * activity * tactical_load
                    + action_load.get(&player_id).copied().unwrap_or(0.0))
                    / capacity;
                self.reduce_energy(player_id, load);
            }
        }
        Ok(())
    }

    pub(crate) fn record_final_energy(
        &mut self,
        events: &mut Vec<MatchEventEnvelope>,
    ) -> EngineResult<()> {
        let mut players: Vec<_> = self.energy_participants.iter()
            .map(|&id| (id, self.player_energy(id))).collect();
        players.sort_by_key(|(id, _)| *id);
        for (player_id, energy) in players {
            events.push(self.emit(MatchEvent::PhysicalStrainRecorded(
                PhysicalStrainRecorded::new(player_id, energy, 1.0, 0.0),
            ))?);
        }
        Ok(())
    }
}

fn assigned_position(team: &TeamInput, state: &MatchState, player_id: Uuid) -> Position {
    let team_state = if team.team_id() == state.home.team_id() {
        &state.home
    } else {
        &state.away
    };
    team.lineup()
        .assignments()
        .iter()
        .find(|assignment| team_state.slot_player_id(assignment.player_id()) == player_id)
        .map(|assignment| assignment.position())
        .unwrap_or(Position::Midcenter)
}

fn attribute(input: &MatchInput, team: &TeamInput, player_id: Uuid, key: AttributeKey) -> f64 {
    let definition = input
        .player_attribute_definitions()
        .iter()
        .find(|definition| definition.key() == key);
    team.roster()
        .iter()
        .find(|player| player.id() == player_id)
        .and_then(|player| {
            definition.and_then(|definition| {
                player
                    .attributes()
                    .iter()
                    .find(|value| value.attribute_definition_id() == definition.id())
            })
        })
        .map(|value| f64::from(value.value()))
        .unwrap_or(10.0)
}

pub(super) fn initial_profiles(input: &MatchInput) -> HashMap<Uuid, EnergyProfile> {
    input.home().roster().iter().map(|player| (input.home(), player.id()))
        .chain(input.away().roster().iter().map(|player| (input.away(), player.id())))
        .map(|(team, player_id)| {
            let stamina = attribute(input, team, player_id, AttributeKey::Stamina);
            let fitness = attribute(input, team, player_id, AttributeKey::NaturalFitness);
            let work_rate = attribute(input, team, player_id, AttributeKey::WorkRate);
            (player_id, EnergyProfile {
                capacity: (0.72 + 0.018 * stamina + 0.010 * fitness).clamp(0.75, 1.30),
                work: (0.82 + 0.018 * work_rate).clamp(0.84, 1.18),
            })
        }).collect()
}
