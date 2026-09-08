use crate::error::TacticsResult;
use crate::persistence::models::play_call_category_code::parse_play_call_category;
use crate::persistence::models::rows::play_call_decision_emphasis_row::{
    build_decision_emphasis, PlayCallDecisionEmphasisRow,
};
use crate::persistence::models::rows::play_call_misdirection_link_row::PlayCallMisdirectionLinkRow;
use crate::persistence::models::rows::play_call_route_assignment_row::PlayCallRouteAssignmentRow;
use crate::persistence::models::rows::play_call_situational_parameter_row::{
    situational_profile_from_pairs, PlayCallSituationalParameterRow,
};
use crate::persistence::models::situational_parameter_key_code::parse_situational_parameter_key;
use crate::playcall::PlayCall;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PlayCallRow {
    pub id: String,
    pub team_id: String,
    pub tactical_lineup_id: String,
    pub name: String,
    pub category: String,
    pub counter_play_id: Option<String>,
    pub created_at_unix_seconds: i64,
}

impl PlayCallRow {
    pub fn to_domain(
        &self,
        situational_rows: &[PlayCallSituationalParameterRow],
        emphasis_rows: &[PlayCallDecisionEmphasisRow],
        route_rows: &[PlayCallRouteAssignmentRow],
        misdirection_row: Option<&PlayCallMisdirectionLinkRow>,
    ) -> TacticsResult<PlayCall> {
        let id = Uuid::parse_str(&self.id)?;
        let team_id = Uuid::parse_str(&self.team_id)?;
        let tactical_lineup_id = Uuid::parse_str(&self.tactical_lineup_id)?;
        let category = parse_play_call_category(&self.category)?;
        let counter_play_id = match &self.counter_play_id {
            Some(cid) => Some(Uuid::parse_str(cid)?),
            None => None,
        };

        let situational_profile = if situational_rows.is_empty() {
            None
        } else {
            let mut pairs = Vec::with_capacity(situational_rows.len());
            for r in situational_rows {
                let key = parse_situational_parameter_key(&r.parameter_key)?;
                pairs.push((key, r.value));
            }
            Some(situational_profile_from_pairs(&pairs))
        };

        let decision_emphasis = build_decision_emphasis(emphasis_rows)?;

        let mut routes = Vec::new();
        let mut role_overrides = Vec::new();

        for r in route_rows {
            let (route_opt, override_opt) = r.to_domain()?;
            if let Some(route) = route_opt {
                routes.push(route);
            }
            if let Some(override_entry) = override_opt {
                role_overrides.push(override_entry);
            }
        }

        let misdirection = match misdirection_row {
            Some(m) => Some(m.to_domain()?),
            None => None,
        };

        Ok(PlayCall::new(
            id,
            team_id,
            tactical_lineup_id,
            &self.name,
            category,
            situational_profile,
            decision_emphasis,
            routes,
            role_overrides,
            misdirection,
            counter_play_id,
        ))
    }
}
