use crate::world_state::match_state::teams::TeamRegistry;
use arlo_domain::{AttributeKey, CaptaincyRole, Player};
use std::collections::HashMap;
use uuid::Uuid;

impl TeamRegistry {
    pub fn home_captain_id(&self) -> Option<Uuid> {
        self.home_lineup
            .assignments()
            .iter()
            .find(|a| a.player().captaincy_role() == Some(CaptaincyRole::Captain))
            .map(|a| a.player().id())
            .or_else(|| {
                self.home_lineup
                    .assignments()
                    .iter()
                    .find(|a| a.player().captaincy_role() == Some(CaptaincyRole::ViceCaptain))
                    .map(|a| a.player().id())
            })
            .or_else(|| self.home_lineup.assignments().first().map(|a| a.player().id()))
    }

    pub fn away_captain_id(&self) -> Option<Uuid> {
        self.away_lineup
            .assignments()
            .iter()
            .find(|a| a.player().captaincy_role() == Some(CaptaincyRole::Captain))
            .map(|a| a.player().id())
            .or_else(|| {
                self.away_lineup
                    .assignments()
                    .iter()
                    .find(|a| a.player().captaincy_role() == Some(CaptaincyRole::ViceCaptain))
                    .map(|a| a.player().id())
            })
            .or_else(|| self.away_lineup.assignments().first().map(|a| a.player().id()))
    }

    pub fn home_captain<'a>(
        &'a self,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        let players: Vec<&Player> = self
            .home_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();
        crate::psychology::systems::baseline::find_active_captain(&players, attribute_keys)
    }

    pub fn away_captain<'a>(
        &'a self,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        let players: Vec<&Player> = self
            .away_lineup
            .assignments()
            .iter()
            .map(|a| a.player())
            .collect();
        crate::psychology::systems::baseline::find_active_captain(&players, attribute_keys)
    }

    pub fn team_captain<'a>(
        &'a self,
        team_id: Uuid,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Option<&'a Player> {
        if team_id == self.home_team_id {
            self.home_captain(attribute_keys)
        } else {
            self.away_captain(attribute_keys)
        }
    }
}