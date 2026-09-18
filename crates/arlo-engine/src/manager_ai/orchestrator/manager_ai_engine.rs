use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::orchestrator::foul_challenge_stage::evaluate_foul_challenge_stage;
use crate::manager_ai::orchestrator::injury_substitution_stage::evaluate_injury_substitution_stage;
use crate::manager_ai::orchestrator::kick_foul_realignment_stage::evaluate_kick_foul_realignment_stage;
use crate::manager_ai::orchestrator::reviewable_call_stage::evaluate_reviewable_call_challenge_stage;
use crate::manager_ai::orchestrator::substitution_stage::evaluate_substitution_stage;
use crate::manager_ai::orchestrator::tactical_adjustment_stage::evaluate_tactical_adjustment_stage;
use crate::manager_ai::orchestrator::time_call_stage::evaluate_time_call_stage;
use crate::manager_ai::play_calling::{
    execute_play_call_selection, rank_playbook, PlayCallDecisionEngine,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::situational::build_situational_context;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use arlo_math::units::Duration;
use arlo_tactics::PlayCallCategory;
use rand::Rng;
use uuid::Uuid;

pub struct ManagerAiEngine;

impl ManagerAiEngine {
    pub fn on_stoppage<R: Rng + ?Sized>(
        publisher: &mut EventPublisher<'_, impl EventSink>,
        team_id: Uuid,
        manager_decision_inbox: &ManagerDecisionInbox,
        rng: &mut R,
    ) -> Duration {
        let mut extra_dead_ball = Duration::new(0.0);
        let is_home = team_id == publisher.state().home_team_id();
        let period_duration_seconds = publisher.state().clock().period_duration_seconds();

        evaluate_reviewable_call_challenge_stage(
            publisher,
            team_id,
            period_duration_seconds,
            manager_decision_inbox,
            rng,
        );

        evaluate_foul_challenge_stage(
            publisher,
            team_id,
            period_duration_seconds,
            manager_decision_inbox,
            rng,
        );

        evaluate_injury_substitution_stage(publisher, team_id, manager_decision_inbox);

        evaluate_substitution_stage(
            publisher,
            team_id,
            period_duration_seconds,
            manager_decision_inbox,
            rng,
        );

        evaluate_tactical_adjustment_stage(
            publisher,
            team_id,
            period_duration_seconds,
            manager_decision_inbox,
            rng,
        );

        let realignment_dead_ball = evaluate_kick_foul_realignment_stage(
            publisher,
            team_id,
            is_home,
            manager_decision_inbox,
            rng,
        );
        extra_dead_ball = extra_dead_ball + realignment_dead_ball;

        let time_call_dead_ball = evaluate_time_call_stage(
            publisher,
            team_id,
            is_home,
            period_duration_seconds,
            manager_decision_inbox,
            rng,
        );
        extra_dead_ball = extra_dead_ball + time_call_dead_ball;

        extra_dead_ball
    }

    pub fn on_down_start<R: Rng + ?Sized>(
        publisher: &mut EventPublisher<'_, impl EventSink>,
        offense_team_id: Uuid,
        last_play_call_id: Option<Uuid>,
        last_play_failed: bool,
        rng: &mut R,
    ) {
        if let Some(last_id) = last_play_call_id {
            publisher.state_mut().record_play_call_outcome(
                offense_team_id,
                last_id,
                !last_play_failed,
            );
        }

        if publisher.state().has_queued_call_for_offense() {
            return;
        }

        let context = ManagerDecisionContext::build(publisher.state(), offense_team_id);
        let playbook = publisher
            .state()
            .playbook_for_team(offense_team_id)
            .to_vec();
        if playbook.is_empty() {
            return;
        }

        let is_home = offense_team_id == publisher.state().home_team_id();
        let pitch_length_mirim = publisher.state().pitch().length_mirim();
        let scrimmage_x_mirim = publisher.state().possession().scrimmage_x_mirim();
        let normalized_x_to_goal = if is_home {
            (scrimmage_x_mirim / pitch_length_mirim).clamp(0.0, 1.0)
        } else {
            ((pitch_length_mirim - scrimmage_x_mirim) / pitch_length_mirim).clamp(0.0, 1.0)
        };
        let situational_ctx = build_situational_context(publisher.state(), normalized_x_to_goal);
        let expected_category = if publisher.state().possession().is_bonus_phase() {
            PlayCallCategory::BonusPhaseConversion
        } else {
            PlayCallCategory::OpenPlay
        };

        let ranked = rank_playbook(&playbook, &situational_ctx, expected_category);
        if ranked.is_empty() {
            return;
        }

        publisher.state_mut().ensure_play_call_beliefs_seeded(
            offense_team_id,
            &playbook,
            &ranked,
            &context.manager_snapshot,
        );

        let efficacy_snapshot = publisher
            .state()
            .play_call_efficacy_snapshot(offense_team_id);

        if let Some(selected_play_call) = PlayCallDecisionEngine::select_next(
            &context,
            &playbook,
            &ranked,
            &efficacy_snapshot,
            last_play_call_id,
            last_play_failed,
            rng,
        ) {
            execute_play_call_selection(publisher, offense_team_id, selected_play_call);
        }
    }
}
