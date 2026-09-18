use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::target_selection::select_finisher;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::DuelKind;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::scoring_attempt_evaluator::evaluate_and_attempt_scoring;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{Player, Position};
use arlo_math::units::{Duration, Position as VectorPosition};
use rand::Rng;
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_cross<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    current_carrier: &Player,
    defense_players: &[&Player],
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let finisher_id = select_finisher(
        &iter_ctx.target_candidates,
        Some(&context.offense_role_index),
        state.pitch(),
        &context.offense_pos_index,
        &context.offense_instructions_index,
        state.teams.player_attribute_tables(),
        context.is_home_offense,
        &HashMap::new(),
        Some(&|id: &Uuid| state.fatigue_lookup().get(id)),
        rng,
    )
    .unwrap_or(current_carrier.id());

    let finisher = iter_ctx
        .target_candidates
        .iter()
        .copied()
        .find(|p| p.id() == finisher_id)
        .unwrap_or(current_carrier);

    let (att_prof, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let fin_rating = calculate_player_duel_rating_from_table(
        finisher,
        Position::CenterOffense,
        state.attribute_table_for(&finisher.id()),
        att_prof,
        &state.fatigue_lookup().get(&finisher.id()),
    );

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::CrossFlight,
        Duration::new(26.0),
    );

    let mut duels = Vec::new();
    let scoring_decision = evaluate_and_attempt_scoring(
        state,
        context,
        iter_ctx,
        pass_phase,
        finisher,
        current_carrier,
        defense_players,
        0,
        15.0,
        fin_rating,
        &mut duels,
        rng,
    );

    let pitch_len_m = state.pitch().length().value();
    let goal_x_m = if context.is_home_offense {
        pitch_len_m
    } else {
        0.0
    };
    let end_position = VectorPosition::from_components(
        goal_x_m,
        state.pitch().width().value() * 0.5,
        0.0,
    );

    ArtrineExecutionOutcome {
        mirins_advanced: 15.0,
        drives_recorded: 0,
        drive_row_indices: SmallVec::new(),
        turnover: None,
        recovering_player_id: None,
        scoring_decision,
        duration_ledger: ledger,
        end_position,
        duels,
        fouls: Vec::new(),
        injuries: Vec::new(),
        receiver_id: Some(finisher.id()),
        distribution_flight: None,
    }
}
