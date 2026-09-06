use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::possession::{transition, TransitionResult, TurnoverCategory};
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use arlo_events::CountdownReason;
use uuid::Uuid;

pub struct PlayClassification {
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub lost_by_player_id: Option<Uuid>,
    pub out_of_bounds: bool,
    pub arbitral_stoppage: bool,
}

pub fn classify_play_outcome(
    pass_phase: &PassPhaseResult<'_>,
    execution_outcome: &ArtrineExecutionOutcome,
    defense_team_id: Uuid,
) -> PlayClassification {
    let is_scored = execution_outcome.scoring_decision.is_scored();
    let is_missed = matches!(
        execution_outcome.scoring_decision,
        ScoringDecision::Missed { .. }
    );
    let pass_failed = !pass_phase.pass_completed;
    let is_distribution_dropped = execution_outcome
        .distribution_flight
        .as_ref()
        .map_or(false, |f| !f.caught && execution_outcome.turnover.is_none());

    let out_of_bounds = pass_failed || is_scored || is_missed || is_distribution_dropped;
    let arbitral_stoppage = is_scored;

    let (turnover, recovering_player_id, lost_by_player_id) = if is_missed {
        let category = TurnoverCategory::MissedShot;
        let lost_by = match &execution_outcome.scoring_decision {
            ScoringDecision::Missed { scorer_id, .. } => Some(*scorer_id),
            _ => execution_outcome
                .receiver_id
                .or(Some(pass_phase.artrine.id())),
        };
        (
            Some(defense_team_id),
            category.sanitize_recovering_player(None),
            lost_by,
        )
    } else if let Some(turnover_team) = execution_outcome.turnover {
        let category = if execution_outcome
            .distribution_flight
            .as_ref()
            .map_or(false, |f| !f.caught)
        {
            TurnoverCategory::Interception
        } else {
            TurnoverCategory::Dispossession
        };
        let lost_by = execution_outcome
            .receiver_id
            .or(Some(pass_phase.artrine.id()));
        (
            Some(turnover_team),
            category.sanitize_recovering_player(execution_outcome.recovering_player_id),
            lost_by,
        )
    } else {
        (None, None, None)
    };

    PlayClassification {
        turnover,
        recovering_player_id,
        lost_by_player_id,
        out_of_bounds,
        arbitral_stoppage,
    }
}

pub fn build_detailed_play_outcome(
    pass_phase: &PassPhaseResult<'_>,
    execution_outcome: &ArtrineExecutionOutcome,
    classification: &PlayClassification,
    resolved_duels: Vec<AttributedDuelOutcome>,
    possession_control_seconds: Option<f64>,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
) -> DetailedPlayOutcome {
    DetailedPlayOutcome {
        offense_team_id,
        defense_team_id,
        passer_id: pass_phase.passer.id(),
        artrine_id: pass_phase.artrine.id(),
        down_number: pass_phase.down_number,
        scrimmage_x_mirim: pass_phase.scrimmage_x_mirim,
        pass_completed: pass_phase.pass_completed,
        pass_is_aerial: pass_phase.is_aerial,
        reception_point: pass_phase.reception_point,
        drives_recorded: execution_outcome.drives_recorded,
        mirins_advanced: execution_outcome.mirins_advanced,
        duels: resolved_duels,
        turnover: classification.turnover,
        recovering_player_id: classification.recovering_player_id,
        lost_by_player_id: classification.lost_by_player_id,
        out_of_bounds: classification.out_of_bounds,
        arbitral_stoppage: classification.arbitral_stoppage,
        last_valid_possession_point: execution_outcome.end_position,
        possession_control_seconds,
        scoring_decision: execution_outcome.scoring_decision.clone(),
    }
}

pub fn resolve_possession_transition(
    state: &MatchState,
    detailed_outcome: &DetailedPlayOutcome,
) -> TransitionResult {
    let possession_outcome = detailed_outcome.to_possession_outcome();
    transition(state.possession(), &possession_outcome)
}

pub fn determine_countdown_reason(
    detailed_outcome: &DetailedPlayOutcome,
    is_possession_change: bool,
) -> CountdownReason {
    if detailed_outcome.scoring_decision.is_scored() {
        CountdownReason::AfterScore
    } else if detailed_outcome.turnover.is_some() {
        CountdownReason::OutOfBoundsAfterTurnover
    } else if is_possession_change {
        CountdownReason::TurnoverOnDowns
    } else {
        CountdownReason::OutOfBoundsPlayEnd
    }
}