use crate::error::{EngineError, EngineResult};
use crate::input::player_attribute_index::PlayerAttributeIndex;
use crate::input::{MatchInput, TeamInput};
use crate::state::MatchState;
use arlo_domain::{AttributeKey, Position, PositionLine};
use arlo_tactics::TeamInstructions;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
mod player;
pub(super) use player::current_player_value;
use player::PlayerFactors;

pub(super) struct RatingIndex {
    attributes: Arc<PlayerAttributeIndex>,
    active_by_team: HashMap<Uuid, Vec<Uuid>>,
    slots_by_team: HashMap<Uuid, HashMap<Uuid, Uuid>>,
    player_factors: HashMap<Uuid, PlayerFactors>,
    instructions_by_team: HashMap<Uuid, TeamInstructions>,
    layouts_by_team: HashMap<Uuid, arlo_tactics::TacticalLayout>,
}

impl RatingIndex {
    pub(super) fn new(input: &MatchInput, state: &MatchState) -> Self {
        let active_by_team = HashMap::from([
            (
                state.home().team_id(),
                state.home().active_player_ids().to_vec(),
            ),
            (
                state.away().team_id(),
                state.away().active_player_ids().to_vec(),
            ),
        ]);
        let slots_by_team = HashMap::from([
            (
                input.home().team_id(),
                state
                    .home()
                    .lineup(input.home())
                    .assignments()
                    .iter()
                    .map(|assignment| {
                        (
                            assignment.player_id(),
                            state.home().slot_player_id(assignment.player_id()),
                        )
                    })
                    .collect(),
            ),
            (
                input.away().team_id(),
                state
                    .away()
                    .lineup(input.away())
                    .assignments()
                    .iter()
                    .map(|assignment| {
                        (
                            assignment.player_id(),
                            state.away().slot_player_id(assignment.player_id()),
                        )
                    })
                    .collect(),
            ),
        ]);
        let player_factors = input
            .home()
            .roster()
            .iter()
            .chain(input.away().roster())
            .map(|player| (player.id(), PlayerFactors::new(state, player.id())))
            .collect();
        let instructions_by_team = HashMap::from([
            (
                input.home().team_id(),
                state.team_instructions(input.home()),
            ),
            (
                input.away().team_id(),
                state.team_instructions(input.away()),
            ),
        ]);
        let layouts_by_team = HashMap::from([
            (
                input.home().team_id(),
                state.home().tactical_layout(input.home()),
            ),
            (
                input.away().team_id(),
                state.away().tactical_layout(input.away()),
            ),
        ]);
        Self {
            layouts_by_team,
            attributes: input.shared_player_attributes(),
            active_by_team,
            slots_by_team,
            player_factors,
            instructions_by_team,
        }
    }

    pub(super) fn lineup(&self, team: &TeamInput) -> &arlo_tactics::TacticalLineup {
        &self.layouts_by_team[&team.team_id()].lineup
    }
    pub(super) fn formation(&self, team: &TeamInput) -> &arlo_domain::Formation {
        &self.layouts_by_team[&team.team_id()].formation
    }

    pub(super) fn instructions(&self, team: &TeamInput) -> TeamInstructions {
        self.instructions_by_team
            .get(&team.team_id())
            .copied()
            .unwrap_or(*team.tactics().instructions())
    }

    pub(super) fn slot_player_id(&self, team: &TeamInput, original_id: Uuid) -> Uuid {
        self.slots_by_team
            .get(&team.team_id())
            .and_then(|slots| slots.get(&original_id))
            .copied()
            .unwrap_or(original_id)
    }

    pub(super) fn is_active_slot(&self, team: &TeamInput, original_id: Uuid) -> bool {
        self.is_active(team, self.slot_player_id(team, original_id))
    }

    pub(super) fn is_active(&self, team: &TeamInput, player_id: Uuid) -> bool {
        self.active_by_team
            .get(&team.team_id())
            .is_some_and(|ids| ids.contains(&player_id))
    }

    pub(super) fn player_value(
        &self,
        team: &TeamInput,
        player_id: Uuid,
        key: AttributeKey,
    ) -> EngineResult<f64> {
        let values = self
            .attributes
            .player_for_team(team.team_id(), player_id)
            .ok_or_else(|| {
                EngineError::InvalidInput("active player is missing from roster".into())
            })?;
        if !self.attributes.is_defined(key) {
            return Err(EngineError::InvalidInput(format!(
                "missing attribute definition: {key:?}"
            )));
        }
        let value = values[key.index()].ok_or_else(|| {
            EngineError::InvalidInput(format!("player {player_id} lacks {key:?}"))
        })?;
        Ok(self.player_factors[&player_id].apply(value))
    }

