use crate::dto::r#match::timeline::*;
use crate::dto::r#match::MatchTimelineEventDto;
use crate::error::{ControllerError, ControllerResult};
use crate::services::r#match::match_clock_math::{format_elapsed_time, MatchClockDurationConfig};
use crate::services::r#match::match_timeline_merger::{
    merge_match_timeline, MatchIncidentsBundle, MatchTimelineEventData,
};
use arlo_persistence::models::incidents::*;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct MatchIncidentsBundleData {
    pub scoring_plays: Vec<MatchScoringPlayRow>,
    pub fouls: Vec<MatchFoulRow>,
    pub injuries: Vec<MatchInjuryRow>,
    pub substitutions: Vec<MatchSubstitutionRow>,
    pub turnovers: Vec<MatchTurnoverRow>,
    pub kick_foul_awards: Vec<MatchKickFoulAwardRow>,
    pub kick_foul_decisions: Vec<MatchKickFoulDecisionRow>,
    pub time_calls: Vec<MatchTimeCallRow>,
    pub challenges: Vec<MatchChallengeRow>,
    pub tactical_profile_activations: Vec<MatchTacticalProfileActivationRow>,
    pub play_call_selections: Vec<MatchPlayCallSelectionRow>,
    pub availability_changes: Vec<MatchAvailabilityChangeRow>,
    pub impulse_critical_events: Vec<MatchImpulseCriticalEventRow>,
    pub added_time_awards: Vec<MatchAddedTimeRow>,
}

