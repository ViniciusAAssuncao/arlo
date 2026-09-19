use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::lineup_runtime::find_goalguard;
use crate::match_decision::scoring::{
    can_attempt_field_point, duel_kind_for_opportunity, evaluate_scoring_opportunity,
    resolve_scoring_attempt, ScoringAttemptRequest, ScoringDecision, ScoringOpportunity,
};
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::AttributedDuelOutcome;
use crate::resolution::DuelKind;
use crate::set_piece::select_kicker;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use crate::world_state::step::down_resolution::progression_stage::ActionProgressionOutcome;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Position};
use rand::Rng;
use uuid::Uuid;

pub fn resolve_scoring<R: Rng + ?Sized>(
    ctx: &DownResolutionContext<'_>,
    decision: ArtrineDecisionKind,
    contest: &ActionContestOutcome<'_>,
    progression: &ActionProgressionOutcome,
    state: &mut MatchState,
    call_context: &CallToActionContext,
    pass_phase: &PassPhaseResult<'_>,
    duels: &mut Vec<AttributedDuelOutcome>,
    rng: &mut R,
) -> ScoringDecision {
    if contest.turnover_team.is_some() {
        return ScoringDecision::NoOpportunity;
    }

    let is_scoring_action =
        decision == ArtrineDecisionKind::SelfFinish || decision == ArtrineDecisionKind::Cross;

    if !is_scoring_action {
        return ScoringDecision::NoOpportunity;
    }

    if decision == ArtrineDecisionKind::Cross && !contest.attacker_won {
        return ScoringDecision::NoOpportunity;
    }

    let tables = &ctx.attribute_tables;
    let total_drives = ctx.drives_in_series + progression.drives_recorded;
    let total_adv = ctx.state_advanced_mirins + progression.mirins_advanced;

    let finisher = contest.receiver.unwrap_or(ctx.carrier);
    let (att_prof, _) = get_duel_profiles(DuelKind::FinishingAttempt);
    let finisher_table = tables
        .get(&finisher.id())
        .copied()
        .unwrap_or_else(|| *state.attribute_table_for(&finisher.id()));

    let fin_fatigue = state.fatigue_lookup().get(&finisher.id());
    let fin_exhaustion = calculate_physical_exhaustion(&fin_fatigue);

    let fin_rating = calculate_player_duel_rating_from_table(
        finisher,
        Position::CenterOffense,
        &finisher_table,
        &att_prof,
        &fin_fatigue,
    ) * ctx.artrine_axis_multiplier;

    let mut opportunity = evaluate_scoring_opportunity(
        ctx.is_bonus_phase,
        total_drives,
        total_adv,
        progression.new_normalized_proximity,
        fin_rating,
    );

    if opportunity == ScoringOpportunity::GoalPoint {
        let can_field_point = can_attempt_field_point(
            total_drives,
            total_adv,
            progression.new_normalized_proximity,
        );
        if can_field_point {
            let under_defensive_pressure = progression.new_normalized_proximity < 0.88
                || ctx.team_advantage < -1.5
                || fin_rating < 11.5
                || ctx.down >= 3
                || fin_exhaustion > 0.40;
            if under_defensive_pressure {
                opportunity = ScoringOpportunity::FieldPoint;
            }
        }
    }

    if opportunity == ScoringOpportunity::None {
        return ScoringDecision::NoOpportunity;
    }

    let goalguard = match find_goalguard(&ctx.defense_players) {
        Ok(g) => g,
        Err(_) => {
            return ScoringDecision::NoOpportunity;
        }
    };

    let finish_ctx = ctx.duel_context.for_duel_kind(duel_kind_for_opportunity(opportunity));

    let effective_kicker = if matches!(
        opportunity,
        ScoringOpportunity::FieldPoint | ScoringOpportunity::FieldGoal(_)
    ) {
        let mut candidates = ctx.target_candidates.clone();
        if !candidates.iter().any(|p| p.id() == finisher.id()) {
            candidates.push(finisher);
        }
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

    let kicker_fatigue = state.fatigue_lookup().get(&effective_kicker.id());
    let gg_fatigue = state.fatigue_lookup().get(&goalguard.id());
    let fin_table = tables.get(&effective_kicker.id());
    let gg_table = tables.get(&goalguard.id());

    let assister_id = state
        .possession()
        .live_sequence()
        .primary_assister(effective_kicker.id())
        .or_else(|| Some(ctx.carrier.id()));

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
        &finish_ctx,
    )
    .with_fatigue(kicker_fatigue, gg_fatigue)
    .with_tables(fin_table, gg_table);

    let (score_dec, fin_duel) = resolve_scoring_attempt(req, rng);
    duels.push(fin_duel);
    score_dec
}