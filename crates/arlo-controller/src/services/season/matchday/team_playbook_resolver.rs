use crate::error::{ControllerError, ControllerResult};
use arlo_domain::{ArtrineDecisionKind, Manager};
use arlo_tactics::{PlayCall, PlayCallCategory, SituationalProfile, TeamTacticalProfile};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_team_playbook(
    pool: &SqlitePool,
    team_id: Uuid,
    tactical_lineup_id: Uuid,
    profile: &TeamTacticalProfile,
    manager: &Manager,
) -> ControllerResult<Vec<PlayCall>> {
    let existing: Vec<_> = if manager.is_human_controlled() {
        arlo_tactics::play_call::list_by_lineup_id(pool, tactical_lineup_id).await
    } else {
        arlo_tactics::play_call::list_authored_by_lineup_id(pool, tactical_lineup_id).await
    }
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .into_iter()
        .filter(|call| call.team_id() == team_id && call.category() == PlayCallCategory::OpenPlay)
        .collect();

    if manager.is_human_controlled() && !existing.is_empty() {
        return Ok(existing);
    }
    if !existing.is_empty() {
        return Ok(existing);
    }

    let base = profile.instructions().default_decision_emphasis();
    let options = [
        ("Equilíbrio", SituationalProfile::new_clamped(0.25, 0.45, 0.45, 0.5, 0.93), base),
        ("Construir Drive", SituationalProfile::new_clamped(0.25, 0.45, 0.4, 1.0, 0.68),
            base.with_emphasis(ArtrineDecisionKind::SelfCarry, (base.self_carry().value() + 0.16).min(1.0))),
        ("Avançar Série", SituationalProfile::new_clamped(0.9, 0.85, 0.5, 0.4, 0.68),
            base.with_emphasis(ArtrineDecisionKind::LongLaunch, (base.long_launch().value() + 0.14).min(1.0))),
        ("Finalizar", SituationalProfile::new_clamped(0.4, 0.25, 0.85, 0.0, 0.68),
            base.with_emphasis(ArtrineDecisionKind::SelfFinish, (base.self_finish().value() + 0.15).min(1.0))),
    ];
    let mut calls = Vec::with_capacity(options.len());
    for (name, situation, emphasis) in options {
        let call = PlayCall::new(
            Uuid::new_v4(), team_id, tactical_lineup_id, name, PlayCallCategory::OpenPlay,
            Some(situation), emphasis, Vec::new(), Vec::new(), None, None,
        );
        arlo_tactics::play_call::insert_generated(pool, &call).await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        calls.push(call);
    }
    Ok(calls)
}
