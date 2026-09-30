use super::valuation::{manager_value, team_pair};
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::resolution::context::validate_match_state;
use crate::state::{MatchPhase, MatchState};
use crate::step::StepResult;
use arlo_domain::AttributeKey;
use arlo_events::{MatchEvent, TacticalProfileActivated};
use arlo_tactics::TeamInstructions;
use uuid::Uuid;

pub fn select_tactical_profile(input: &MatchInput, state: &MatchState, team_id: Uuid) -> Option<Uuid> {
    if state.phase() != MatchPhase::Stopped
        || state.clock().seconds_in_period() >= state.clock().period_limit_seconds() { return None; }
    let (team, team_state) = team_pair(input, state, team_id)?;
    if team.manager().is_human_controlled() || team_state.tactical_switches() >= 3
        || team_state.last_tactical_switch_at().is_some_and(|last| state.clock().total_elapsed_seconds() - last < 1800.0) {
        return None;
    }
    let elapsed = state.clock().total_elapsed_seconds();
    let opponent_score = if team_id == input.home().team_id() {
        state.away().score().total_points()
    } else { state.home().score().total_points() };
    let deficit = i64::from(opponent_score) - i64::from(team_state.score().total_points());
    let chasing = elapsed >= 1800.0 && deficit >= if elapsed >= 5400.0 { 5 } else { 10 };
    let protecting = deficit <= -10 && elapsed >= 5400.0;
    let current_id = team_state.active_tactical_profile_id();
    if !chasing && !protecting {
        return (current_id != team.tactics().id() && deficit.abs() <= 3 && elapsed <= 6300.0)
            .then_some(team.tactics().id());
    }
    let flexibility = team.manager().tactical_profile()
        .map_or_else(|| manager_value(input, team.manager(), AttributeKey::Adaptability) / 20.0,
            |profile| profile.flexibility_tendency()).clamp(0.0, 1.0);
    let adjustments = (manager_value(input, team.manager(), AttributeKey::InGameAdjustments) / 20.0)
        .clamp(0.0, 1.0);
    let conviction = (1.0 - flexibility) * 0.10 + (1.0 - adjustments) * 0.035;
    let trend = f64::from(state.recent_score_balance(team_id).clamp(-15, 15)) * 0.002;
    let margin = if chasing {
        let score_pressure = ((deficit - 5).max(0) as f64 * 0.004).min(0.065);
        let time_pressure = ((elapsed - 5400.0) / 1800.0).clamp(0.0, 1.0) * 0.025;
        0.14 + conviction - score_pressure - time_pressure + trend
    } else {
        let lead_pressure = ((-deficit - 10).max(0) as f64 * 0.0015).min(0.025);
        0.085 + conviction * 0.65 - lead_pressure + trend
    };
    let current = team.tactical_profiles().find(|profile| profile.id() == current_id)?;
    let current_value = profile_value(current.instructions(), chasing);
    team.tactical_profiles()
        .filter(|profile| profile.id() != current_id)
        .map(|profile| (profile.id(), profile_value(profile.instructions(), chasing)))
        .filter(|(_, value)| *value > current_value + margin)
        .max_by(|left, right| left.1.total_cmp(&right.1))
        .map(|(id, _)| id)
}

fn profile_value(instructions: &TeamInstructions, chasing: bool) -> f64 {
    let offense = instructions.in_possession();
    let defense = instructions.out_of_possession();
    if chasing {
        0.38 * offense.mentality().value() + 0.24 * offense.tempo().value()
            + 0.20 * offense.directness().value() + 0.18 * defense.pressing_intensity().value()
    } else {
        -0.32 * offense.mentality().value() - 0.18 * offense.tempo().value()
            + 0.32 * defense.compactness().value() - 0.18 * defense.defensive_line_height().value()
    }
}

pub fn resolve_tactical_switch_segment(
    input: &MatchInput, state: &mut MatchState, team_id: Uuid, profile_id: Uuid,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    let team = if team_id == input.home().team_id() { input.home() }
        else if team_id == input.away().team_id() { input.away() }
        else { return Err(EngineError::InvalidInput("unknown tactical team".into())); };
    let profile = team.tactical_profiles().find(|profile| profile.id() == profile_id)
        .ok_or_else(|| EngineError::InvalidInput("unknown tactical profile".into()))?;
    let mut next = state.clone();
    next.activate_tactical_profile(team_id, profile_id)?;
    let event = next.emit(MatchEvent::TacticalProfileActivated(
        TacticalProfileActivated::new(team_id, profile_id, profile.name()),
    ))?;
    *state = next;
    Ok(StepResult::resolved(vec![event]))
}
