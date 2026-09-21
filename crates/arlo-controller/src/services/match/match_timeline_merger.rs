use crate::services::r#match::match_clock_math::{
    calculate_total_elapsed_seconds, MatchClockDurationConfig,
};
use arlo_persistence::models::incidents::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "camelCase")]
pub enum MatchTimelineEventData {
    ScoringPlay(MatchScoringPlayRow),
    Foul(MatchFoulRow),
    Injury(MatchInjuryRow),
    Substitution(MatchSubstitutionRow),
    Turnover(MatchTurnoverRow),
    KickFoulAward(MatchKickFoulAwardRow),
    KickFoulDecision(MatchKickFoulDecisionRow),
    TimeCall(MatchTimeCallRow),
    Challenge(MatchChallengeRow),
    TacticalProfileActivation(MatchTacticalProfileActivationRow),
    PlayCallSelection(MatchPlayCallSelectionRow),
    AvailabilityChange(MatchAvailabilityChangeRow),
    ImpulseCritical(MatchImpulseCriticalEventRow),
    AddedTime(MatchAddedTimeRow),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTimelineItem {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub event: MatchTimelineEventData,
}

#[derive(Debug, Clone, Default)]
pub struct MatchIncidentsBundle<'a> {
    pub scoring_plays: &'a [MatchScoringPlayRow],
    pub fouls: &'a [MatchFoulRow],
    pub injuries: &'a [MatchInjuryRow],
    pub substitutions: &'a [MatchSubstitutionRow],
    pub turnovers: &'a [MatchTurnoverRow],
    pub kick_foul_awards: &'a [MatchKickFoulAwardRow],
    pub kick_foul_decisions: &'a [MatchKickFoulDecisionRow],
    pub time_calls: &'a [MatchTimeCallRow],
    pub challenges: &'a [MatchChallengeRow],
    pub tactical_profile_activations: &'a [MatchTacticalProfileActivationRow],
    pub play_call_selections: &'a [MatchPlayCallSelectionRow],
    pub availability_changes: &'a [MatchAvailabilityChangeRow],
    pub impulse_critical_events: &'a [MatchImpulseCriticalEventRow],
    pub added_time_awards: &'a [MatchAddedTimeRow],
}

pub fn merge_match_timeline(
    clock_config: &MatchClockDurationConfig,
    incidents: MatchIncidentsBundle<'_>,
) -> Vec<MatchTimelineItem> {
    let total_count = incidents.scoring_plays.len()
        + incidents.fouls.len()
        + incidents.injuries.len()
        + incidents.substitutions.len()
        + incidents.turnovers.len()
        + incidents.kick_foul_awards.len()
        + incidents.kick_foul_decisions.len()
        + incidents.time_calls.len()
        + incidents.challenges.len()
        + incidents.tactical_profile_activations.len()
        + incidents.play_call_selections.len()
        + incidents.availability_changes.len()
        + incidents.impulse_critical_events.len()
        + incidents.added_time_awards.len();

    let mut items = Vec::with_capacity(total_count);

    for row in incidents.scoring_plays {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::ScoringPlay(row.clone()),
        });
    }

    for row in incidents.fouls {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::Foul(row.clone()),
        });
    }

    for row in incidents.injuries {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::Injury(row.clone()),
        });
    }

    for row in incidents.substitutions {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::Substitution(row.clone()),
        });
    }

    for row in incidents.turnovers {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::Turnover(row.clone()),
        });
    }

    for row in incidents.kick_foul_awards {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::KickFoulAward(row.clone()),
        });
    }

    for row in incidents.kick_foul_decisions {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::KickFoulDecision(row.clone()),
        });
    }

    for row in incidents.time_calls {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::TimeCall(row.clone()),
        });
    }

    for row in incidents.challenges {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::Challenge(row.clone()),
        });
    }

    for row in incidents.tactical_profile_activations {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::TacticalProfileActivation(row.clone()),
        });
    }

    for row in incidents.play_call_selections {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::PlayCallSelection(row.clone()),
        });
    }

    for row in incidents.availability_changes {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::AvailabilityChange(row.clone()),
        });
    }

    for row in incidents.impulse_critical_events {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::ImpulseCritical(row.clone()),
        });
    }

    for row in incidents.added_time_awards {
        let period = row.period.max(0) as u32;
        let seconds_in_period = row.seconds_in_period.max(0.0);
        let total_elapsed = calculate_total_elapsed_seconds(clock_config, period, seconds_in_period);
        items.push(MatchTimelineItem {
            sequence_number: row.sequence_number.max(0) as u64,
            period,
            seconds_in_period,
            total_elapsed_seconds: total_elapsed,
            event: MatchTimelineEventData::AddedTime(row.clone()),
        });
    }

    items.sort_by_key(|item| item.sequence_number);
    items
}

#[allow(clippy::too_many_arguments)]
pub fn merge_match_timeline_from_slices(
    clock_config: &MatchClockDurationConfig,
    scoring_plays: &[MatchScoringPlayRow],
    fouls: &[MatchFoulRow],
    injuries: &[MatchInjuryRow],
    substitutions: &[MatchSubstitutionRow],
    turnovers: &[MatchTurnoverRow],
    kick_foul_awards: &[MatchKickFoulAwardRow],
    kick_foul_decisions: &[MatchKickFoulDecisionRow],
    time_calls: &[MatchTimeCallRow],
    challenges: &[MatchChallengeRow],
    tactical_profile_activations: &[MatchTacticalProfileActivationRow],
    play_call_selections: &[MatchPlayCallSelectionRow],
    availability_changes: &[MatchAvailabilityChangeRow],
    impulse_critical_events: &[MatchImpulseCriticalEventRow],
    added_time_awards: &[MatchAddedTimeRow],
) -> Vec<MatchTimelineItem> {
    merge_match_timeline(
        clock_config,
        MatchIncidentsBundle {
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
        },
    )
}