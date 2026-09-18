use crate::lineup_runtime::find_goalguard;
use crate::match_decision::scoring::{
    duel_kind_for_opportunity, resolve_scoring_attempt, ScoringAttemptRequest, ScoringDecision,
    ScoringOpportunity,
};
use crate::resolution::AttributedDuelOutcome;
use crate::set_piece::kicker_selection::select_kicker;
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
    designated_player: &Player,
    defense_players: &[&Player],
    opportunity: ScoringOpportunity,
    total_drives: u32,
    total_advance: f64,
    assister_id: Option<Uuid>,
    rng: &mut R,
) -> Option<(ScoringDecision, AttributedDuelOutcome)> {
    let goalguard = find_goalguard(defense_players).ok()?;

    let mut candidates = iter_ctx.target_candidates.clone();
    if !candidates.iter().any(|p| p.id() == designated_player.id()) {
        candidates.push(designated_player);
    }

    let tables = state.teams.player_attribute_tables();
    let kicker = select_kicker(
        &candidates,
        Some(&context.offense_role_index),
        tables,
        rng,
    )
    .unwrap_or(designated_player);

    let finish_ctx = iter_ctx
        .duel_context
        .for_duel_kind(duel_kind_for_opportunity(opportunity));
    let kicker_fatigue = state.fatigue_lookup().get(&kicker.id());
    let gg_fatigue = state.fatigue_lookup().get(&goalguard.id());
    let kicker_table = tables.get(&kicker.id());
    let gg_table = tables.get(&goalguard.id());

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
        &finish_ctx,
    )
    .with_fatigue(kicker_fatigue, gg_fatigue)
    .with_tables(kicker_table, gg_table);

    Some(resolve_scoring_attempt(req, rng))
}