use crate::lineup_runtime::find_goalguard;
use crate::match_decision::{
    duel_kind_for_opportunity, resolve_scoring_attempt, ScoringAttemptRequest, ScoringDecision,
    ScoringOpportunity,
};
use crate::resolution::AttributedDuelOutcome;
use crate::scoring_regime::evaluate_scoring_opportunity;
use crate::set_piece::select_kicker;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use crate::world_state::step::down_resolution::progression_stage::ActionProgressionOutcome;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::ArtrineDecisionKind;
use rand::Rng;
use uuid::Uuid;

pub fn resolve_scoring<R: Rng + ?Sized>(
    ctx: &DownResolutionContext<'_>,
    decision: ArtrineDecisionKind,
    contest: &ActionContestOutcome<'_>,
    progression: &ActionProgressionOutcome,
    state: &MatchState,
    call_context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    duels: &mut Vec<AttributedDuelOutcome>,
    rng: &mut R,
) -> ScoringDecision {
    if contest.turnover_team.is_some() {
        return ScoringDecision::NoOpportunity;
    }

    let is_scoring_action = decision == ArtrineDecisionKind::SelfFinish
        || decision == ArtrineDecisionKind::Cross;

    if !is_scoring_action {
        return ScoringDecision::NoOpportunity;
    }

    let total_drives = ctx.drives_in_series + progression.drives_recorded;
    let total_adv = ctx.possession_advanced_mirins + progression.mirins_advanced;

    let opportunity = evaluate_scoring_opportunity(
        &ctx.scoring_regime,
        ctx.is_bonus_phase,
        total_drives,
        total_adv,
    );

    if opportunity == ScoringOpportunity::None {
        return ScoringDecision::NoOpportunity;
    }

    let goalguard = match find_goalguard(&ctx.defense_players) {
        Ok(g) => g,
        Err(_) => return ScoringDecision::NoOpportunity,
    };

    let finish_ctx = ctx
        .duel_context
        .for_duel_kind(duel_kind_for_opportunity(opportunity));

    let finisher = contest.receiver.unwrap_or(ctx.carrier);

    let effective_kicker = if matches!(
        opportunity,
        ScoringOpportunity::FieldPoint | ScoringOpportunity::FieldGoal
    ) {
        let mut candidates = ctx.target_candidates.clone();
        if !candidates.iter().any(|p| p.id() == finisher.id()) {
            candidates.push(finisher);
        }
        let tables = state.teams.player_attribute_tables();
        let fatigue_lookup = state.fatigue_lookup();
        let fatigue_for = |id: &Uuid| fatigue_lookup.get(id);
        select_kicker(
            &candidates,
            Some(&call_context.offense_role_index),
            state.pitch(),
            &call_context.offense_pos_index,
            &call_context.offense_instructions_index,
            tables,
            call_context.is_home_offense,
            Some(&fatigue_for),
            rng,
        )
        .unwrap_or(finisher)
    } else {
        finisher
    };

    let fin_fatigue = state.fatigue_lookup().get(&effective_kicker.id());
    let gg_fatigue = state.fatigue_lookup().get(&goalguard.id());
    let fin_table = state.teams.player_attribute_tables().get(&effective_kicker.id());
    let gg_table = state.teams.player_attribute_tables().get(&goalguard.id());

    let assister_id = state
        .possession()
        .live_sequence()
        .primary_assister(effective_kicker.id())
        .or_else(|| Some(ctx.carrier.id()));

    let defense_closed = progression.new_normalized_proximity >= 0.75
        && state
            .instructions_for_team(ctx.defense_team_id)
            .out_of_possession()
            .defensive_line_height()
            .value()
            < 0.5;

    let difficulty_profile = state.tuning().scoring_difficulty;

    let req = ScoringAttemptRequest::new(
        effective_kicker,
        goalguard,
        state.attribute_keys(),
        ctx.offense_team_id,
        pass_phase.artrine.id(),
        assister_id,
        opportunity,
        total_drives,
        total_adv,
        progression.new_normalized_proximity,
        &finish_ctx,
    )
    .with_fatigue(fin_fatigue, gg_fatigue)
    .with_tables(fin_table, gg_table)
    .with_defense_closed(defense_closed)
    .with_difficulty_profile(difficulty_profile);

    let (score_dec, fin_duel) = resolve_scoring_attempt(req, rng);
    duels.push(fin_duel);
    score_dec
}