    pub(super) fn reliable_probability(
        &self,
        player_id: Uuid,
        probability: f64,
        ceiling: f64,
    ) -> f64 {
        let morale = self
            .player_factors
            .get(&player_id)
            .map_or(100.0, |factors| factors.morale);
        let gain = ((morale - 100.0).max(0.0) * 0.001 * (1.0 - probability)).min(0.02);
        (probability + gain).min(ceiling)
    }

    pub(super) fn specialist(
        &self,
        team: &TeamInput,
        position: Position,
        key: AttributeKey,
    ) -> EngineResult<f64> {
        let player_id = self
            .lineup(team)
            .assignments()
            .iter()
            .find(|assignment| {
                assignment.position() == position
                    && self.is_active_slot(team, assignment.player_id())
            })
            .or_else(|| {
                self.lineup(team)
                    .assignments()
                    .iter()
                    .find(|assignment| self.is_active_slot(team, assignment.player_id()))
            })
            .ok_or_else(|| EngineError::InvalidInput("lineup lacks a required specialist".into()))?
            .player_id();
        let player_id = self.slot_player_id(team, player_id);
        self.player_value(team, player_id, key)
    }

    pub(super) fn active_average(
        &self,
        team: &TeamInput,
        key: AttributeKey,
        exclude_specialists: bool,
    ) -> EngineResult<f64> {
        let mut total = 0.0;
        let mut count = 0u32;
        for assignment in self.lineup(team).assignments() {
            if !self.is_active_slot(team, assignment.player_id())
                || assignment.position() == Position::Goalguard
                || (exclude_specialists
                    && matches!(assignment.position(), Position::Artrine | Position::Passer))
            {
                continue;
            }
            total +=
                self.player_value(team, self.slot_player_id(team, assignment.player_id()), key)?;
            count += 1;
        }
        if count == 0 {
            return Err(EngineError::InvalidInput(
                "lineup has no eligible outfield players".into(),
            ));
        }
        Ok(total / f64::from(count))
    }

    pub(super) fn team_ratings(&self, team: &TeamInput) -> EngineResult<TeamRatings> {
        let offense = 0.20 * self.specialist(team, Position::Passer, AttributeKey::Passing)?
            + 0.20 * self.specialist(team, Position::Passer, AttributeKey::Decisions)?
            + 0.20 * self.specialist(team, Position::Artrine, AttributeKey::ArloControl)?
            + 0.20 * self.active_average(team, AttributeKey::OffensiveBlocking, true)?
            + 0.20 * self.frontline_attack(team)?;
        let defense = 0.50
            * self.active_average(team, AttributeKey::DefensiveContainment, false)?
            + 0.25 * self.active_average(team, AttributeKey::PasserPressure, false)?
            + 0.25 * self.active_average(team, AttributeKey::Pace, false)?;
        Ok(TeamRatings { offense, defense })
    }

    fn frontline_attack(&self, team: &TeamInput) -> EngineResult<f64> {
        let mut total = 0.0;
        let mut count = 0u32;
        for assignment in self.lineup(team).assignments() {
            if !self.is_active_slot(team, assignment.player_id())
                || assignment.position().line() != PositionLine::OffensiveLine
            {
                continue;
            }
            let player_id = self.slot_player_id(team, assignment.player_id());
            total += 0.45 * self.player_value(team, player_id, AttributeKey::Finishing)?
                + 0.30 * self.player_value(team, player_id, AttributeKey::Positioning)?
                + 0.25 * self.player_value(team, player_id, AttributeKey::Anticipation)?;
            count += 1;
        }
        if count == 0 {
            return Ok(
                0.45 * self.active_average(team, AttributeKey::Finishing, true)?
                    + 0.30 * self.active_average(team, AttributeKey::Positioning, true)?
                    + 0.25 * self.active_average(team, AttributeKey::Anticipation, true)?,
            );
        }
        Ok(total / f64::from(count))
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TeamRatings {
    pub offense: f64,
    pub defense: f64,
}