pub async fn load_all_match_incidents(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<MatchIncidentsBundleData> {
    let scoring_plays =
        arlo_persistence::repositories::match_scoring_plays::list_by_match_id(pool, match_id)
            .await?;
    let fouls =
        arlo_persistence::repositories::match_fouls::list_by_match_id(pool, match_id).await?;
    let injuries =
        arlo_persistence::repositories::match_injuries::list_by_match_id(pool, match_id).await?;
    let substitutions =
        arlo_persistence::repositories::match_substitutions::list_by_match_id(pool, match_id)
            .await?;
    let turnovers =
        arlo_persistence::repositories::match_turnovers::list_by_match_id(pool, match_id).await?;
    let kick_foul_awards =
        arlo_persistence::repositories::match_kick_fouls::list_awards_by_match_id(pool, match_id)
            .await?;
    let kick_foul_decisions =
        arlo_persistence::repositories::match_kick_fouls::list_decisions_by_match_id(
            pool, match_id,
        )
        .await?;
    let time_calls =
        arlo_persistence::repositories::match_manager_decision_timeline::list_time_calls_by_match_id(
            pool, match_id,
        )
        .await?;
    let challenges =
        arlo_persistence::repositories::match_manager_decision_timeline::list_challenges_by_match_id(
            pool, match_id,
        )
        .await?;
    let tactical_profile_activations = arlo_persistence::repositories::match_manager_decision_timeline::list_tactical_profile_activations_by_match_id(
        pool,
        match_id,
    )
    .await?;
    let play_call_selections = arlo_persistence::repositories::match_manager_decision_timeline::list_play_call_selections_by_match_id(
        pool,
        match_id,
    )
    .await?;
    let availability_changes =
        arlo_persistence::repositories::match_availability_changes::list_by_match_id(
            pool, match_id,
        )
        .await?;
    let impulse_critical_events =
        arlo_persistence::repositories::match_impulse_critical_events::list_by_match_id(
            pool, match_id,
        )
        .await?;
    let added_time_awards =
        arlo_persistence::repositories::match_added_time::list_by_match_id(pool, match_id).await?;

    Ok(MatchIncidentsBundleData {
        scoring_plays,
        fouls,
        injuries,
        substitutions,
        turnovers,
        kick_foul_awards,
        kick_foul_decisions,
        time_calls,
        challenges,
        tactical_profile_activations,
        play_call_selections,
        availability_changes,
        impulse_critical_events,
        added_time_awards,
    })
}

pub fn build_timeline(
    clock_config: &MatchClockDurationConfig,
    bundle: &MatchIncidentsBundleData,
) -> Vec<MatchTimelineEventDto> {
    let items = merge_match_timeline(
        clock_config,
        MatchIncidentsBundle {
            scoring_plays: &bundle.scoring_plays,
            fouls: &bundle.fouls,
            injuries: &bundle.injuries,
            substitutions: &bundle.substitutions,
            turnovers: &bundle.turnovers,
            kick_foul_awards: &bundle.kick_foul_awards,
            kick_foul_decisions: &bundle.kick_foul_decisions,
            time_calls: &bundle.time_calls,
            challenges: &bundle.challenges,
            tactical_profile_activations: &bundle.tactical_profile_activations,
            play_call_selections: &bundle.play_call_selections,
            availability_changes: &bundle.availability_changes,
            impulse_critical_events: &bundle.impulse_critical_events,
            added_time_awards: &bundle.added_time_awards,
        },
    );

    items
        .into_iter()
        .map(|item| {
            let formatted_time = format_elapsed_time(item.total_elapsed_seconds);
            match item.event {
                MatchTimelineEventData::ScoringPlay(row) => {
                    MatchTimelineEventDto::Scoring(ScoringTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::Foul(row) => {
                    MatchTimelineEventDto::Foul(FoulTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::Injury(row) => {
                    MatchTimelineEventDto::Injury(InjuryTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::Substitution(row) => {
                    MatchTimelineEventDto::Substitution(SubstitutionTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::Turnover(row) => {
                    MatchTimelineEventDto::Turnover(TurnoverTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::KickFoulAward(row) => {
                    MatchTimelineEventDto::KickFoul(KickFoulTimelineEntryDto::from_award(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::KickFoulDecision(row) => {
                    MatchTimelineEventDto::KickFoul(KickFoulTimelineEntryDto::from_decision(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::TimeCall(row) => {
                    MatchTimelineEventDto::TimeCall(TimeCallTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::Challenge(row) => {
                    MatchTimelineEventDto::Challenge(ChallengeTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::TacticalProfileActivation(row) => {
                    MatchTimelineEventDto::TacticalProfile(
                        TacticalProfileTimelineEntryDto::from_row(
                            item.sequence_number,
                            item.period,
                            item.seconds_in_period,
                            item.total_elapsed_seconds,
                            formatted_time,
                            &row,
                        ),
                    )
                }
                MatchTimelineEventData::PlayCallSelection(row) => {
                    MatchTimelineEventDto::PlayCall(PlayCallTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
                MatchTimelineEventData::AvailabilityChange(row) => {
                    MatchTimelineEventDto::AvailabilityChange(
                        AvailabilityChangeTimelineEntryDto::from_row(
                            item.sequence_number,
                            item.period,
                            item.seconds_in_period,
                            item.total_elapsed_seconds,
                            formatted_time,
                            &row,
                        ),
                    )
                }
                MatchTimelineEventData::ImpulseCritical(row) => {
                    MatchTimelineEventDto::ImpulseCritical(
                        ImpulseCriticalTimelineEntryDto::from_row(
                            item.sequence_number,
                            item.period,
                            item.seconds_in_period,
                            item.total_elapsed_seconds,
                            formatted_time,
                            &row,
                        ),
                    )
                }
                MatchTimelineEventData::AddedTime(row) => {
                    MatchTimelineEventDto::AddedTime(AddedTimeTimelineEntryDto::from_row(
                        item.sequence_number,
                        item.period,
                        item.seconds_in_period,
                        item.total_elapsed_seconds,
                        formatted_time,
                        &row,
                    ))
                }
            }
        })
        .collect()
}

pub async fn get_match_timeline(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<Vec<MatchTimelineEventDto>> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    let clock_config = MatchClockDurationConfig::from_match_row(&match_row);
    let bundle = load_all_match_incidents(pool, match_id).await?;
    Ok(build_timeline(&clock_config, &bundle))
}
