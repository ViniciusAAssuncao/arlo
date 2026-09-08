use crate::error::TacticsResult;
use crate::lineup::TacticalLineup;
use crate::playcall::category::PlayCallCategory;
use crate::playcall::decision_emphasis::DecisionEmphasis;
use crate::playcall::misdirection::MisdirectionLink;
use crate::playcall::play_call::PlayCall;
use crate::playcall::route::RouteAssignment;
use crate::playcall::situational::SituationalProfile;
use crate::playcall::validation::validate_play_call;
use arlo_domain::SlotRole;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct PlayCallBuilder {
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

impl PlayCallBuilder {
    pub fn new(
        id: Uuid,
        team_id: Uuid,
        tactical_lineup_id: Uuid,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            team_id,
            tactical_lineup_id,
            name: name.into(),
            category: PlayCallCategory::default(),
            situational_profile: None,
            decision_emphasis: DecisionEmphasis::default(),
            routes: Vec::new(),
            role_overrides: Vec::new(),
            misdirection: None,
            counter_play_id: None,
        }
    }

    pub fn with_tactical_lineup_id(mut self, tactical_lineup_id: Uuid) -> Self {
        self.tactical_lineup_id = tactical_lineup_id;
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_category(mut self, category: PlayCallCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_situational_profile(mut self, situational_profile: SituationalProfile) -> Self {
        self.situational_profile = Some(situational_profile);
        self
    }

    pub fn with_optional_situational_profile(
        mut self,
        situational_profile: Option<SituationalProfile>,
    ) -> Self {
        self.situational_profile = situational_profile;
        self
    }

    pub fn with_decision_emphasis(mut self, decision_emphasis: DecisionEmphasis) -> Self {
        self.decision_emphasis = decision_emphasis;
        self
    }

    pub fn with_route(mut self, route: RouteAssignment) -> Self {
        self.routes.push(route);
        self
    }

    pub fn with_routes(mut self, routes: impl IntoIterator<Item = RouteAssignment>) -> Self {
        self.routes.extend(routes);
        self
    }

    pub fn with_role_override(mut self, slot_index: usize, role: SlotRole) -> Self {
        self.role_overrides.push((slot_index, role));
        self
    }

    pub fn with_role_overrides(
        mut self,
        role_overrides: impl IntoIterator<Item = (usize, SlotRole)>,
    ) -> Self {
        self.role_overrides.extend(role_overrides);
        self
    }

    pub fn with_misdirection(mut self, misdirection: MisdirectionLink) -> Self {
        self.misdirection = Some(misdirection);
        self
    }

    pub fn with_optional_misdirection(
        mut self,
        misdirection: Option<MisdirectionLink>,
    ) -> Self {
        self.misdirection = misdirection;
        self
    }

    pub fn with_counter_play(mut self, counter_play_id: Uuid) -> Self {
        self.counter_play_id = Some(counter_play_id);
        self
    }

    pub fn with_counter_play_id(mut self, counter_play_id: Option<Uuid>) -> Self {
        self.counter_play_id = counter_play_id;
        self
    }

    pub fn build(self, lineup: &TacticalLineup) -> TacticsResult<PlayCall> {
        validate_play_call(
            &self.routes,
            &self.role_overrides,
            self.misdirection.as_ref(),
            self.category,
            lineup,
        )?;

        Ok(PlayCall::new(
            self.id,
            self.team_id,
            self.tactical_lineup_id,
            self.name,
            self.category,
            self.situational_profile,
            self.decision_emphasis,
            self.routes,
            self.role_overrides,
            self.misdirection,
            self.counter_play_id,
        ))
    }
}