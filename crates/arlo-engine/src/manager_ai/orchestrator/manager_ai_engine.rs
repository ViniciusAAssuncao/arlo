use crate::manager_ai::challenges::execute_challenge;
use crate::manager_ai::cognition::{derive_cooldown_seconds, ManagerDecisionKind};
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::play_calling::{
    execute_play_call_selection, rank_playbook, PlayCallDecisionEngine,
};
use crate::manager_ai::substitutions::{execute_substitutions, SubstitutionDecisionEngine};
use crate::manager_ai::tactical_adjustment::{
    execute_tactical_adjustment_by_id, TacticalAdjustmentDecisionEngine,
};
use crate::manager_ai::time_calls::{execute_time_call, TimeCallDecisionEngine};
use crate::time::DurationLedger;
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::situational::build_situational_context;
use arlo_events::EventSink;
use arlo_math::units::Duration;
use arlo_tactics::PlayCallCategory;
use rand::Rng;
use uuid::Uuid;

pub struct ManagerAiEngine;

impl ManagerAiEngine {
    pub fn on_stoppage<R: Rng + ?Sized>(
        publisher: &mut EventPublisher<'_, impl EventSink>,
        team_id: Uuid,
        rng: &mut R,
    ) -> Duration {
        let mut extra_dead_ball = Duration::new(0.0);
        let is_home = team_id == publisher.state().home_team_id();
        let period_duration_seconds = publisher.state().clock().period_duration_seconds();

        if let Some((call_team_id, call)) = publisher.state().last_reviewable_call().cloned() {
            if call_team_id == team_id {
                let context = ManagerDecisionContext::build(publisher.state(), team_id);
                let challenge_cooldown = derive_cooldown_seconds(
                    period_duration_seconds,
                    context.manager_snapshot.challenge_judgment,
                );
                if publisher.state().is_decision_ready(
                    team_id,
                    ManagerDecisionKind::Challenge,
                    challenge_cooldown,
                ) {
                    execute_challenge(publisher, &context, team_id, &call, rng);
                }
            }
        }

        let context = ManagerDecisionContext::build(publisher.state(), team_id);

        let sub_cooldown = derive_cooldown_seconds(
            period_duration_seconds,
            context.manager_snapshot.in_game_adjustments,
        );
        if publisher.state().is_decision_ready(
            team_id,
            ManagerDecisionKind::Substitution,
            sub_cooldown,
        ) {
            let lineup = if is_home {
                publisher.state().home_lineup().clone()
            } else {
                publisher.state().away_lineup().clone()
            };
            let squad = if is_home {
                publisher.state().home_squad().clone()
            } else {
                publisher.state().away_squad().clone()
            };
            let attribute_keys = publisher.state().attribute_keys().clone();
            let home_fatigue = publisher.state().home_fatigue().clone();
            let away_fatigue = publisher.state().away_fatigue().clone();
            let fatigue_lookup = |id: &Uuid| {
                home_fatigue
                    .get(id)
                    .or_else(|| away_fatigue.get(id))
                    .copied()
                    .unwrap_or_default()
            };

            let plans = SubstitutionDecisionEngine::evaluate_plans(
                &context,
                &context.squad_fatigue_summary,
                &lineup,
                &squad,
                &attribute_keys,
                fatigue_lookup,
                rng,
            );

            if !plans.is_empty() {
                if let Ok(executed) = execute_substitutions(publisher, team_id, &plans) {
                    if executed > 0 {
                        publisher
                            .state_mut()
                            .mark_decision_triggered(team_id, ManagerDecisionKind::Substitution);
                    }
                }
            }
        }

        let tac_cooldown = derive_cooldown_seconds(
            period_duration_seconds,
            context.manager_snapshot.adaptability,
        );
        if publisher.state().is_decision_ready(
            team_id,
            ManagerDecisionKind::TacticalAdjustment,
            tac_cooldown,
        ) {
            let available_profiles = publisher
                .state()
                .available_profiles_for_team(team_id)
                .to_vec();
            let active_profile_id = publisher.state().tactical_profile_for_team(team_id).id();
            let pitch_length_m = publisher.state().pitch().length().value();
            let scrimmage_x_m = publisher.state().possession().scrimmage_point().raw().0;
            let normalized_x_to_goal = if is_home {
                (scrimmage_x_m / pitch_length_m).clamp(0.0, 1.0)
            } else {
                ((pitch_length_m - scrimmage_x_m) / pitch_length_m).clamp(0.0, 1.0)
            };
            let situational_ctx =
                build_situational_context(publisher.state(), normalized_x_to_goal);

            if let Some(new_profile_id) = TacticalAdjustmentDecisionEngine::evaluate(
                &context,
                &available_profiles,
                active_profile_id,
                &situational_ctx,
                rng,
            ) {
                if execute_tactical_adjustment_by_id(
                    publisher,
                    team_id,
                    new_profile_id,
                    &available_profiles,
                ) {
                    publisher
                        .state_mut()
                        .mark_decision_triggered(team_id, ManagerDecisionKind::TacticalAdjustment);
                }
            }
        }

        let time_cooldown = derive_cooldown_seconds(
            period_duration_seconds,
            context.manager_snapshot.time_call_management,
        );
        if publisher.state().is_decision_ready(
            team_id,
            ManagerDecisionKind::TimeCall,
            time_cooldown,
        ) {
            let just_conceded = publisher.state().last_action_score_occurred()
                && publisher.state().last_scoring_team() != Some(team_id);
            if TimeCallDecisionEngine::evaluate(&context, just_conceded, rng) {
                let mut ledger = DurationLedger::new();
                if execute_time_call(publisher, team_id, is_home, &mut ledger) {
                    extra_dead_ball = extra_dead_ball + ledger.total_dead_ball();
                    publisher
                        .state_mut()
                        .mark_decision_triggered(team_id, ManagerDecisionKind::TimeCall);
                }
            }
        }

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
        let pitch_length_m = publisher.state().pitch().length().value();
        let scrimmage_x_m = publisher.state().possession().scrimmage_point().raw().0;
        let normalized_x_to_goal = if is_home {
            (scrimmage_x_m / pitch_length_m).clamp(0.0, 1.0)
        } else {
            ((pitch_length_m - scrimmage_x_m) / pitch_length_m).clamp(0.0, 1.0)
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
