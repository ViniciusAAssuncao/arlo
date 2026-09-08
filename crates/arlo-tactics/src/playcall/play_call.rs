use crate::lineup::TacticalLineup;
use crate::playcall::builder::PlayCallBuilder;
use crate::playcall::category::PlayCallCategory;
use crate::playcall::decision_emphasis::DecisionEmphasis;
use crate::playcall::misdirection::MisdirectionLink;
use crate::playcall::route::RouteAssignment;
use crate::playcall::situational::{HasSituationalProfile, SituationalProfile};
use arlo_domain::sport_constants::{
    MANDATORY_ARTRINES_PER_FORMATION, MANDATORY_PASSERS_PER_FORMATION,
};
use arlo_domain::SlotRole;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayCall {
    id: Uuid,
    team_id: Uuid,
    tactical_lineup_id: Uuid,
    name: String,
    category: PlayCallCategory,
    situational_profile: Option<SituationalProfile>,
    decision_emphasis: DecisionEmphasis,
    routes: Vec<RouteAssignment>,
    role_overrides: Vec<(usize, SlotRole)>,
    misdirection: Option<MisdirectionLink>,
    counter_play_id: Option<Uuid>,
}

impl PlayCall {
    pub fn new(
        id: Uuid,
        team_id: Uuid,
        tactical_lineup_id: Uuid,
        name: impl Into<String>,
        category: PlayCallCategory,
        situational_profile: Option<SituationalProfile>,
        decision_emphasis: DecisionEmphasis,
        routes: Vec<RouteAssignment>,
        role_overrides: Vec<(usize, SlotRole)>,
        misdirection: Option<MisdirectionLink>,
        counter_play_id: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            team_id,
            tactical_lineup_id,
            name: name.into(),
            category,
            situational_profile,
            decision_emphasis,
            routes,
            role_overrides,
            misdirection,
            counter_play_id,
        }
    }

    pub fn builder(
        id: Uuid,
        team_id: Uuid,
        tactical_lineup_id: Uuid,
        name: impl Into<String>,
    ) -> PlayCallBuilder {
        PlayCallBuilder::new(id, team_id, tactical_lineup_id, name)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn tactical_lineup_id(&self) -> Uuid {
        self.tactical_lineup_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn category(&self) -> PlayCallCategory {
        self.category
    }

    pub fn situational_profile(&self) -> Option<&SituationalProfile> {
        self.situational_profile.as_ref()
    }

    pub fn decision_emphasis(&self) -> &DecisionEmphasis {
        &self.decision_emphasis
    }

    pub fn routes(&self) -> &[RouteAssignment] {
        &self.routes
    }

    pub fn role_overrides(&self) -> &[(usize, SlotRole)] {
        &self.role_overrides
    }

    pub fn misdirection(&self) -> Option<&MisdirectionLink> {
        self.misdirection.as_ref()
    }

    pub fn counter_play_id(&self) -> Option<Uuid> {
        self.counter_play_id
    }

    pub fn progression(&self) -> Vec<&RouteAssignment> {
        let mut sorted: Vec<&RouteAssignment> = self.routes.iter().collect();
        sorted.sort_by(|a, b| {
            b.read_priority()
                .value()
                .partial_cmp(&a.read_priority().value())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }

    pub fn protection_commitment(&self, lineup: &TacticalLineup) -> f64 {
        let eligible_slots = lineup
            .assignments()
            .len()
            .saturating_sub(MANDATORY_PASSERS_PER_FORMATION + MANDATORY_ARTRINES_PER_FORMATION);
        if eligible_slots == 0 {
            return 1.0;
        }
        1.0 - (self.routes.len() as f64 / eligible_slots as f64)
    }
}

impl HasSituationalProfile for PlayCall {
    fn situational_profile(&self) -> Option<&SituationalProfile> {
        self.situational_profile.as_ref()
    }
}