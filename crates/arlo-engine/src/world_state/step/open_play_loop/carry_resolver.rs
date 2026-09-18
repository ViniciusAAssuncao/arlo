use crate::artrine::execution::detect_drive_crossings_arithmetic;
use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{
    sample_action_progression, ActionProgressionKind, AttributedDuelOutcome, DuelKind,
};
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::contact_event_evaluator::evaluate_contact_events;
use crate::world_state::step::open_play_loop::scoring_attempt_evaluator::evaluate_and_attempt_scoring;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{Player, Position};
use arlo_math::stats::contrast::logistic;
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use arlo_math::Probability;
use rand::Rng;
use smallvec::{smallvec, SmallVec};

pub fn resolve_carry<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    current_carrier: &Player,
    defense_players: &[&Player],
    is_true_artrine: bool,
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let primary_defender = defense_players.first().copied().unwrap_or(current_carrier);
    let carrier_table = *state.attribute_table_for(&current_carrier.id());
    let defender_table = *state.attribute_table_for(&primary_defender.id());
    let carrier_fatigue = state.fatigue_lookup().get(&current_carrier.id());
    let defender_fatigue = state.fatigue_lookup().get(&primary_defender.id());

    let duel_kind = if is_true_artrine {
        DuelKind::ArtroBreakthrough
    } else {
        DuelKind::RunBreakthrough
    };

    let (att_prof, def_prof) = get_duel_profiles(duel_kind);
    let carrier_pos_domain = context
        .offense_pos_index
        .get(&current_carrier.id())
        .copied()
        .unwrap_or(Position::CenterOffense);
    let defender_pos_domain = context
        .defense_pos_index
        .get(&primary_defender.id())
        .copied()
        .unwrap_or(Position::Centerback);

    let base_att_rating = calculate_player_duel_rating_from_table(
        current_carrier,
        carrier_pos_domain,
        &carrier_table,
        att_prof,
        &carrier_fatigue,
    );
    let base_def_rating = calculate_player_duel_rating_from_table(
        primary_defender,
        defender_pos_domain,
        &defender_table,
        def_prof,
        &defender_fatigue,
    );

    let offense_power = state.power_for_team(context.offense_team_id);
    let defense_power = state.power_for_team(context.defense_team_id);

    let req = DuelResolutionRequest::with_states(
        duel_kind,
        base_att_rating,
        base_def_rating,
        current_carrier,
        primary_defender,
        carrier_fatigue,
        defender_fatigue,
        state.attribute_keys(),
        &iter_ctx.duel_context,
    )
    .with_tables(Some(&carrier_table), Some(&defender_table))
    .with_team_powers(
        Some(offense_power.offensive_power()),
        Some(defense_power.defensive_power()),
    );

    let duel_outcome = resolve_duel(req, rng);
    let attributed_duel = AttributedDuelOutcome::new(
        duel_outcome,
        smallvec![current_carrier.id()],
        smallvec![primary_defender.id()],
    );

    let macro_advance = sample_action_progression(
        ActionProgressionKind::Carry,
        duel_outcome.net_advantage(),
        1.0,
        rng,
    );

    let mut drive_row_indices = SmallVec::new();
    let mut drives_recorded = 0;
    if is_true_artrine && macro_advance > 0.0 {
        let start_x = pass_phase.scrimmage_x_mirim;
        let end_x = if context.is_home_offense {
            start_x + macro_advance
        } else {
            start_x - macro_advance
        };
        let crossed = detect_drive_crossings_arithmetic(
            state.pitch(),
            start_x,
            end_x,
            context.is_home_offense,
        );
        for row in crossed {
            drive_row_indices.push(row);
            drives_recorded += 1;
        }
    }

    let to_base = if duel_outcome.attacker_won() {
        -3.5
    } else {
        -1.5
    };
    let to_p = (logistic(to_base - 0.20 * duel_outcome.net_advantage())
        / iter_ctx.risk_profile.tolerance_index())
    .clamp(0.005, 0.45);
    let turnover = if Probability::new_clamped(to_p).sample(rng) {
        Some(context.defense_team_id)
    } else {
        None
    };
    let recovering_player_id = if turnover.is_some() {
        Some(primary_defender.id())
    } else {
        None
    };

    let mut ledger = DurationLedger::new();
    let carry_time_secs = if turnover.is_some() { 18.0 } else { 26.0 };
    ledger.record_live(
        DurationComponentKind::CarrierMovement,
        Duration::new(carry_time_secs),
    );

    let mut duels = vec![attributed_duel];
    let mut scoring_decision = ScoringDecision::NoOpportunity;

    if turnover.is_none() && duel_outcome.attacker_won() {
        scoring_decision = evaluate_and_attempt_scoring(
            state,
            context,
            iter_ctx,
            pass_phase,
            current_carrier,
            current_carrier,
            defense_players,
            drives_recorded,
            macro_advance,
            duel_outcome.net_advantage().max(10.0),
            &mut duels,
            rng,
        );
    }

    let pitch_len_m = state.pitch().length().value();
    let advance_m = macro_advance * MIRIM_TO_METERS;
    let end_x_m = if context.is_home_offense {
        (pass_phase.scrimmage_point.raw().0 + advance_m).min(pitch_len_m)
    } else {
        (pass_phase.scrimmage_point.raw().0 - advance_m).max(0.0)
    };
    let end_position = VectorPosition::from_components(
        end_x_m,
        pass_phase.scrimmage_point.raw().1,
        0.0,
    );

    let (fouls, injuries) = evaluate_contact_events(
        state,
        context,
        iter_ctx,
        current_carrier,
        primary_defender,
        carrier_fatigue,
        defender_fatigue,
        carrier_table,
        defender_table,
        duel_outcome,
        rng,
    );

    ArtrineExecutionOutcome {
        mirins_advanced: macro_advance,
        drives_recorded,
        drive_row_indices,
        turnover,
        recovering_player_id,
        scoring_decision,
        duration_ledger: ledger,
        end_position,
        duels,
        fouls,
        injuries,
        receiver_id: Some(current_carrier.id()),
        distribution_flight: None,
    }
}
