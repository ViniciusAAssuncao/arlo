mod adapter;
mod candidates;
mod choice;
mod context;
mod cooldowns;
mod diagnosis;
mod evidence;
mod perception;
mod plan_adapter;
mod plan_context;
mod plans;
mod random;
mod realignment;
mod valuation;

pub use candidates::SubstitutionCandidate;
pub use diagnosis::TeamDiagnosis;
pub use plans::PreparedPlanCandidate;
pub use realignment::RealignmentCandidate;

use arlo_engine::{MatchInput, MatchState};
use arlo_events::SubstitutionReason;
use arlo_manager_control::{PreparedPlanIntent, SubstitutionIntent, TacticalRealignmentIntent};
use arlo_stats::AggregatorRegistry;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SubstitutionAssessment {
    pub diagnosis: TeamDiagnosis,
    pub aspiration: f64,
    pub candidates: Vec<SubstitutionCandidate>,
    pub selected_index: Option<usize>,
}

impl SubstitutionAssessment {
    pub fn selected(&self) -> Option<(SubstitutionIntent, SubstitutionReason)> {
        let candidate = self.candidates.get(self.selected_index?)?;
        Some((candidate.intent, candidate.reason))
    }
}

pub fn assess_substitution(
    input: &MatchInput,
    state: &MatchState,
    registry: &AggregatorRegistry,
    team_id: Uuid,
) -> Option<SubstitutionAssessment> {
    let assessment = assess(input, state, Some(registry), team_id)?;
    let selected_index = assessment.selected().and_then(|action| match action {
        ManagerAction::Substitution(intent, _) => assessment
            .substitutions
            .iter()
            .position(|candidate| candidate.intent == intent),
        ManagerAction::Realignment(_) | ManagerAction::PreparedPlan(_) => None,
    });
    Some(SubstitutionAssessment {
        diagnosis: assessment.diagnosis,
        aspiration: assessment.aspiration,
        candidates: assessment.substitutions,
        selected_index,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerAction {
    Substitution(SubstitutionIntent, SubstitutionReason),
    Realignment(TacticalRealignmentIntent),
    PreparedPlan(PreparedPlanIntent),
}

#[derive(Debug, Clone)]
pub struct ActionCandidate {
    pub action: ManagerAction,
    pub utility: f64,
}

#[derive(Debug, Clone)]
pub struct ManagerAssessment {
    pub diagnosis: TeamDiagnosis,
    pub aspiration: f64,
    pub substitutions: Vec<SubstitutionCandidate>,
    pub realignments: Vec<RealignmentCandidate>,
    pub prepared_plans: Vec<PreparedPlanCandidate>,
    pub candidates: Vec<ActionCandidate>,
    pub selected_index: Option<usize>,
}

impl ManagerAssessment {
    pub fn selected(&self) -> Option<ManagerAction> {
        Some(self.candidates.get(self.selected_index?)?.action)
    }
}

pub fn assess_manager_decision(
    input: &MatchInput,
    state: &MatchState,
    registry: &AggregatorRegistry,
    team_id: Uuid,
) -> Option<ManagerAssessment> {
    assess(input, state, Some(registry), team_id)
}

pub(crate) fn select_action(
    input: &MatchInput,
    state: &MatchState,
    registry: Option<&AggregatorRegistry>,
    team_id: Uuid,
) -> Option<ManagerAction> {
    let team = state.team_state(team_id).ok()?;
    let readiness =
        cooldowns::DecisionReadiness::from_state(team, state.clock().total_elapsed_seconds());
    if !readiness.substitution && !readiness.realignment && !readiness.plan {
        return None;
    }
    assess(input, state, registry, team_id)?.selected()
}

fn assess(
    input: &MatchInput,
    state: &MatchState,
    registry: Option<&AggregatorRegistry>,
    team_id: Uuid,
) -> Option<ManagerAssessment> {
    let context = adapter::build_context(input, state, registry, team_id)?;
    let perceptions = perception::perceive(&context);
    let diagnosis = diagnosis::diagnose_team(&context, &perceptions);
    let substitutions = candidates::generate(&context, &perceptions, &diagnosis);
    let realignments = realignment::generate(&context, &perceptions, &diagnosis);
    let prepared_plans = plans::generate(&context, &diagnosis);
    let mut candidates: Vec<_> = substitutions
        .iter()
        .map(|candidate| ActionCandidate {
            action: ManagerAction::Substitution(candidate.intent, candidate.reason),
            utility: candidate.utility,
        })
        .chain(realignments.iter().map(|candidate| ActionCandidate {
            action: ManagerAction::Realignment(candidate.intent),
            utility: candidate.utility,
        }))
        .chain(prepared_plans.iter().map(|candidate| ActionCandidate {
            action: ManagerAction::PreparedPlan(candidate.intent),
            utility: candidate.utility,
        }))
        .collect();
    candidates.sort_by(|a, b| b.utility.total_cmp(&a.utility));
    let aspiration = choice::aspiration(&context);
    let utilities: Vec<_> = candidates
        .iter()
        .map(|candidate| candidate.utility)
        .collect();
    let selected_index = choice::choose(&context, &utilities, aspiration);
    Some(ManagerAssessment {
        diagnosis,
        aspiration,
        substitutions,
        realignments,
        prepared_plans,
        candidates,
        selected_index,
    })
}
