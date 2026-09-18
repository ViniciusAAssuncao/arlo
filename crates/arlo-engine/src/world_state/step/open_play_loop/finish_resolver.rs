use crate::artrine::ArtrineExecutionOutcome;
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

pub fn resolve_finish<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    current_carrier: &Player,
    defense_players: &[&Player],
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let (att_prof, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let fin_rating = calculate_player_duel_rating_from_table(
        current_carrier,
        Position::CenterOffense,
        state.attribute_table_for(&current_carrier.id()),
        att_prof,
        &state.fatigue_lookup().get(&current_carrier.id()),
    );

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::FinishingEngagement,
        Duration::new(22.0),
    );

    let mut duels = Vec::new();
    let scoring_decision = evaluate_and_attempt_scoring(
        state,
        context,
        iter_ctx,
        pass_phase,
        current_carrier,
        current_carrier,
        defense_players,
        0,
        0.0,
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
        mirins_advanced: 0.0,
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
        receiver_id: Some(current_carrier.id()),
        distribution_flight: None,
    }
}
