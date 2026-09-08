use crate::match_decision::scoring::ScoringDecision;
use crate::possession::PossessionSnapshot;
use crate::world_state::match_state::MatchState;
use arlo_domain::{Player, Position as DomainPosition};
use arlo_math::units::Position as VectorPosition;
use uuid::Uuid;

pub fn find_defense_goalguard<'a>(defense_players: &[&'a Player]) -> &'a Player {
    defense_players
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == DomainPosition::Goalguard && pos.proficiency() > 0)
        })
        .unwrap_or(defense_players[0])
}

pub fn publish_scoring_impulse(
    state: &mut MatchState,
    scoring_decision: &ScoringDecision,
    finisher_id: Uuid,
    defense_players: &[&Player],
    offense_players: &[&Player],
) {
    if scoring_decision.is_scored()
        || matches!(scoring_decision, ScoringDecision::Missed { .. })
    {
        let goalguard = find_defense_goalguard(defense_players);
        state.impulse_bus_mut().publish_scoring_decision(
            scoring_decision,
            finisher_id,
            goalguard.id(),
            0.5,
            offense_players,
            defense_players,
        );
    }
}

pub fn apply_match_score(
    state: &mut MatchState,
    offense_team_id: Uuid,
    scoring_decision: &ScoringDecision,
) {
    match scoring_decision {
        ScoringDecision::GoalPoint { .. } => {
            state.record_goal_point(offense_team_id);
        }
        ScoringDecision::FieldPoint { .. } => {
            state.record_field_point(offense_team_id);
        }
        ScoringDecision::FieldGoal { post, .. } => {
            state.record_field_goal(offense_team_id, *post);
        }
        _ => {}
    }
}

pub fn post_transition_score_reset(
    state: &mut MatchState,
    scoring_decision: &ScoringDecision,
    is_possession_change: bool,
    mut next_snapshot: PossessionSnapshot,
) -> PossessionSnapshot {
    if scoring_decision.is_scored() || is_possession_change {
        state.reset_drives();
    }

    if scoring_decision.is_scored() {
        let center_scrimmage = VectorPosition::from_components(
            state.pitch().length().value() / 2.0,
            state.pitch().width().value() / 2.0,
            0.0,
        );
        next_snapshot.series_state_mut().reset(center_scrimmage);
        if matches!(scoring_decision, ScoringDecision::GoalPoint { .. }) {
            next_snapshot.series_state_mut().is_bonus_phase = true;
        }
    }

    next_snapshot
}