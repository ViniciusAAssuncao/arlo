use crate::Result;
use arlo_controller::services::season::matchday::PreparedMatchdayFixture;
use arlo_domain::{
    AttributeDefinition, AttributeKey, Formation, Manager, MatchFormatRules, Pitch, Player, Referee,
};
use arlo_engine::{MatchInput, TeamInput};
use arlo_persistence::persister::MatchPersistenceContext;
#[cfg(not(feature = "pre-epic"))]
use arlo_tactics::PreparedTacticalPlan;
use arlo_tactics::{PlayCall, TacticalLineup, TeamTacticalProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CapturedFixture {
    pub fixture_id: String,
    pub match_id: Uuid,
    home: CapturedTeam,
    away: CapturedTeam,
    format: MatchFormatRules,
    pitch: Pitch,
    referees: Vec<Referee>,
    definitions: Vec<AttributeDefinition>,
    manager_keys: HashMap<Uuid, AttributeKey>,
    energy: HashMap<Uuid, f64>,
    morale: HashMap<Uuid, f64>,
    seed: u64,
    pub calls: Vec<PlayCall>,
    venue_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
struct CapturedTeam {
    id: Uuid,
    formation: Formation,
    lineup: TacticalLineup,
    roster: Vec<Player>,
    manager: Manager,
    profiles: Vec<TeamTacticalProfile>,
    #[cfg(not(feature = "pre-epic"))]
    plans: Vec<PreparedTacticalPlan>,
}

impl CapturedTeam {
    fn capture(team: &TeamInput) -> Self {
        Self {
            id: team.team_id(),
            formation: team.formation().clone(),
            lineup: team.lineup().clone(),
            roster: team.roster().to_vec(),
            manager: team.manager().clone(),
            profiles: team.tactical_profiles().cloned().collect(),
            #[cfg(not(feature = "pre-epic"))]
            plans: team.prepared_plans().to_vec(),
        }
    }

    fn restore(&self) -> Result<TeamInput> {
        let team = TeamInput::new(
            self.id,
            self.formation.clone(),
            self.lineup.clone(),
            self.roster.clone(),
            self.manager.clone(),
            self.profiles[0].clone(),
        )?
        .with_alternative_tactics(self.profiles[1..].to_vec())?;
        #[cfg(not(feature = "pre-epic"))]
        let team = team.with_prepared_plans(self.plans.clone())?;
        Ok(team)
    }
}

impl CapturedFixture {
    pub fn capture(prepared: &PreparedMatchdayFixture) -> Self {
        let input = &prepared.input;
        let ids: Vec<_> = input
            .home()
            .roster()
            .iter()
            .chain(input.away().roster())
            .map(Player::id)
            .collect();
        Self {
            fixture_id: prepared.fixture_row.id.clone(),
            match_id: input.match_id(),
            home: CapturedTeam::capture(input.home()),
            away: CapturedTeam::capture(input.away()),
            format: input.format(),
            pitch: input.pitch(),
            referees: input.referees().to_vec(),
            definitions: input.player_attribute_definitions().to_vec(),
            manager_keys: input.manager_attribute_keys().clone(),
            energy: ids
                .iter()
                .map(|id| (*id, input.player_start_energy(*id)))
                .collect(),
            morale: ids
                .iter()
                .map(|id| (*id, input.player_start_morale(*id)))
                .collect(),
            seed: input.seed(),
            calls: prepared.play_calls.clone(),
            venue_id: prepared.persistence_context.venue_id,
        }
    }

    pub fn restore(
        &self,
        catalogs: &arlo_controller::services::season::matchday::MatchdayCatalogs,
    ) -> Result<MatchInput> {
        Ok(MatchInput::new(
            self.match_id,
            self.home.restore()?,
            self.away.restore()?,
            self.format,
            self.pitch,
            self.referees.clone(),
            catalogs.attribute_keys_by_id.clone(),
            catalogs.fault_catalog.clone(),
            catalogs.injury_catalog.clone(),
            self.definitions.clone(),
            self.seed,
        )?
        .with_manager_decision_context(Arc::new(self.manager_keys.clone()), self.energy.clone())?
        .with_player_start_morale(self.morale.clone())?)
    }

    pub fn persistence_context(&self) -> MatchPersistenceContext {
        MatchPersistenceContext::new(
            self.home.lineup.id(),
            self.away.lineup.id(),
            self.home.formation.id(),
            self.away.formation.id(),
            Some(self.home.profiles[0].id()),
            Some(self.away.profiles[0].id()),
            self.venue_id,
        )
        .with_timestamps(0, 0)
    }
}
