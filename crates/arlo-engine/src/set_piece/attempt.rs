use crate::lineup_runtime::find_goalguard;
use crate::match_decision::scoring::{
    resolve_scoring_attempt,
    ScoringAttemptRequest,
    ScoringDecision,
    ScoringOpportunity,
};
use crate::possession::TouchActionType;
use crate::resolution::{ AttributedDuelOutcome, DuelKind };
use crate::set_piece::kicker_selection::select_kicker_from_tables;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::Player;
use rand::Rng;
use uuid::Uuid;

pub fn attempt_placed_kick<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    default_kicker: &Player,
    defense_players: &[&Player],
    opportunity: ScoringOpportunity,
    total_drives: u32,
    total_advance: f64,
    assister_id: Option<Uuid>,
    rng: &mut R
) -> Option<(ScoringDecision, AttributedDuelOutcome)> {
    let goalguard = match find_goalguard(defense_players) {
        Ok(g) => g,
        Err(_) => {
            return None;
        }
    };

    let mut candidates: Vec<&Player> = Vec::with_capacity(iter_ctx.target_candidates.len() + 1);
    if !iter_ctx.target_candidates.iter().any(|p| p.id() == default_kicker.id()) {
        candidates.push(default_kicker);
    }
    candidates.extend_from_slice(&iter_ctx.target_candidates);

    let kicker_id = select_kicker_from_tables(
        &candidates,
        Some(&context.offense_role_index),
        state.spatial_map(),
        state.pitch(),
        &context.offense_pos_index,
        &context.offense_instructions_index,
        state.teams.player_attribute_tables(),
        context.is_home_offense,
        &iter_ctx.openness_by_player,
        Some(&(|id: &Uuid| state.fatigue_lookup().get(id))),
        rng
    ).unwrap_or(default_kicker.id());

    let kicker = candidates
        .iter()
        .copied()
        .find(|p| p.id() == kicker_id)
        .unwrap_or(default_kicker);

    let kicker_pos = state
        .spatial_map()
        .get_position(&kicker.id())
        .unwrap_or(pass_phase.scrimmage_point);
    let shot_zone = state.pitch().zone_at_position(kicker_pos);
    let current_time = state.clock().seconds_in_period();
    state
        .possession_mut()
        .live_sequence_mut()
        .record_touch(kicker.id(), TouchActionType::FinishingAttempt, shot_zone, current_time);

    let finish_context = iter_ctx.duel_context.for_duel_kind(DuelKind::FieldGoalAttempt);
    let kicker_fatigue = state.fatigue_lookup().get(&kicker.id());
    let gg_fatigue = state.fatigue_lookup().get(&goalguard.id());
    let kicker_table = state.teams.player_attribute_tables().get(&kicker.id());
    let gg_table = state.teams.player_attribute_tables().get(&goalguard.id());

    let req = ScoringAttemptRequest::new(
        kicker,
        goalguard,
        state.attribute_keys(),
        context.offense_team_id,
        pass_phase.artrine.id(),
        assister_id,
        opportunity,
        total_drives,
        total_advance,
        &finish_context
    )
        .with_fatigue(kicker_fatigue, gg_fatigue)
        .with_tables(kicker_table, gg_table);

    let (score_dec, fin_duel) = resolve_scoring_attempt(req, rng);
    Some((score_dec, fin_duel))
}
