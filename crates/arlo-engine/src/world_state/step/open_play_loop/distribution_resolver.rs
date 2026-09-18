use crate::artrine::{ArtrineExecutionOutcome, DistributionFlightInfo};
use crate::match_decision::scoring::ScoringDecision;
use crate::match_decision::target_selection::{select_target, ReceptionRole};
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{
    calculate_player_duel_rating_from_table, calculate_side_rating, RatingParticipants,
};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{
    sample_action_progression, ActionProgressionKind, AttributedDuelOutcome, DuelKind,
};
use crate::team_identity::{long_launch_advance_multiplier, short_pass_advance_multiplier};
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::scoring_attempt_evaluator::evaluate_and_attempt_scoring;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player, Position};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_distribution<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    current_carrier: &Player,
    defense_players: &[&Player],
    chosen_decision: ArtrineDecisionKind,
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let receiver_id = select_target(
        &iter_ctx.target_candidates,
        state.pitch(),
        &context.offense_pos_index,
        &context.offense_instructions_index,
        Some(&context.offense_role_index),
        state.teams.player_attribute_tables(),
        context.is_home_offense,
        ReceptionRole::OpenPlayReceiver,
        &HashMap::new(),
        Some(&|id: &Uuid| state.fatigue_lookup().get(id)),
        rng,
    )
    .unwrap_or(current_carrier.id());

    let receiver = iter_ctx
        .target_candidates
        .iter()
        .copied()
        .find(|p| p.id() == receiver_id)
        .unwrap_or(current_carrier);

    let lead_defender = defense_players.first().copied().unwrap_or(current_carrier);

    let duel_kind = if chosen_decision == ArtrineDecisionKind::ShortPass {
        DuelKind::ShortDistribution
    } else {
        DuelKind::LongDistribution
    };

    let (att_prof, def_prof) = get_duel_profiles(duel_kind);
    let tables = state.teams.player_attribute_tables();
    let offense_power = state.power_for_team(context.offense_team_id);
    let defense_power = state.power_for_team(context.defense_team_id);

    let carrier_pos_domain = context
        .offense_pos_index
        .get(&current_carrier.id())
        .copied()
        .unwrap_or(Position::CenterOffense);
    let att_rating = calculate_player_duel_rating_from_table(
        current_carrier,
        carrier_pos_domain,
        state.attribute_table_for(&current_carrier.id()),
        att_prof,
        &state.fatigue_lookup().get(&current_carrier.id()),
    );
    let def_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(defense_players, &context.defense_pos_index)
            .with_fatigue(&|id| state.fatigue_lookup().get(id))
            .with_attribute_tables(tables)
            .with_team_power(defense_power.defensive_power()),
        state.attribute_keys(),
        def_prof,
    );

    let req = DuelResolutionRequest::with_states(
        duel_kind,
        att_rating,
        def_rating,
        current_carrier,
        lead_defender,
        state.fatigue_lookup().get(&current_carrier.id()),
        state.fatigue_lookup().get(&lead_defender.id()),
        state.attribute_keys(),
        &iter_ctx.duel_context,
    )
    .with_tables(
        tables.get(&current_carrier.id()),
        tables.get(&lead_defender.id()),
    )
    .with_team_powers(
        Some(offense_power.control_power()),
        Some(defense_power.defensive_power()),
    );

    let throw_duel = resolve_duel(req, rng);
    let mut duels = vec![AttributedDuelOutcome::new(
        throw_duel,
        smallvec![current_carrier.id()],
        smallvec![lead_defender.id()],
    )];

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let passing_range = offense_instructions.in_possession().passing_range();
    let (prog_kind, pass_mult) = if chosen_decision == ArtrineDecisionKind::ShortPass {
        (
            ActionProgressionKind::ShortPass,
            short_pass_advance_multiplier(passing_range),
        )
    } else {
        (
            ActionProgressionKind::LongLaunch,
            long_launch_advance_multiplier(passing_range),
        )
    };

    let advance_mirim = sample_action_progression(
        prog_kind,
        throw_duel.net_advantage(),
        pass_mult,
        rng,
    );

    let is_aerial = chosen_decision == ArtrineDecisionKind::LongLaunch;
    let rec_duel_kind = if is_aerial {
        DuelKind::AerialDuel
    } else {
        DuelKind::RouteContest
    };
    let (rec_att_prof, rec_def_prof) = get_duel_profiles(rec_duel_kind);
    let rec_att_rating = calculate_player_duel_rating_from_table(
        receiver,
        context
            .offense_pos_index
            .get(&receiver.id())
            .copied()
            .unwrap_or(Position::CenterOffense),
        state.attribute_table_for(&receiver.id()),
        rec_att_prof,
        &state.fatigue_lookup().get(&receiver.id()),
    );
    let rec_def_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(defense_players, &context.defense_pos_index)
            .with_fatigue(&|id| state.fatigue_lookup().get(id))
            .with_attribute_tables(tables)
            .with_team_power(defense_power.defensive_power()),
        state.attribute_keys(),
        rec_def_prof,
    );

    let rec_req = DuelResolutionRequest::with_states(
        rec_duel_kind,
        rec_att_rating,
        rec_def_rating,
        receiver,
        lead_defender,
        state.fatigue_lookup().get(&receiver.id()),
        state.fatigue_lookup().get(&lead_defender.id()),
        state.attribute_keys(),
        &iter_ctx.duel_context,
    )
    .with_tables(
        tables.get(&receiver.id()),
        tables.get(&lead_defender.id()),
    )
    .with_team_powers(
        Some(offense_power.offensive_power()),
        Some(defense_power.defensive_power()),
    );

    let catch_duel = resolve_duel(rec_req, rng);
    duels.push(AttributedDuelOutcome::new(
        catch_duel,
        smallvec![receiver.id()],
        smallvec![lead_defender.id()],
    ));

    let caught = throw_duel.attacker_won() && catch_duel.attacker_won();
    let turnover = if !caught && catch_duel.net_advantage() <= -2.0 {
        Some(context.defense_team_id)
    } else {
        None
    };
    let recovering_player_id = if turnover.is_some() {
        Some(lead_defender.id())
    } else {
        None
    };

    let total_secs = if caught { 26.0 } else { 16.0 };
    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::DistributionEngagement,
        Duration::new(total_secs * 0.4),
    );
    ledger.record_live(
        DurationComponentKind::DistributionFlight,
        Duration::new(total_secs * 0.6),
    );

    let actual_advance = if caught { advance_mirim } else { 0.0 };
    let pitch_len_m = state.pitch().length().value();
    let advance_m = actual_advance * MIRIM_TO_METERS;
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

    let flight_info = DistributionFlightInfo {
        receiver_id: receiver.id(),
        passer_id: current_carrier.id(),
        decision_kind: chosen_decision,
        is_aerial,
        reception_point: end_position,
        distance_mirim: advance_mirim,
        caught,
    };

    let mut scoring_decision = ScoringDecision::NoOpportunity;
    if caught && turnover.is_none() {
        scoring_decision = evaluate_and_attempt_scoring(
            state,
            context,
            iter_ctx,
            pass_phase,
            receiver,
            current_carrier,
            defense_players,
            0,
            actual_advance,
            rec_att_rating,
            &mut duels,
            rng,
        );
    }

    ArtrineExecutionOutcome {
        mirins_advanced: actual_advance,
        drives_recorded: 0,
        drive_row_indices: SmallVec::new(),
        turnover,
        recovering_player_id,
        scoring_decision,
        duration_ledger: ledger,
        end_position,
        duels,
        fouls: Vec::new(),
        injuries: Vec::new(),
        receiver_id: Some(receiver.id()),
        distribution_flight: Some(flight_info),
    }
}
