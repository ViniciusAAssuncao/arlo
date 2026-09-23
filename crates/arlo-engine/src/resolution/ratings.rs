use crate::error::{EngineError, EngineResult};
use crate::input::{MatchInput, TeamInput};
use arlo_domain::{AttributeKey, Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

pub(super) struct RatingIndex {
    attribute_ids: HashMap<AttributeKey, Uuid>,
}

impl RatingIndex {
    pub(super) fn new(input: &MatchInput) -> Self {
        let attribute_ids = input
            .player_attribute_definitions()
            .iter()
            .map(|definition| (definition.key(), definition.id()))
            .collect();
        Self { attribute_ids }
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

    fn specialist(
        &self,
        team: &TeamInput,
        position: Position,
        key: AttributeKey,
    ) -> EngineResult<f64> {
        let player_id = team
            .lineup()
            .assignments()
            .iter()
            .find(|assignment| assignment.position() == position)
            .ok_or_else(|| EngineError::InvalidInput("lineup lacks a required specialist".into()))?
            .player_id();
        self.value(self.player(team, player_id)?, key)
    }

    fn active_average(
        &self,
        team: &TeamInput,
        key: AttributeKey,
        exclude_specialists: bool,
    ) -> EngineResult<f64> {
        let mut total = 0.0;
        let mut count = 0u32;
        for assignment in team.lineup().assignments() {
            if assignment.position() == Position::Goalguard
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
        let offense = 0.25 * self.specialist(team, Position::Passer, AttributeKey::Passing)?
            + 0.25 * self.specialist(team, Position::Passer, AttributeKey::Decisions)?
            + 0.25 * self.specialist(team, Position::Artrine, AttributeKey::ArloControl)?
            + 0.25 * self.active_average(team, AttributeKey::OffensiveBlocking, true)?;
        let defense = 0.50
            * self.active_average(team, AttributeKey::DefensiveContainment, false)?
            + 0.25 * self.active_average(team, AttributeKey::PasserPressure, false)?
            + 0.25 * self.active_average(team, AttributeKey::Pace, false)?;
        Ok(TeamRatings { offense, defense })
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct TeamRatings {
    pub offense: f64,
    pub defense: f64,
}
