use super::valuation::{assigned_position, manager_value, player_quality, tactical_fit, team_pair};
use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::resolution::context::validate_match_state;
use crate::state::{MatchPhase, MatchState};
use crate::step::StepResult;
use arlo_domain::{AttributeKey, Position};
use arlo_events::{MatchClockInstant, MatchEvent, SubstitutionMade, SubstitutionReason};
use arlo_manager_control::SubstitutionIntent;
use uuid::Uuid;

pub fn select_substitution(input: &MatchInput, state: &MatchState, team_id: Uuid) -> Option<(SubstitutionIntent, SubstitutionReason)> {
    if state.phase() != MatchPhase::Stopped || state.clock().seconds_in_period() >= state.clock().period_limit_seconds()
        || state.clock().total_elapsed_seconds() < 2400.0 { return None; }
    let (team, team_state) = team_pair(input, state, team_id)?;
    if team.manager().is_human_controlled() || team_state.active_player_ids().len() < 14
        || team_state.last_voluntary_substitution_at().is_some_and(|last| state.clock().total_elapsed_seconds() - last < 900.0) {
        return None;
    }
    let management = manager_value(input, team.manager(), AttributeKey::LoadManagement);
    let adjustments = manager_value(input, team.manager(), AttributeKey::InGameAdjustments);
    let opposing_score = if team_id == input.home().team_id() { state.away().score().total_points() }
        else { state.home().score().total_points() };
    let deficit = i64::from(opposing_score) - i64::from(team_state.score().total_points());
    let tactical = state.clock().total_elapsed_seconds() >= 4800.0 && deficit.abs() >= 6;
    let mut best: Option<(f64, SubstitutionIntent, SubstitutionReason)> = None;
    for &outgoing in team_state.active_player_ids() {
        let Some(position) = assigned_position(team, team_state, outgoing) else { continue };
        let energy = state.player_energy(outgoing);
        let injured = team_state.injured_player_ids().contains(&outgoing);
        if energy > 0.46 && !injured && !tactical { continue; }
        let current = player_quality(input, state, team, outgoing, position, false);
        for &incoming in team_state.reserve_player_ids() {
            if team_state.injured_player_ids().contains(&incoming) { continue; }
            let replacement = player_quality(input, state, team, incoming, position, true);
            let specialist_cost = if matches!(position, Position::Artrine | Position::Passer | Position::Goalguard) { 0.10 } else { 0.0 };
            let injury_risk = if injured { 0.12 } else { 0.0 };
            let fatigue_risk = (0.3 - energy).max(0.0) * (0.14 + management * 0.005);
            let tactical_gain = if tactical {
                (tactical_fit(input, team, incoming, position, deficit > 0)
                    - tactical_fit(input, team, outgoing, position, deficit > 0))
                    * (0.08 + adjustments * 0.006)
            } else { 0.0 };
            let gain = replacement - current + injury_risk + fatigue_risk + tactical_gain - specialist_cost;
            if gain > 0.10 && best.as_ref().is_none_or(|(value, _, _)| gain > *value) {
                let reason = if tactical_gain > injury_risk + fatigue_risk && energy > 0.46 {
                    SubstitutionReason::Tactical
                } else { SubstitutionReason::Fatigue };
                best = Some((gain, SubstitutionIntent::new(outgoing, incoming), reason));
            }
        }
    }
    best.map(|(_, intent, reason)| (intent, reason))
}

pub fn resolve_substitution_segment(
    input: &MatchInput, state: &mut MatchState, team_id: Uuid, intents: &[SubstitutionIntent], reason: SubstitutionReason,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    let mut next = state.clone();
    let mut events = Vec::with_capacity(intents.len());
    for intent in intents {
        next.substitute_voluntarily(team_id, intent.outgoing_player_id(), intent.incoming_player_id())?;
        let clock = MatchClockInstant::with_total_elapsed_seconds(
            next.clock().period(), next.clock().seconds_in_period(), next.clock().total_elapsed_seconds(),
        );
        events.push(next.emit(MatchEvent::SubstitutionMade(SubstitutionMade::new(
            team_id, intent.outgoing_player_id(), intent.incoming_player_id(), clock, reason,
        )))?);
    }
    *state = next;
    Ok(StepResult::resolved(events))
}
