use crate::manager_ai::challenges::decision::ChallengeDecisionEngine;
use crate::manager_ai::challenges::perception::perceives_bad_call;
use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::officiating::resolution::resolve_true_ruling;
use crate::officiating::{ReviewableCall, ReviewableCallKind};
use crate::possession::transition::handle_turnover_without_out;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_math::units::Position as VectorPosition;
use rand::Rng;
use uuid::Uuid;

pub fn reverse_out_of_bounds_ruling(state: &mut MatchState, previous_scrimmage: VectorPosition) {
    state
        .possession_mut()
        .series_state_mut()
        .reset(previous_scrimmage);
}

pub fn execute_challenge<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    context: &ManagerDecisionContext,
    team_id: Uuid,
    call: &ReviewableCall,
    rng: &mut R,
) -> bool {
    let perceived_bad = perceives_bad_call(
        call,
        context.manager_snapshot.challenge_judgment,
        context.manager_snapshot.judging_ability,
        rng,
    );

    if !ChallengeDecisionEngine::evaluate(context, call, perceived_bad, rng) {
        return false;
    }

    let success = resolve_true_ruling(call, rng);
    let is_home = context.is_home;
    let used = publisher
        .state_mut()
        .clock_mut()
        .use_challenge(is_home, success);
    if !used {
        return false;
    }

    publisher
        .state_mut()
        .mark_decision_triggered(team_id, ManagerDecisionKind::Challenge);

    if success {
        match call.kind() {
            ReviewableCallKind::TurnoverClassification => {
                let snap = publisher.state().possession().clone();
                let turnover_team = if is_home {
                    publisher.state().away_team_id()
                } else {
                    publisher.state().home_team_id()
                };
                let transition_res = handle_turnover_without_out(&snap, turnover_team);
                *publisher.state_mut().possession_mut() = transition_res.snapshot;
            }
            ReviewableCallKind::OutOfBoundsClassification => {
                let prev_scrimmage = publisher.state().possession().scrimmage_point();
                reverse_out_of_bounds_ruling(publisher.state_mut(), prev_scrimmage);
            }
            ReviewableCallKind::DriveValidity => {
                publisher.state_mut().reverse_drive();
            }
            ReviewableCallKind::FoulClassification => {}
        }
    }

    let remaining_challenges_after = if is_home {
        publisher.state().clock().home_challenges()
    } else {
        publisher.state().clock().away_challenges()
    };

    publisher.emit_challenge_resolved(team_id, call.kind(), success, remaining_challenges_after);

    publisher.state_mut().clear_last_reviewable_call();

    success
}