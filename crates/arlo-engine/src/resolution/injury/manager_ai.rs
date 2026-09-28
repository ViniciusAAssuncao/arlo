use crate::error::{EngineError, EngineResult};
use crate::input::{MatchInput, TeamInput};
use crate::state::{MatchState, PendingInjuryDecision, TeamState};
use arlo_domain::{AttributeKey, InjurySeverityGrade, Manager, Player, Position, RotationPolicy};
use arlo_manager_control::InjuryDecisionIntent;
use rand::Rng;
use uuid::Uuid;

pub(super) fn decide(
    input: &MatchInput,
    state: &mut MatchState,
    pending: PendingInjuryDecision,
) -> EngineResult<InjuryDecisionIntent> {
    let (team, team_state, opponent_score) = if pending.team_id() == input.home().team_id() {
        (input.home(), state.home(), state.away().score().total_points())
    } else if pending.team_id() == input.away().team_id() {
        (input.away(), state.away(), state.home().score().total_points())
    } else {
        return Err(EngineError::InvalidInput("unknown injury decision team".into()));
    };
    let reserve = best_reserve(input, team, team_state, pending.player_id());
    let grade_risk = match pending.severity_grade() {
        InjurySeverityGrade::Grade1 => 0.09,
        InjurySeverityGrade::Grade2 => 0.34,
        InjurySeverityGrade::Grade3 => 0.62,
    };
    let typical_days = input.injury_catalog()
        .expected_recovery_days(pending.injury_definition_id(), pending.severity_grade())
        .unwrap_or(match pending.severity_grade() {
            InjurySeverityGrade::Grade1 => 10.0,
            InjurySeverityGrade::Grade2 => 35.0,
            InjurySeverityGrade::Grade3 => 100.0,
        });
    let duration_risk = ((typical_days.max(1.0) + 1.0).ln() / 181.0_f64.ln()).clamp(0.0, 1.0);
    let energy = input.player_start_energy(pending.player_id());
    let played_fraction = (state.clock().total_elapsed_seconds() / 7200.0).clamp(0.0, 1.0);
    let effective_energy = (energy - 0.25 * played_fraction).clamp(0.0, 1.0);
    let load_management = manager_value(input, team.manager(), AttributeKey::LoadManagement);
    let judging = manager_value(input, team.manager(), AttributeKey::JudgingAbility);
    let profile_bias = match team.manager().tactical_profile().map(|profile| profile.rotation_policy()) {
        Some(RotationPolicy::StrictCore) => -0.06,
        Some(RotationPolicy::HighRotation) => 0.06,
        _ => 0.0,
    };
    let reserve_bias = reserve.map(|(_, quality)| {
        let current = player_quality(input, team, pending.player_id(), assigned_position(team, team_state, pending.player_id()));
        0.20 * (quality - current)
    }).unwrap_or(-0.20);
    let score_delta = f64::from(team_state.score().total_points()) - f64::from(opponent_score);
    let score_bias = if score_delta > 0.0 { 0.05 * played_fraction }
        else if score_delta < 0.0 { -0.05 * played_fraction }
        else { 0.0 };
    let judgment_bias = (judging - 10.0) / 10.0 * 0.04 * duration_risk;
    let probability = (grade_risk + 0.21 * duration_risk + 0.18 * (1.0 - effective_energy)
        + 0.10 * (load_management - 10.0) / 10.0 + profile_bias
        + reserve_bias + score_bias + judgment_bias).clamp(0.03, 0.92);
    let withdraw = state.rng_mut().gen_bool(probability);
    Ok(match (withdraw, reserve) {
        (true, Some((replacement_id, _))) => InjuryDecisionIntent::withdraw(pending.player_id(), replacement_id),
        (true, None) if probability >= 0.70 => InjuryDecisionIntent::withdraw_without_replacement(pending.player_id()),
        _ => InjuryDecisionIntent::keep(pending.player_id()),
    })
}

pub(super) fn best_reserve(
    input: &MatchInput,
    team: &TeamInput,
    state: &TeamState,
    injured_id: Uuid,
) -> Option<(Uuid, f64)> {
    let position = assigned_position(team, state, injured_id);
    state.reserve_player_ids().iter().filter_map(|id| {
        team.roster().iter().find(|player| player.id() == *id)
            .map(|_| (*id, player_quality(input, team, *id, position)))
    }).max_by(|left, right| left.1.total_cmp(&right.1).then_with(|| left.0.cmp(&right.0)))
}

fn assigned_position(team: &TeamInput, state: &TeamState, player_id: Uuid) -> Option<Position> {
    team.lineup().assignments().iter()
        .find(|assignment| state.slot_player_id(assignment.player_id()) == player_id)
        .map(|assignment| assignment.position())
}

fn player_quality(input: &MatchInput, team: &TeamInput, player_id: Uuid, position: Option<Position>) -> f64 {
    let Some(player) = team.roster().iter().find(|player| player.id() == player_id) else { return 0.0 };
    let proficiency = position.and_then(|position| player.positions().iter()
        .find(|candidate| candidate.position() == position)
        .map(|candidate| candidate.proficiency())).unwrap_or(0);
    let stamina = player_value(input, player, AttributeKey::Stamina);
    let skill = if position == Some(Position::Goalguard) {
        player_value(input, player, AttributeKey::Reflexes)
    } else {
        player_value(input, player, AttributeKey::Decisions)
    };
    0.55 * f64::from(proficiency) / 10.0 + 0.10 * stamina / 20.0
        + 0.20 * skill / 20.0 + 0.15 * input.player_start_energy(player_id)
}

fn player_value(input: &MatchInput, player: &Player, key: AttributeKey) -> f64 {
    let Some(definition) = input.player_attribute_definitions().iter().find(|definition| definition.key() == key) else { return 10.0 };
    player.attributes().iter().find(|value| value.attribute_definition_id() == definition.id())
        .map(|value| f64::from(value.value())).unwrap_or(10.0)
}

fn manager_value(input: &MatchInput, manager: &Manager, key: AttributeKey) -> f64 {
    manager.attributes().iter()
        .find(|value| input.manager_attribute_keys().get(&value.attribute_definition_id()) == Some(&key))
        .map(|value| f64::from(value.value())).unwrap_or(10.0)
}
