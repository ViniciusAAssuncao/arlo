use crate::error::{EngineError, EngineResult};
use crate::input::{MatchInput, TeamInput};
use crate::state::MatchState;
use arlo_domain::{AttributeKey, Player, Position, PositionLine};
use std::collections::HashMap;
use uuid::Uuid;

pub(super) struct RatingIndex {
    attribute_ids: HashMap<AttributeKey, Uuid>,
    active_by_team: HashMap<Uuid, Vec<Uuid>>,
}

impl RatingIndex {
    pub(super) fn new(input: &MatchInput, state: &MatchState) -> Self {
        let attribute_ids = input
            .player_attribute_definitions()
            .iter()
            .map(|definition| (definition.key(), definition.id()))
            .collect();
        let active_by_team = HashMap::from([
            (state.home().team_id(), state.home().active_player_ids().to_vec()),
            (state.away().team_id(), state.away().active_player_ids().to_vec()),
        ]);
        Self { attribute_ids, active_by_team }
    }

    pub(super) fn is_active(&self, team: &TeamInput, player_id: Uuid) -> bool {
        self.active_by_team.get(&team.team_id()).is_some_and(|ids| ids.contains(&player_id))
    }

    fn value(&self, player: &Player, key: AttributeKey) -> EngineResult<f64> {
        let attribute_id = self.attribute_ids.get(&key).ok_or_else(|| {
            EngineError::InvalidInput(format!("missing attribute definition: {key:?}"))
        })?;
        let value = player
            .attributes()
            .iter()
            .find(|value| value.attribute_definition_id() == *attribute_id)
            .ok_or_else(|| {
                EngineError::InvalidInput(format!("player {} lacks {key:?}", player.id()))
            })?;
        Ok(f64::from(value.value()))
    }

    fn player<'a>(&self, team: &'a TeamInput, player_id: Uuid) -> EngineResult<&'a Player> {
        team.roster()
            .iter()
            .find(|player| player.id() == player_id)
            .ok_or_else(|| EngineError::InvalidInput("active player is missing from roster".into()))
    }

    pub(super) fn player_value(
        &self,
        team: &TeamInput,
        player_id: Uuid,
        key: AttributeKey,
    ) -> EngineResult<f64> {
        self.value(self.player(team, player_id)?, key)
    }

    pub(super) fn specialist(
        &self,
        team: &TeamInput,
        position: Position,
        key: AttributeKey,
    ) -> EngineResult<f64> {
        let player_id = team
            .lineup()
            .assignments()
            .iter()
            .find(|assignment| assignment.position() == position && self.is_active(team, assignment.player_id()))
            .or_else(|| team.lineup().assignments().iter().find(|assignment| self.is_active(team, assignment.player_id())))
            .ok_or_else(|| EngineError::InvalidInput("lineup lacks a required specialist".into()))?
            .player_id();
        self.value(self.player(team, player_id)?, key)
    }

    pub(super) fn active_average(
        &self,
        team: &TeamInput,
        key: AttributeKey,
        exclude_specialists: bool,
    ) -> EngineResult<f64> {
        let mut total = 0.0;
        let mut count = 0u32;
        for assignment in team.lineup().assignments() {
            if !self.is_active(team, assignment.player_id()) || assignment.position() == Position::Goalguard
                || (exclude_specialists
                    && matches!(assignment.position(), Position::Artrine | Position::Passer))
            {
                continue;
            }
            total += self.value(self.player(team, assignment.player_id())?, key)?;
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
        for assignment in team.lineup().assignments() {
            if !self.is_active(team, assignment.player_id()) || assignment.position().line() != PositionLine::OffensiveLine {
                continue;
            }
            let player_id = assignment.player_id();
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
