use crate::lineup_runtime::find_goalguard;
use crate::match_decision::{
    duel_kind_for_opportunity, resolve_scoring_attempt, ScoringAttemptRequest, ScoringDecision,
    ScoringOpportunity,
};
use crate::resolution::AttributedDuelOutcome;
use crate::scoring_model::margin::MarginContext;
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

    let is_home = ctx.offense_team_id == state.home_team_id();
    let (offense_score, defense_score) = if is_home {
        (
            state.home_score().total_points,
            state.away_score().total_points,
        )
    } else {
        (
            state.away_score().total_points,
            state.home_score().total_points,
        )
    };
    let diff_pts = (offense_score as f64) - (defense_score as f64);
    let advantage_gp = diff_pts / (arlo_domain::sport_constants::GOAL_POINT_VALUE as f64);

    let offense_power = state.power_for_team(ctx.offense_team_id);
    let defense_power = state.power_for_team(ctx.defense_team_id);
    let league_scale = &state.tuning().league_strength_scale;
    let mut z_gap = crate::team_strength::calculate_strength_z_gap(
        offense_power.offensive_power(),
        defense_power.defensive_power(),
        league_scale.offense_mean,
        league_scale.offense_stddev,
        league_scale.defense_mean,
        league_scale.defense_stddev,
    );
    if ctx.duel_context.attacker_is_home() {
        z_gap += state.tuning().home_advantage_profile.power_z_boost();
    }
    if ctx.duel_context.defender_is_home() {
        z_gap -= state.tuning().home_advantage_profile.power_z_boost();
    }

    let margin_ctx = MarginContext::new(advantage_gp, z_gap);

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
    .with_difficulty_profile(difficulty_profile)
    .with_margin(margin_ctx);

    let (score_dec, fin_duel) = resolve_scoring_attempt(req, rng);
    duels.push(fin_duel);
    score_dec
}
