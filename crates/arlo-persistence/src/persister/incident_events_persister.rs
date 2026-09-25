use crate::error::PersistenceResult;
use crate::models::{
    MatchAddedTimeRow, MatchAvailabilityChangeRow, MatchChallengeRow, MatchFoulRow,
    MatchImpulseCriticalEventRow, MatchInjuryRow, MatchKickFoulAwardRow, MatchKickFoulDecisionRow,
    MatchPlayCallSelectionRow, MatchScoringPlayRow, MatchSubstitutionRow,
    MatchTacticalProfileActivationRow, MatchTimeCallRow, MatchTurnoverRow,
};
use crate::repositories;
use arlo_events::MatchEvent;
use arlo_match_runner::MatchRunResult;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_incident_events(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    run_result: &MatchRunResult,
) -> PersistenceResult<()> {
    let mut scoring_plays = Vec::new();
    let mut fouls = Vec::new();
    let mut injuries = Vec::new();
    let mut substitutions = Vec::new();
    let mut turnovers = Vec::new();
    let mut kick_foul_awards = Vec::new();
    let mut kick_foul_decisions = Vec::new();
    let mut time_calls = Vec::new();
    let mut challenges = Vec::new();
    let mut tactical_profile_activations = Vec::new();
    let mut play_call_selections = Vec::new();
    let mut availability_changes = Vec::new();
    let mut impulse_critical_events = Vec::new();
    let mut added_time_awards = Vec::new();
    let mut referee_decisions = Vec::new();
    let mut foul_punishments = Vec::new();

    for envelope in run_result.raw_sink.events() {
        let seq = envelope.sequence_number();
        let clock = envelope.clock();
        match envelope.event() {
            MatchEvent::GoalPoint(e) => {
                scoring_plays.push(MatchScoringPlayRow::from_goal_point(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::FieldPoint(e) => {
                scoring_plays.push(MatchScoringPlayRow::from_field_point(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::FieldGoal(e) => {
                scoring_plays.push(MatchScoringPlayRow::from_field_goal(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::ScoringAttemptMissed(e) => {
                scoring_plays.push(MatchScoringPlayRow::from_missed_attempt(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::FoulRaised(e) => {
                fouls.push(MatchFoulRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::RefereeDecisionResolved(e) => {
                let origin = match e.origin() {
                    arlo_events::FoulOrigin::ContactDuel(kind) => format!("ContactDuel:{}", kind.as_str()),
                    arlo_events::FoulOrigin::LineFault => "LineFault".to_owned(),
                    arlo_events::FoulOrigin::CallToAction => "CallToAction".to_owned(),
                    arlo_events::FoulOrigin::Drive => "Drive".to_owned(),
                    arlo_events::FoulOrigin::ShotAttempt => "ShotAttempt".to_owned(),
                    arlo_events::FoulOrigin::OutOfBounds => "OutOfBounds".to_owned(),
                };
                referee_decisions.push((seq as i64, clock.period() as i32, clock.seconds_in_period(),
                    e.offending_player_id().to_string(), e.offending_team_id().to_string(),
                    e.opposing_player_id().to_string(), e.opposing_team_id().to_string(), origin,
                    e.fault_definition_id().map(|id| id.to_string()), e.factual_foul(), e.original_call(),
                    e.peace_referee_intervened(), e.final_call()));
            }
            MatchEvent::PunishmentApplied(e) => {
                foul_punishments.push((seq as i64, clock.period() as i32, clock.seconds_in_period(),
                    e.offending_player_id().to_string(), e.offending_team_id().to_string(),
                    e.fault_definition_id().map(|id| id.to_string()), format!("{:?}", e.kind()), e.magnitude()));
            }
            MatchEvent::InjuryIncidentRecorded(e) => {
                injuries.push(MatchInjuryRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::SubstitutionMade(e) => {
                substitutions.push(MatchSubstitutionRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::Turnover(e) => {
                turnovers.push(MatchTurnoverRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::KickFoulAwarded(e) => {
                kick_foul_awards.push(MatchKickFoulAwardRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::KickFoulDecisionMade(e) => {
                kick_foul_decisions.push(MatchKickFoulDecisionRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::TimeCallUsed(e) => {
                time_calls.push(MatchTimeCallRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::ChallengeResolved(e) => {
                challenges.push(MatchChallengeRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::TacticalProfileActivated(e) => {
                tactical_profile_activations.push(MatchTacticalProfileActivationRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::PlayCallSelected(e) => {
                play_call_selections.push(MatchPlayCallSelectionRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::PlayerAvailabilityChanged(e) => {
                availability_changes.push(MatchAvailabilityChangeRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::ImpulseCriticalReached(e) => {
                impulse_critical_events.push(MatchImpulseCriticalEventRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            MatchEvent::AddedTimeAwarded(e) => {
                added_time_awards.push(MatchAddedTimeRow::from_event(
                    Uuid::new_v4(),
                    match_id,
                    seq,
                    clock,
                    e,
                ));
            }
            _ => {}
        }
    }

    repositories::match_scoring_plays::insert_batch(tx, &scoring_plays).await?;
    repositories::match_fouls::insert_batch(tx, &fouls).await?;
    repositories::match_injuries::insert_batch(tx, &injuries).await?;
    repositories::match_substitutions::insert_batch(tx, &substitutions).await?;
    repositories::match_turnovers::insert_batch(tx, &turnovers).await?;
    repositories::match_kick_fouls::insert_awards_batch(tx, &kick_foul_awards).await?;
    repositories::match_kick_fouls::insert_decisions_batch(tx, &kick_foul_decisions).await?;
    repositories::match_manager_decision_timeline::insert_time_calls_batch(tx, &time_calls).await?;
    repositories::match_manager_decision_timeline::insert_challenges_batch(tx, &challenges).await?;
    repositories::match_manager_decision_timeline::insert_tactical_profile_activations_batch(
        tx,
        &tactical_profile_activations,
    )
    .await?;
    repositories::match_manager_decision_timeline::insert_play_call_selections_batch(
        tx,
        &play_call_selections,
    )
    .await?;
    repositories::match_availability_changes::insert_batch(tx, &availability_changes).await?;
    repositories::match_impulse_critical_events::insert_batch(tx, &impulse_critical_events).await?;
    repositories::match_added_time::insert_batch(tx, &added_time_awards).await?;
    for (seq, period, seconds, offender, team, opposing, opposing_team, origin, definition, factual, original, intervened, final_call) in referee_decisions {
        sqlx::query("INSERT INTO match_referee_decisions (match_id, sequence_number, period, seconds_in_period, offending_player_id, offending_team_id, opposing_player_id, opposing_team_id, origin, fault_definition_id, factual_foul, original_call, peace_referee_intervened, final_call) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(match_id.to_string()).bind(seq).bind(period).bind(seconds).bind(offender).bind(team)
            .bind(opposing).bind(opposing_team).bind(origin).bind(definition).bind(factual)
            .bind(original).bind(intervened).bind(final_call).execute(&mut **tx).await?;
    }
    for (seq, period, seconds, offender, team, definition, kind, magnitude) in foul_punishments {
        sqlx::query("INSERT INTO match_foul_punishments (match_id, sequence_number, period, seconds_in_period, offending_player_id, offending_team_id, fault_definition_id, kind, magnitude) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(match_id.to_string()).bind(seq).bind(period).bind(seconds).bind(offender).bind(team)
            .bind(definition).bind(kind).bind(magnitude).execute(&mut **tx).await?;
    }

    Ok(())
}
