use crate::lineup_runtime::find_goalguard;
use crate::match_decision::scoring::{
    duel_kind_for_opportunity, evaluate_scoring_opportunity, resolve_scoring_attempt,
    ScoringAttemptRequest, ScoringDecision, ScoringOpportunity,
};
use crate::resolution::AttributedDuelOutcome;
use crate::set_piece::attempt_placed_kick;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::Player;
use rand::Rng;

pub fn evaluate_and_attempt_scoring<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    finisher: &Player,
    passer: &Player,
    defense_players: &[&Player],
    drives_recorded: u32,
    advance_mirim: f64,
    fin_rating: f64,
    duels: &mut Vec<AttributedDuelOutcome>,
    rng: &mut R,
) -> ScoringDecision {
    let total_drives = state.drives_in_current_series() + drives_recorded;
    let total_adv = state.possession().series_state().advanced_mirins() + advance_mirim;

    let opportunity = evaluate_scoring_opportunity(
        state.possession().is_bonus_phase(),
        total_drives,
        total_adv,
        fin_rating,
    );

    if opportunity == ScoringOpportunity::None {
        return ScoringDecision::NoOpportunity;
    }

    if matches!(
        opportunity,
        ScoringOpportunity::FieldPoint | ScoringOpportunity::FieldGoal(_)
    ) {
        let assister_id = state
            .possession()
            .live_sequence()
            .primary_assister(finisher.id())
            .or_else(|| Some(passer.id()));

        if let Some((score_dec, fin_duel)) = attempt_placed_kick(
            state,
            context,
            iter_ctx,
            pass_phase,
            finisher,
            defense_players,
            opportunity,
            total_drives,
            total_adv,
            assister_id,
            rng,
        ) {
            duels.push(fin_duel);
            return score_dec;
        }
    }

    let goalguard = match find_goalguard(defense_players) {
        Ok(g) => g,
        Err(_) => return ScoringDecision::NoOpportunity,
    };

    let finish_ctx = iter_ctx
        .duel_context
        .for_duel_kind(duel_kind_for_opportunity(opportunity));
    let fin_fatigue = state.fatigue_lookup().get(&finisher.id());
    let gg_fatigue = state.fatigue_lookup().get(&goalguard.id());
    let fin_table = state.teams.player_attribute_tables().get(&finisher.id());
    let gg_table = state.teams.player_attribute_tables().get(&goalguard.id());

    let assister_id = state
        .possession()
        .live_sequence()
        .primary_assister(finisher.id())
        .or_else(|| Some(passer.id()));

    let req = ScoringAttemptRequest::new(
        finisher,
        goalguard,
        state.attribute_keys(),
        context.offense_team_id,
        pass_phase.artrine.id(),
        assister_id,
        opportunity,
        total_drives,
        total_adv,
        &finish_ctx,
    )
    .with_fatigue(fin_fatigue, gg_fatigue)
    .with_tables(fin_table, gg_table);

    let (score_dec, fin_duel) = resolve_scoring_attempt(req, rng);
    duels.push(fin_duel);
    score_dec
}
