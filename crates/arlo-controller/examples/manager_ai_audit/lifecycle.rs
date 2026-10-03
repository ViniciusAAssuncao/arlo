use super::AuditResult;
use arlo_engine::{MatchInput, MatchState};
use arlo_events::{MatchEvent, MatchEventEnvelope};
use arlo_match_runner::manager_ai::{ManagerAction, ManagerAssessment};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, serde::Serialize)]
pub struct Lifecycle {
    pub recovered_reserve_segments: usize,
    pub reentries_with_recovery: usize,
    pub plans_after_substitution: usize,
    pub plans_while_shorthanded: usize,
    pub invalidations_after_tactical_change: usize,
    pub unavailable_candidate_violations: usize,
    pub plan_cooldown_violations: usize,
    pub suspension_returns_after_invalidation: usize,
    #[serde(skip)]
    exits: HashMap<Uuid, f64>,
    #[serde(skip)]
    substitutions: HashMap<Uuid, usize>,
    #[serde(skip)]
    plans: HashMap<Uuid, f64>,
    #[serde(skip)]
    changed: bool,
    #[serde(skip)]
    invalidation_pending: bool,
}

impl std::fmt::Debug for Lifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lifecycle")
            .field(
                "recovered_reserve_segments",
                &self.recovered_reserve_segments,
            )
            .field("reentries_with_recovery", &self.reentries_with_recovery)
            .field("plans_after_substitution", &self.plans_after_substitution)
            .field("plans_while_shorthanded", &self.plans_while_shorthanded)
            .field(
                "suspension_returns_after_invalidation",
                &self.suspension_returns_after_invalidation,
            )
            .field(
                "invalidations_after_tactical_change",
                &self.invalidations_after_tactical_change,
            )
            .finish()
    }
}

impl Lifecycle {
    pub fn include(&mut self, other: &Self) {
        self.recovered_reserve_segments += other.recovered_reserve_segments;
        self.reentries_with_recovery += other.reentries_with_recovery;
        self.plans_after_substitution += other.plans_after_substitution;
        self.plans_while_shorthanded += other.plans_while_shorthanded;
        self.invalidations_after_tactical_change += other.invalidations_after_tactical_change;
        self.unavailable_candidate_violations += other.unavailable_candidate_violations;
        self.plan_cooldown_violations += other.plan_cooldown_violations;
        self.suspension_returns_after_invalidation += other.suspension_returns_after_invalidation;
    }

    pub fn assessment(
        &self,
        state: &MatchState,
        team: Uuid,
        assessment: &ManagerAssessment,
    ) -> AuditResult<()> {
        let current = state.team_state(team)?;
        if !assessment.aspiration.is_finite()
            || assessment
                .candidates
                .iter()
                .any(|value| !value.utility.is_finite())
        {
            return Err("Nonfinite manager assessment".into());
        }
        for candidate in &assessment.candidates {
            let valid = match candidate.action {
                ManagerAction::Substitution(intent, _) => {
                    current
                        .active_player_ids()
                        .contains(&intent.outgoing_player_id())
                        && current.is_available_reserve(intent.incoming_player_id())
                }
                ManagerAction::Realignment(intent) => {
                    current
                        .active_player_ids()
                        .contains(&intent.first_player_id())
                        && current
                            .active_player_ids()
                            .contains(&intent.second_player_id())
                }
                ManagerAction::PreparedPlan(_) => true,
            };
            if !valid {
                return Err("Manager considered an unavailable player".into());
            }
        }
        Ok(())
    }

    pub fn event(&mut self, state: &MatchState, envelope: &MatchEventEnvelope) -> AuditResult<()> {
        if !matches!(
            envelope.event(),
            MatchEvent::PlayInvalidated(_) | MatchEvent::PlayerAvailabilityChanged(_)
        ) {
            self.invalidation_pending = false;
        }
        match envelope.event() {
            MatchEvent::SubstitutionMade(value) => {
                if self
                    .exits
                    .get(&value.player_in())
                    .is_some_and(|exit| state.player_energy(value.player_in()) > *exit + 1e-9)
                {
                    self.reentries_with_recovery += 1;
                }
                self.exits
                    .insert(value.player_out(), state.player_energy(value.player_out()));
                *self.substitutions.entry(value.team_id()).or_default() += 1;
            }
            MatchEvent::TacticalPlanActivated(value) => {
                let elapsed = envelope.clock().total_elapsed_seconds();
                if self
                    .plans
                    .insert(value.team_id, elapsed)
                    .is_some_and(|last| elapsed - last < 1800.0 - 1e-6)
                {
                    return Err("Automatic plan violated its cooldown".into());
                }
                self.plans_after_substitution +=
                    usize::from(self.substitutions.contains_key(&value.team_id));
                self.plans_while_shorthanded +=
                    usize::from(state.team_state(value.team_id)?.active_player_ids().len() < 14);
                self.changed = true;
            }
            MatchEvent::TacticalRealignmentMade(_) => self.changed = true,
            MatchEvent::PlayInvalidated(_) => {
                self.invalidation_pending = true;
                self.invalidations_after_tactical_change += usize::from(self.changed)
            }
            MatchEvent::PlayerAvailabilityChanged(value) => {
                self.suspension_returns_after_invalidation += usize::from(
                    self.invalidation_pending
                        && value.previous_status() == arlo_events::AvailabilityStatus::Suspended
                        && value.new_status() == arlo_events::AvailabilityStatus::Active,
                );
            }
            _ => {}
        }
        Ok(())
    }

    pub fn reserves(state: &MatchState) -> HashMap<Uuid, f64> {
        [state.home(), state.away()]
            .into_iter()
            .flat_map(|team| {
                team.reserve_player_ids()
                    .iter()
                    .filter(|id| team.is_available_reserve(**id))
                    .map(|id| (*id, state.player_energy(*id)))
            })
            .collect()
    }

    pub fn recovery(
        &mut self,
        input: &MatchInput,
        state: &MatchState,
        before: &HashMap<Uuid, f64>,
    ) -> AuditResult<()> {
        for (&id, &old) in before {
            let new = state.player_energy(id);
            if new > old + 1e-12 {
                if new > input.player_start_energy(id) + 1e-12 {
                    return Err("Bench recovery exceeded starting condition".into());
                }
                self.recovered_reserve_segments += 1;
            }
        }
        Ok(())
    }
}
